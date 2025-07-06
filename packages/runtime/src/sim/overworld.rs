use std::collections::VecDeque;

use blueflame::game::{PouchItem, WeaponModifierInfo};
use blueflame::memory::{self, Memory, Ptr, mem};
use skybook_parser::cir;
use teleparse::Span;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::{iv, sim};

#[derive(Debug, Default, Clone)]
pub struct OverworldSystem {
    pub weapon: Option<OverworldActor>,
    pub bow: Option<OverworldActor>,
    pub shield: Option<OverworldActor>,

    /// Ground weapons that are scheduled to spawn
    spawning_ground_weapons: Vec<OverworldActor>,
    /// Weapons already on the ground
    ground_weapons: Vec<OverworldActor>,
    /// Materials already on the ground
    ground_materials: VecDeque<OverworldActor>,
    /// Materials on the ground that are despawning
    ground_materials_despawning: Vec<OverworldActor>,
    /// Items held by player in the overworld
    holding: Vec<OverworldActor>,
    /// If currently in the "hold attached" state
    /// used for arrowless offset
    is_hold_attached: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverworldPreDropResult {
    /// No extra clean up needed
    Ok,
    /// Is holding normally, so pre-action-auto-drop is not possible
    Holding,
    /// Is holding attached, so items in the overworld are dropped now,
    /// and items in the inventory must be dropped after the action
    AutoDrop,
}

/// Simulates an actor in the overworld
#[derive(Debug, Default, Clone)]
pub struct OverworldActor {
    /// Name of the actor
    pub name: String,
    /// Value for weapons (1 for materials)
    pub value: i32,
    /// Weapon modifier if any, None if not weapon
    pub modifier: Option<WeaponModifierInfo>,
}

impl OverworldSystem {
    pub fn to_iv(&self) -> iv::Overworld {
        let mut items = vec![];

        if let Some(weapon) = &self.weapon {
            items.push(weapon.to_equipped_iv());
        }
        if let Some(bow) = &self.bow {
            items.push(bow.to_equipped_iv());
        }
        if let Some(shield) = &self.shield {
            items.push(shield.to_equipped_iv());
        }
        for item in &self.holding {
            items.push(item.to_held_iv());
        }
        for item in &self.ground_weapons {
            items.push(item.to_ground_weapon_iv());
        }
        for item in &self.ground_materials {
            items.push(item.to_ground_item_iv(false));
        }
        for item in &self.ground_materials_despawning {
            items.push(item.to_ground_item_iv(true));
        }

        iv::Overworld { items }
    }

    /// Spawn additional items held by the player (does not replacing existing)
    pub fn spawn_held_items(&mut self, items: Vec<String>) {
        for item in items {
            self.holding.push(OverworldActor {
                name: item,
                value: 1,
                modifier: None,
            });
        }
    }

    /// Add an actor to the queue to spawn when inventory is closed
    pub fn spawn_weapon_later(&mut self, actor: OverworldActor) {
        log::debug!("adding ground equipments to spawn: {}", actor.name);
        self.spawning_ground_weapons.push(actor)
    }

    /// Spawn items that are previous dropped
    pub fn spawn_ground_weapons(&mut self) {
        log::debug!(
            "spawning ground equipments: {:?}",
            self.spawning_ground_weapons
        );
        self.ground_weapons
            .extend(std::mem::take(&mut self.spawning_ground_weapons))
    }

    /// Clear the weapons that are about to spawn (as if spawning failed)
    pub fn clear_spawning_weapons(&mut self) {
        self.spawning_ground_weapons.clear()
    }

    /// Set the overworld holding state, only possible if there are held items
    pub fn set_held_attached(&mut self, attached: bool) {
        self.is_hold_attached = attached && !self.holding.is_empty();
    }

