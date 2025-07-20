use blueflame::game::PouchItem;
use blueflame::memory::{self, Memory, Ptr, mem};
use skybook_parser::cir;
use teleparse::Span;

use crate::error::{ErrorReport, sim_warning};
use crate::sim;

/// Items in inventory screen (for pouch and shop)
#[derive(Debug, Clone)]
pub struct ScreenItems {
    /// Tabbed item data
    pub tabs: Vec<ScreenTab>,
}

#[derive(Debug, Clone)]
pub struct ScreenTab {
    /// Items in the tab
    ///
    /// The items are `Option`s since the bow tab can have empty
    /// slots between bow and arrows
    pub items: Vec<Option<ScreenItem>>,
    pub category: Option<cir::Category>,
}

/// Simulation of temporary inventory item state
#[derive(Debug, Clone)]
pub struct ScreenItem {
    /// PouchItem pointer for access actual item data in game
    pub ptr: Ptr![PouchItem],

    /// If the item is in inventory (not translucent)
    pub in_inventory: bool,

    /// If the item is equipped on screen.
    ///
    /// You can have item equipped on screen but not actually
    /// in inventory using Prompt Entanglement
    pub equipped: bool,

    /// Cached item name, used for selection
    pub name: String,

    /// Cached item category, used for selection
    pub category: Option<cir::Category>,
}

impl ScreenItems {
    /// Get list of items that are displayed as equipped
    ///
    /// The returned list is sorted so you can binary search
    pub fn equipped_item_ptrs(&self) -> Vec<u64> {
        let mut out = vec![];
        for tab in &self.tabs {
            for item in &tab.items {
                let Some(item) = item else {
                    continue;
                };
                if item.in_inventory && item.equipped {
                    out.push(item.ptr.to_raw());
                }
            }
        }
        out.sort();
        out
    }

    /// Get list of items that are displayed in the inventory pause menu
    ///
    /// The returned list is sorted so you can binary search
    pub fn accessible_item_ptrs(&self) -> Vec<u64> {
        let mut out = vec![];
        for tab in &self.tabs {
            for item in tab.items.iter().flatten() {
                out.push(item.ptr.to_raw());
            }
        }
        out.sort();
        out
    }

    /// Get list of items that *may* be reachable with the currently PE activation.
    ///
    /// The returned list is sorted so you can binary search
    pub fn pe_reachable_item_ptrs(&self, active_tab: usize, active_slot: usize) -> Vec<u64> {
        let mut out = vec![];
        for (tab_i, tab) in self.tabs.iter().enumerate() {
            if tab_i % 3 != active_tab % 3 {
                continue;
            }
            let Some(Some(item)) = tab.items.get(active_slot) else {
                continue;
            };
            out.push(item.ptr.to_raw());
        }
        out.sort();
        out
    }

    /// Get the "corrected" item position to use in PMDM function calls
    ///
    /// This is because the slot index passed into PMDM function is the real
    /// position (index), but in our implementation is the visual implementation.
    /// These two are the same except for the arrow slots, where the number of empty
    /// bow slots need to be removed
    pub fn corrected_slot(&self, tab: usize, slot: usize) -> i32 {
        let Some(tab) = self.tabs.get(tab) else {
            return slot as i32;
        };
        let mut real_slot = slot as i32;
        for item in tab.items.iter().take(slot) {
            if item.is_none() {
                real_slot -= 1;
            }
        }

        real_slot
    }

    /// Get the (visual) slot index of the first item in the tab (might be translucent)
    pub fn first_item(&self, tab: usize) -> Option<usize> {
        let tab = self.tabs.get(tab)?;
        let (i, _) = tab.items.iter().enumerate().find(|(_, x)| x.is_some())?;
        Some(i)
    }

    /// Get the pointer of the item by tab index and slot
    pub fn get(&self, tab: usize, slot: usize) -> ScreenItemState<Ptr![PouchItem]> {
        let Some(tab) = self.tabs.get(tab) else {
            return ScreenItemState::Empty;
        };
        let Some(Some(item)) = tab.items.get(slot) else {
            return ScreenItemState::Empty;
        };
        if item.in_inventory {
            ScreenItemState::Normal(item.ptr)
        } else {
            ScreenItemState::Translucent(item.ptr)
        }
    }

