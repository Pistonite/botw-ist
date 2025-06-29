use std::collections::VecDeque;

use blueflame::game::WeaponModifierInfo;
use teleparse::Span;

use crate::error::{ErrorReport, sim_error};
use crate::iv;

#[derive(Debug, Default, Clone)]
pub struct OverworldSystem {
    pub weapon: Option<OverworldActor>,
    pub bow: Option<OverworldActor>,
    pub shield: Option<OverworldActor>,

    ground_weapons: Vec<OverworldActor>,
    ground_materials: VecDeque<OverworldActor>,
    ground_materials_despawning: Vec<OverworldActor>,
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
            errors.push(sim_error!(span, CannotDoWhileHolding));
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
}

impl OverworldActor {
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