    /// Check if the player is either currently not holding or the items can be auto-dropped (i.e.
    /// is in attached state). If so, auto-drop the item before performing an action
    #[must_use = "result needs to be checked for extra clean up"]
    pub fn predrop_for_action(
        &mut self,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> OverworldPreDropResult {
        if self.holding.is_empty() {
            return OverworldPreDropResult::Ok;
        }
        if !self.is_hold_attached {
            errors.push(sim_error!(span, CannotDoWhileHoldingInOverworld));
            return OverworldPreDropResult::Holding;
        }
        // perform auto-drop
        OverworldPreDropResult::AutoDrop
    }

    /// Delete the actors currently being held
    pub fn delete_held_items(&mut self) {
        self.is_hold_attached = false;
        self.holding.clear();
    }

    /// Drop items held by the player to the ground
    pub fn drop_held_items(&mut self) {
        self.is_hold_attached = false;
        self.ground_materials
            .extend(std::mem::take(&mut self.holding));
        while self.ground_materials.len() > 10 {
            // unwrap: length is > 10
            let item = self.ground_materials.pop_front().unwrap();
            self.ground_materials_despawning.push(item);
        }
    }

    pub fn is_holding(&self) -> bool {
        !self.holding.is_empty()
    }

    /// Despawn items that are over the limit
    pub fn despawn_items(&mut self) {
        self.ground_materials_despawning.clear();
    }

    /// Change the player equipment if the item is not null. Do nothing if null
    pub fn change_player_equipment(
        &mut self,
        item: Ptr![PouchItem],
        memory: &Memory,
    ) -> Result<(), memory::Error> {
        if item.is_nullptr() {
            return Ok(());
        }
        mem! { memory:
            let item_type = *(&item->mType);
            let value = *(&item->mValue);
            let modifier_flags = *(&item->mSellPrice);
            let modifier_value = *(&item->mHealthRecover);
        }

        // get equipment slot based on pouch item type
        // see uking::ui::getCreateEquipmentSlot
        let slot = match item_type {
            0 => &mut self.weapon,
            1 => &mut self.bow,
            3 => &mut self.shield,
            _ => return Ok(()),
        };

        let modifier = if modifier_flags == 0 {
            None
        } else {
            Some(WeaponModifierInfo {
                flags: modifier_flags as u32,
                value: modifier_value,
            })
        };

        let name = Ptr!(&item->mName).cstr(memory)?.load_utf8_lossy(memory)?;
        *slot = Some(OverworldActor {
            name,
            value,
            modifier,
        });

        Ok(())
    }

    /// Drop the currently equipped weapon on the player
    pub fn drop_player_equipment(&mut self, item_type: i32) {
        let actor = match item_type {
            0 => self.weapon.take(),
            1 => self.bow.take(),
            3 => self.shield.take(),
            _ => return,
        };
        if let Some(actor) = actor {
            self.spawn_weapon_later(actor);
        }
    }

    /// Select an item from the ground
    pub fn ground_select(
        &self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Option<GroundItemHandle<&Self>> {
        let handle = self.do_ground_select(item, meta, span, errors)?;
        Some(handle.bind(self))
    }

    /// Select an item from the ground, with the ability to remove it
    pub fn ground_select_mut(
        &mut self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Option<GroundItemHandle<&mut Self>> {
        let handle = self.do_ground_select(item, meta, span, errors)?;
        Some(handle.bind(self))
    }

    /// Select an item from the ground
    fn do_ground_select(
        &self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Option<GroundItemHandle<()>> {
        let meta = match &meta {
            None => {
                return self.do_ground_select_without_position_nth(item, None, 0, span, errors);
            }
            Some(x) => x,
        };
        let from_slot = match &meta.position {
            None => 0, // match first slot
            Some(cir::ItemPosition::FromSlot(n)) => (*n as usize).saturating_sub(1), // match x-th slot, 1 indexed
            _ => {
                // cannot specify by tab for items on the ground
                errors.push(sim_error!(span, PositionSpecNotAllowed));
                return None;
            }
        };
        self.do_ground_select_without_position_nth(item, Some(meta), from_slot, span, errors)
    }

    fn do_ground_select_without_position_nth(
        &self,
        name: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        nth: usize,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Option<GroundItemHandle<()>> {
        if let Some(meta) = meta {
            if meta.equip.is_some()
                || meta.effect_duration.is_some()
                || meta.effect_id.is_some()
                || meta.effect_level.is_some()
                || !meta.ingredients.is_empty()
            {
                errors.push(sim_warning!(span, UselessItemMatchProp));
            }
        }
        let mut count = nth;
        for (handle, item) in self.iter_ground_items() {
            if !item.matches(name, meta) {
                continue;
            }
            // matched
            if count == 0 {
                return Some(handle);
            }
            count -= 1;
        }
        None
    }

    /// Get number of items on the ground that matches the selector
    pub fn get_ground_amount(
        &self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
    ) -> usize {
        let meta = match &meta {
            None => {
                return self.get_ground_amount_without_position_nth(item, None, 0);
            }
            Some(x) => x,
        };
        let from_slot = match &meta.position {
            Some(cir::ItemPosition::FromSlot(n)) => (*n as usize).saturating_sub(1), // match x-th slot, 1 indexed
            _ => 0,
        };
        self.get_ground_amount_without_position_nth(item, Some(meta), from_slot)
    }

    /// Get number of items on the ground that matches the selector,
    /// without considering position meta properties
    pub fn get_ground_amount_without_position_nth(
        &self,
        name: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        nth: usize,
    ) -> usize {
        let mut skip = nth;
        let mut count = 0;
        for (_, item) in self.iter_ground_items() {
            if !item.matches(name, meta) {
                continue;
            }
            if skip > 0 {
                skip -= 1;
                continue;
            }
            count += 1;
        }
        count
    }

    #[inline(always)]
    fn iter_ground_items(&self) -> impl Iterator<Item = (GroundItemHandle<()>, &OverworldActor)> {
        self.ground_materials_despawning
            .iter()
            .enumerate()
            .map(|(i, item)| (GroundItemHandle::MaterialDespawning((), i), item))
            .chain(
                self.ground_materials
                    .iter()
                    .enumerate()
                    .map(|(i, item)| (GroundItemHandle::Material((), i), item)),
            )
            .chain(
                self.ground_weapons
                    .iter()
                    .enumerate()
                    .map(|(i, item)| (GroundItemHandle::Weapon((), i), item)),
            )
    }

    /// Select an item from equipped items
    ///
    /// meta is ignored, and will emit a warning if is not None
    pub fn equipped_select(
        &self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Option<EquippedItemHandle<&Self>> {
        Some(
            self.do_equipped_select(item, meta, span, errors)?
                .bind(self),
        )
    }

    /// Select an item from equipped items
    ///
    /// meta is ignored, and will emit a warning if is not None
    pub fn equipped_select_mut(
        &mut self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Option<EquippedItemHandle<&mut Self>> {
        Some(
            self.do_equipped_select(item, meta, span, errors)?
                .bind(self),
        )
    }

    /// Select an item from equipped items
    fn do_equipped_select(
        &self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Option<EquippedItemHandle<()>> {
        if meta.is_some() {
            errors.push(sim_warning!(span, UselessMetaForOverworldEquipment));
        }
        match item {
            cir::ItemNameSpec::Actor(actor) => {
                if self.weapon.as_ref().is_some_and(|x| &x.name == actor) {
                    return Some(EquippedItemHandle::Weapon(()));
                }
                if self.bow.as_ref().is_some_and(|x| &x.name == actor) {
                    return Some(EquippedItemHandle::Bow(()));
                }
                if self.shield.as_ref().is_some_and(|x| &x.name == actor) {
                    return Some(EquippedItemHandle::Shield(()));
                }
            }
            cir::ItemNameSpec::Category(category) => match category {
                cir::Category::Weapon => {
                    if self.weapon.is_some() {
                        return Some(EquippedItemHandle::Weapon(()));
                    }
                }
                cir::Category::Bow => {
                    if self.bow.is_some() {
                        return Some(EquippedItemHandle::Weapon(()));
                    }
                }
                cir::Category::Shield => {
                    if self.shield.is_some() {
                        return Some(EquippedItemHandle::Weapon(()));
                    }
                }
                _ => {}
            },
        }
        errors.push(sim_error!(span, NotEquippedInOverworld));
        None
    }
}

impl OverworldActor {
    /// Returns if the overworld actor matches the item selector
    pub fn matches(&self, name: &cir::ItemNameSpec, meta: Option<&cir::ItemMeta>) -> bool {
        if !sim::util::name_spec_matches(name, &self.name) {
            return false;
        }
        // matching value for overworld actors is mostly
        // used for weapons, since materials can only have value = 1
        if let Some(wanted_value) = meta.and_then(|x| x.value)
            && wanted_value != self.value
        {
            return false;
        }
        if let Some(wanted_mod_value) = meta.and_then(|x| x.life_recover)
            && self.modifier.is_none_or(|m| m.value != wanted_mod_value)
        {
            return false;
        }

        if let Some(wanted_flags) = meta.and_then(|x| x.sell_price)
            && self.modifier.is_none_or(|m| {
                !sim::util::modifier_meta_matches(name, wanted_flags, m.flags as i32)
            })
        {
            return false;
        }

        true
    }

    pub fn to_equipped_iv(&self) -> iv::OverworldItem {
        iv::OverworldItem::Equipped {
            actor: self.name.clone(),
            value: self.value,
            modifier: self
                .modifier
                .map(|x| iv::WeaponModifier {
                    flag: x.flags as i32,
                    value: x.value,
                })
                .unwrap_or_default(),
        }
    }
    pub fn to_ground_weapon_iv(&self) -> iv::OverworldItem {
        iv::OverworldItem::GroundEquipment {
            actor: self.name.clone(),
            value: self.value,
            modifier: self
                .modifier
                .map(|x| iv::WeaponModifier {
                    flag: x.flags as i32,
                    value: x.value,
                })
                .unwrap_or_default(),
        }
    }
    pub fn to_held_iv(&self) -> iv::OverworldItem {
        iv::OverworldItem::Held {
            actor: self.name.clone(),
        }
    }
    pub fn to_ground_item_iv(&self, is_despawning: bool) -> iv::OverworldItem {
        iv::OverworldItem::GroundItem {
            actor: self.name.clone(),
            despawning: is_despawning,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GroundItemHandle<TSys> {
    Weapon(TSys, usize),
    Material(TSys, usize),
    MaterialDespawning(TSys, usize),
}

impl GroundItemHandle<()> {
    pub fn bind<TSys>(self, sys: TSys) -> GroundItemHandle<TSys> {
        match self {
            Self::Weapon(_, i) => GroundItemHandle::Weapon(sys, i),
            Self::Material(_, i) => GroundItemHandle::Material(sys, i),
            Self::MaterialDespawning(_, i) => GroundItemHandle::MaterialDespawning(sys, i),
        }
    }
}

impl GroundItemHandle<&mut OverworldSystem> {
    /// Get reference to the actor
    pub fn actor(&self) -> &OverworldActor {
        match self {
            Self::Weapon(o, i) => &o.ground_weapons[*i],
            Self::Material(o, i) => &o.ground_materials[*i],
            Self::MaterialDespawning(o, i) => &o.ground_materials_despawning[*i],
        }
    }

    /// Remove the item from the ground
    pub fn remove(self) -> OverworldActor {
        match self {
            Self::Weapon(o, i) => o.ground_weapons.remove(i),
            Self::Material(o, i) => o.ground_materials.remove(i).unwrap(),
            Self::MaterialDespawning(o, i) => o.ground_materials_despawning.remove(i),
        }
    }
}

impl GroundItemHandle<&OverworldSystem> {
    /// Get reference to the actor
    pub fn actor(&self) -> &OverworldActor {
        match self {
            Self::Weapon(o, i) => &o.ground_weapons[*i],
            Self::Material(o, i) => &o.ground_materials[*i],
            Self::MaterialDespawning(o, i) => &o.ground_materials_despawning[*i],
        }
    }
}

/// Handle representing an item equipped in the overworld
#[derive(Debug, Clone, Copy)]
pub enum EquippedItemHandle<TSys> {
    Weapon(TSys),
    Bow(TSys),
    Shield(TSys),
}

impl EquippedItemHandle<()> {
    pub fn bind<TSys>(self, sys: TSys) -> EquippedItemHandle<TSys> {
        match self {
            Self::Weapon(_) => EquippedItemHandle::Weapon(sys),
            Self::Bow(_) => EquippedItemHandle::Bow(sys),
            Self::Shield(_) => EquippedItemHandle::Shield(sys),
        }
    }
}

impl EquippedItemHandle<&mut OverworldSystem> {
    /// Get reference to the actor
    pub fn actor(&self) -> &OverworldActor {
        match self {
            Self::Weapon(o) => o.weapon.as_ref().unwrap(),
            Self::Bow(o) => o.bow.as_ref().unwrap(),
            Self::Shield(o) => o.shield.as_ref().unwrap(),
        }
    }

    /// Remove the item
    pub fn remove(self) -> OverworldActor {
        match self {
            Self::Weapon(o) => std::mem::take(&mut o.weapon).unwrap(),
            Self::Bow(o) => std::mem::take(&mut o.bow).unwrap(),
            Self::Shield(o) => std::mem::take(&mut o.shield).unwrap(),
        }
    }
}