    /// Check if the position is empty (no item is there) or translucent (not in inventory item)
    pub fn is_translucent_or_empty(&self, tab: usize, slot: usize) -> bool {
        self.is_translucent(tab, slot).unwrap_or_default()
    }

    /// Check if the position is translucent.
    ///
    /// Returns `None` is the position is empty.
    pub fn is_translucent(&self, tab: usize, slot: usize) -> Option<bool> {
        let tab = self.tabs.get(tab)?;
        let item = tab.items.get(slot)?.as_ref()?;
        Some(!item.in_inventory)
    }

    /// Select an item from the screen
    ///
    /// If position meta property is specified, the name spec is not used
    /// for matching, but will give a warning if it doesn't match.
    /// The position may also target an empty or translucent slot, which can
    /// be queried by the `get` function after selection.
    /// Without position meta, it will not target translucent or empty slots
    ///
    /// Returns the tab index and slot index
    pub fn select(
        &self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        value_at_least: Option<i32>,
        memory: &Memory,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<Option<(usize, usize)>, memory::Error> {
        let Some(meta) = meta else {
            return self.select_without_position_nth(item, None, value_at_least, 0, memory);
        };
        let Some(selector) = self.process_position_meta(meta.position.as_ref()) else {
            return Ok(None);
        };
        let from_slot = match selector {
            Selector::FromSlot(n) => n,
            Selector::IdxAndSlot(tab_i, slot) => {
                // warn if the item specified by the position does not match
                // the name in the command
                let Some(tab) = self.tabs.get(tab_i) else {
                    return Ok(Some((tab_i, slot)));
                };
                let Some(Some(item_in_inv)) = tab.items.get(slot) else {
                    return Ok(Some((tab_i, slot)));
                };
                if !sim::util::name_spec_matches(item, &item_in_inv.name) {
                    match item {
                        cir::ItemNameSpec::Actor(name) => {
                            errors.push(sim_warning!(
                                span,
                                ItemMismatch(item_in_inv.name.clone(), name.clone())
                            ));
                        }
                        cir::ItemNameSpec::Category(category) => {
                            errors.push(sim_warning!(
                                span,
                                ItemMismatchCategory(item_in_inv.name.clone(), *category)
                            ));
                        }
                    }
                }
                return Ok(Some((tab_i, slot)));
            }
        };
        self.select_without_position_nth(item, Some(meta), value_at_least, from_slot, memory)
    }

    /// Select an item from the screen, without considering position meta
    ///
    /// Returns the tab index and slot index
    pub fn select_without_position_nth(
        &self,
        name: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        value_at_least: Option<i32>,
        nth: usize,
        memory: &Memory,
    ) -> Result<Option<(usize, usize)>, memory::Error> {
        let mut count = nth;
        for (tab_i, slot, _, item) in self.iter_items() {
            if !item.matches(name, value_at_least, meta, memory)? {
                continue;
            }
            // by default, does not target translucent items.
            // still targetable by using position directly
            if !item.in_inventory {
                continue;
            }
            if count == 0 {
                return Ok(Some((tab_i, slot)));
            }
            count -= 1;
        }
        Ok(None)
    }

    /// Get amount of item that match the input name spec and meta
    ///
    /// If position meta property is specified, the name spec is not used
    /// for matching.
    pub fn get_amount(
        &self,
        item: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        method: sim::CountingMethod,
        memory: &Memory,
    ) -> Result<usize, memory::Error> {
        let Some(meta) = meta else {
            return self.get_amount_without_position_nth(item, None, method, 0, memory);
        };
        let Some(selector) = self.process_position_meta(meta.position.as_ref()) else {
            return Ok(0);
        };
        let from_slot = match selector {
            Selector::FromSlot(n) => n,
            Selector::IdxAndSlot(tab_i, slot) => {
                let Some(tab) = self.tabs.get(tab_i) else {
                    return Ok(0);
                };
                let Some(Some(item)) = tab.items.get(slot) else {
                    return Ok(0);
                };
                // we don't check if the actor matches here, since we just need to
                // grab the amount
                return item.get_amount(method, memory);
            }
        };
        self.get_amount_without_position_nth(item, Some(meta), method, from_slot, memory)
    }

    /// Get amount of item that match the input that can be operated on,
    /// without considering position meta properties.
    ///
    /// The first `nth` matched slots will be skipped, regardless of the counting method.
    ///
    /// If PE target is set,
    pub fn get_amount_without_position_nth(
        &self,
        name: &cir::ItemNameSpec,
        meta: Option<&cir::ItemMeta>,
        method: sim::CountingMethod,
        nth: usize,
        memory: &Memory,
    ) -> Result<usize, memory::Error> {
        let mut skip = nth;
        let mut count = 0;
        for (_, _, _, item) in self.iter_items() {
            if !item.matches(name, None, meta, memory)? {
                continue;
            }
            if skip != 0 {
                skip -= 1;
                continue;
            }
            count += item.get_amount(method, memory)?;
        }
        Ok(count)
    }

    /// Update one inventory slot
    ///
    /// Return `Ok(true)` if the slot was changed
    pub fn update(
        &mut self,
        tab: usize,
        slot: usize,
        update_equipped: Option<bool>,
        memory: &Memory,
    ) -> Result<bool, memory::Error> {
        let Some(tab) = self.tabs.get_mut(tab) else {
            return Ok(false);
        };
        let Some(item) = tab.items.get_mut(slot) else {
            return Ok(false);
        };
        let mut changed = false;
        if let Some(x) = item.as_mut() {
            let ptr = x.ptr;
            mem! { memory: let in_inventory = *(&ptr->mInInventory); };
            if x.in_inventory != in_inventory {
                x.in_inventory = in_inventory;
                changed = true;
            }

            if let Some(new_equip) = update_equipped {
                x.equipped = new_equip;
                changed = true;
            }
        }

        Ok(changed)
    }

    fn process_position_meta(&self, position: Option<&cir::ItemPosition>) -> Option<Selector> {
        match position {
            // match first slot
            None => Some(Selector::FromSlot(0)),
            // match x-th slot, 1 indexed
            Some(cir::ItemPosition::FromSlot(n)) => {
                Some(Selector::FromSlot((*n as usize).saturating_sub(1)))
            }
            Some(cir::ItemPosition::TabIdxAndSlot(tab_i, slot)) => Some(Selector::IdxAndSlot(
                *tab_i as usize,
                (*slot as usize).min(19),
            )),
            Some(cir::ItemPosition::TabCategoryAndSlot(spec)) => self
                .select_by_category_and_slot(spec)
                .map(|(tab, slot)| Selector::IdxAndSlot(tab, slot)),
        }
    }

    /// Select an item in the inventory by category and slot position
    ///
    /// Returns the tab index and slot index. Returns None if the spec is out of bounds.
    /// However, does not check if the slot actually has item or not
    fn select_by_category_and_slot(&self, spec: &cir::CategorySpec) -> Option<(usize, usize)> {
        log::debug!("selecting {spec:?}");
        let row = (spec.row as usize).saturating_sub(1);
        let col = (spec.col as usize).saturating_sub(1);
        let slot = row * 5 + col;
        let mut count = (spec.amount as usize).max(1);
        for (tab_i, tab) in self.tabs.iter().enumerate() {
            let Some(category) = tab.category else {
                continue;
            };
            if category.coerce_armor() == spec.category.coerce_armor() {
                count -= 1;
                if count == 0 {
                    return Some((tab_i, slot));
                }
            }
        }
        None
    }

    /// Iterate all items by (tab_index, slot_index, tab, item), skipping empty slots and
    /// translucent items
    #[inline(always)]
    fn iter_items(&self) -> impl Iterator<Item = (usize, usize, &ScreenTab, &ScreenItem)> {
        self.iter_slots().filter_map(|(tab_i, slot, tab, item)| {
            item.as_ref().map(|item| (tab_i, slot, tab, item))
        })
    }

    /// Iterate all slots by (tab_index, slot_index, tab, item)
    #[inline(always)]
    fn iter_slots(&self) -> impl Iterator<Item = (usize, usize, &ScreenTab, &Option<ScreenItem>)> {
        self.tabs.iter().enumerate().flat_map(|(tab_i, tab)| {
            tab.items
                .iter()
                .enumerate()
                .map(move |(slot, item)| (tab_i, slot, tab, item))
        })
    }
}

impl ScreenItem {
    fn matches(
        &self,
        item: &cir::ItemNameSpec,
        value_at_least: Option<i32>,
        meta: Option<&cir::ItemMeta>,
        memory: &Memory,
    ) -> Result<bool, memory::Error> {
        if !sim::util::name_spec_matches(item, &self.name) {
            return Ok(false);
        }
        let item_ptr = self.ptr;
        if let Some(wanted_value) = meta.and_then(|x| x.value) {
            mem! { memory: let actual_value = *(&item_ptr->mValue); };
            if actual_value != wanted_value {
                return Ok(false);
            }
        } else if let Some(value_at_least) = value_at_least {
            mem! { memory: let actual_value = *(&item_ptr->mValue); };
            if actual_value < value_at_least {
                return Ok(false);
            }
        }

        macro_rules! do_match {
            ($memory: ident, $meta_field:ident, $item_field:ident) => {
                if let Some(wanted) = meta.and_then(|x| x.$meta_field) {
                    mem! { memory: let actual = *(&item_ptr->$item_field); };
                    if actual != wanted {
                        return Ok(false);
                    }
                }
            };
            ($memory: ident, $meta_field:ident (), $item_field:ident) => {
                if let Some(wanted) = meta.and_then(|x| x.$meta_field()) {
                    mem! { memory: let actual = *(&item_ptr->$item_field); };
                    if actual != wanted {
                        return Ok(false);
                    }
                }
            };
        }
        do_match!(memory, equip, mEquipped);
        do_match!(memory, life_recover, mHealthRecover);
        do_match!(memory, effect_duration, mEffectDuration);
        if let Some(wanted) = meta.and_then(|x| x.sell_price) {
            mem! { memory: let actual = *(&item_ptr->mSellPrice); };
            if !sim::util::modifier_meta_matches(item, wanted, actual) {
                return Ok(false);
            }
        }
        do_match!(memory, effect_id_f32(), mEffectId);
        do_match!(memory, effect_level, mEffectLevel);
        if let Some(wanted) = meta.map(|x| &x.ingredients)
            && !wanted.is_empty()
        {
            mem! {memory:
                let ingredients_ptr = *(&item_ptr->mIngredients.mPtrs);
            };
            let mut actual_ingrs = vec![];
            for i in 0..5 {
                mem! {memory:
                    let actual_ingr = *(ingredients_ptr.ith(i as u64));
                };
                let actual_ingr = actual_ingr.cstr(memory)?.load_utf8_lossy(memory)?;
                if actual_ingr.is_empty() {
                    break;
                }
                actual_ingrs.push(actual_ingr);
            }
            if wanted != &actual_ingrs {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn get_amount(
        &self,
        method: sim::CountingMethod,
        memory: &Memory,
    ) -> Result<usize, memory::Error> {
        if !method.should_use_value(&self.name) {
            return Ok(1);
        }
        let ptr = self.ptr;
        mem! {memory: let value = *(&ptr->mValue); }
        if value <= 0 {
            return Ok(0);
        }

        Ok(value as usize)
    }
}

enum Selector {
    FromSlot(usize),
    IdxAndSlot(usize, usize),
}

pub enum ScreenItemState<T> {
    /// Slot is Empty (no item is there)
    Empty,
    /// Slot has a translucent item (in_inventory = false)
    Translucent(T),
    /// Slot has a normal item (in_inventory = true)
    Normal(T),
}

impl<T> ScreenItemState<T> {
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            ScreenItemState::Empty => None,
            ScreenItemState::Translucent(x) => Some(x),
            ScreenItemState::Normal(x) => Some(x),
        }
    }
}
