use blueflame::game::{self, PouchItem};
use blueflame::memory::{self, Memory, Ptr, mem};
use skybook_parser::cir;
use teleparse::Span;

use crate::error::{ErrorReport, sim_warning};

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
                if item.equipped {
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

    /// Get the value (count) of the item by tab index and slot
    pub fn value_at(
        &self,
        tab: usize,
        slot: usize,
        memory: &Memory,
    ) -> Result<Option<i32>, memory::Error> {
        let Some(tab) = self.tabs.get(tab) else {
            return Ok(None);
        };
        let Some(Some(item)) = tab.items.get(slot) else {
            return Ok(None);
        };
        let ptr = item.ptr;
        mem! { memory: let value = *(&ptr->mValue); };

        Ok(Some(value))
    }

    /// Get the pointer of the item by tab index and slot
    pub fn ptr_at(&self, tab: usize, slot: usize) -> Option<Ptr![PouchItem]> {
        self.tabs.get(tab)?.items.get(slot)?.as_ref().map(|x| x.ptr)
    }

    /// Select an item from the screen
    ///
    /// Returns the tab index and slot index
    pub fn select(
        &self,
        item: &cir::ItemOrCategory,
        value_at_least: Option<i32>,
        memory: &Memory,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<Option<(usize, usize)>, memory::Error> {
        match item {
            cir::ItemOrCategory::Category(category) => {
                // selecting category selects the first item in a matching category
                // FIXME: we are not checking value_at_least here,
                // which should be a rarely-used edge case. If we
                // want to do this properly, the easiest way
                // is to make a cir::Item with the actor and position meta
                // then call select_item()
                Ok(self.iter_items_matching_category(*category)
                    .next().map(|(tab_i, slot, _, _)| (tab_i, slot)))
            }
            cir::ItemOrCategory::Item(item) => {
                self.select_item(item, value_at_least, memory, span, errors)
            }
        }
    }

    /// Select an item in the inventory.
    ///
    /// Returns the tab index and slot index
    pub fn select_item(
        &self,
        item: &cir::Item,
        value_at_least: Option<i32>,
        memory: &Memory,
        span: Span,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<Option<(usize, usize)>, memory::Error> {
        let meta = match &item.meta {
            None => {
                return self.select_item_by_name_meta(&item.actor, None, value_at_least, 0, memory);
            }
            Some(x) => x,
        };
        // check if the meta specifies the item's position directly
        let Some(selector) = self.process_item_position(meta.position.as_ref()) else {
            return Ok(None);
        };
        let from_slot = match selector {
            Selector::FromSlot(n) => n,
            Selector::IdxAndSlot(tab_i, slot) => {
                let Some(tab) = self.tabs.get(tab_i) else {
                    return Ok(None);
                };
                let Some(Some(item2)) = tab.items.get(slot) else {
                    return Ok(None);
                };
                if item2.name != item.actor {
                    errors.push(sim_warning!(
                        span,
                        ItemMismatch(item2.name.clone(), item.actor.clone())
                    ));
                }
                return Ok(Some((tab_i, slot)));
            }
        };

        self.select_item_by_name_meta(&item.actor, Some(meta), value_at_least, from_slot, memory)
    }

    /// Select an item in the inventory. Only non-position meta properties are considered
    ///
    /// Returns the tab index and slot index
    pub fn select_item_by_name_meta(
        &self,
        item_name: &str,
        meta: Option<&cir::ItemMeta>,
        value_at_least: Option<i32>,
        nth: usize,
        memory: &Memory,
    ) -> Result<Option<(usize, usize)>, memory::Error> {
        let mut count = nth;
        for (tab_i, slot, _, item) in self.iter_items() {
            if !item.matches(item_name, value_at_least, meta, memory)? {
                continue;
            }
            if count == 0 {
                return Ok(Some((tab_i, slot)));
            }
            count -= 1;
        }
        Ok(None)
    }

    fn process_item_position(&self, position: Option<&cir::ItemPosition>) -> Option<Selector> {
        match position {
            // match first slot
            None => Some(Selector::FromSlot(0)),
            // match x-th slot, 1 indexed
            Some(cir::ItemPosition::FromSlot(n)) => {
                Some(Selector::FromSlot((*n as usize).saturating_sub(1)))
            }, 
            Some(cir::ItemPosition::TabIdxAndSlot(tab_i, slot)) => {
                Some(Selector::IdxAndSlot(*tab_i as usize, (*slot as usize).min(19)))
            }
            Some(cir::ItemPosition::TabCategoryAndSlot(spec)) => {
                self.select_item_by_category_and_slot(spec)
                    .map(|(tab, slot)| Selector::IdxAndSlot(tab, slot))
            }
        }
    }

    /// Select an item in the inventory by category and slot position
    ///
    /// Returns the tab index and slot index
    pub fn select_item_by_category_and_slot(
        &self,
        spec: &cir::CategorySpec,
    ) -> Option<(usize, usize)> {
        let row = (spec.row as usize).saturating_sub(1);
        let col = (spec.col as usize).saturating_sub(1);
        let slot = row * 5 + col;
        let mut count = (spec.amount as usize).min(1);
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

    /// Get amount of item that match the input that can be operated on
    pub fn get_amount(&self, item: &cir::ItemOrCategory, method: ScreenItemCountingMethod, memory: &Memory) -> Result<usize, memory::Error> {
        match item {
            cir::ItemOrCategory::Category(category) => {
                let mut count = 0;
                for (_, _, _, item) in self.iter_items_matching_category(*category) {
                    count += item.get_amount(method, memory)?;
                }
                Ok(count)
            }
            cir::ItemOrCategory::Item(item) => {
                self.get_item_amount(item, method, memory)
            }
        }
    }

    /// Get amount of item that match the input that can be operated on
    pub fn get_item_amount(&self, item: &cir::Item, method: ScreenItemCountingMethod, memory: &Memory) -> Result<usize, memory::Error> {
        let meta = match &item.meta {
            None => {
                return self.get_item_amount_by_name_meta(&item.actor, None, method, 0, memory);
            }
            Some(x) => x,
        };
        // check if the meta specifies the item's position directly
        let Some(selector) = self.process_item_position(meta.position.as_ref()) else {
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

        self.get_item_amount_by_name_meta(&item.actor, Some(meta), method, from_slot, memory)

    }

    /// Get amount of item that match the input that can be operated on, without considering
    /// position meta properties
    pub fn get_item_amount_by_name_meta(
        &self,
        item_name: &str,
        meta: Option<&cir::ItemMeta>,
        method: ScreenItemCountingMethod,
        nth: usize,
        memory: &Memory,
    ) -> Result<usize, memory::Error> {
        let mut skip = nth;
        let mut count = 0;
        for (_, _, _, item) in self.iter_items() {
            if !item.matches(item_name, None, meta, memory)? {
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
            if !in_inventory {
                *item = None;
                return Ok(true);
            }

            if let Some(new_equip) = update_equipped {
                x.equipped = new_equip;
                changed = true;
            }
        }

        Ok(changed)
    }

    /// Iterate all items matching the category by (tab_index, slot_index, tab, item), skipping empty slots and
    ///
    /// Armor will match all 3 types of armor
    #[inline(always)]
    fn iter_items_matching_category(&self, category: cir::Category) -> impl Iterator<Item=(usize, usize, &ScreenTab, &ScreenItem)> {
        self.iter_items().filter(move |(_, _, _, item)| 
            item.category.is_some_and(|x| {
                if category == cir::Category::Armor {
                    x.coerce_armor() == category
                } else {
                    x == category
                }
            })
        )
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
    fn matches(&self, actor: &str, value_at_least: Option<i32>, meta: Option<&cir::ItemMeta>, memory: &Memory) -> Result<bool, memory::Error> {
        if self.name != actor {
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
                    if actual != wanted { return Ok(false); }
                }
            };
            ($memory: ident, $meta_field:ident (), $item_field:ident) => {
                if let Some(wanted) = meta.and_then(|x| x.$meta_field()) {
                    mem! { memory: let actual = *(&item_ptr->$item_field); };
                    if actual != wanted { return Ok(false); }
                }
            };
        }
        do_match!(memory, equip, mEquipped);
        do_match!(memory, life_recover, mHealthRecover);
        do_match!(memory, effect_duration, mEffectDuration);
        do_match!(memory, sell_price, mSellPrice);
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

    fn get_amount(&self, method: ScreenItemCountingMethod, memory: &Memory) -> Result<usize, memory::Error> {
        let use_value = match method {
            ScreenItemCountingMethod::Slot => false,
            ScreenItemCountingMethod::CanStack => game::can_stack(&self.name),
            ScreenItemCountingMethod::CanStackOrFood => game::can_stack(&self.name) || self.name.starts_with("Item_Cook"),
            ScreenItemCountingMethod::Value => true,
        };
        if !use_value {
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

#[derive(Debug, Clone, Copy)]
pub enum ScreenItemCountingMethod {
    /// Each slot is 1
    Slot,
    /// If the item has the CanStack tag, use its value, otherwise is 1
    CanStack,
    /// If the item has the CanStack tag, or is a food, use its value, otherwise is 1
    CanStackOrFood,
    /// Use the value for each slot
    Value,
}
enum Selector {
    FromSlot(usize),
    IdxAndSlot(usize, usize),
}
