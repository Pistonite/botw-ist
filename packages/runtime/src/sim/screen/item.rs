use blueflame::game::PouchItem;
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
                let category = *category;
                // selecting category selects the first item in a matching category
                for (tab_i, tab) in self.tabs.iter().enumerate() {
                    for (slot, item) in tab.items.iter().enumerate() {
                        let Some(item) = item else {
                            // there could be empty slots in the tab
                            continue;
                        };
                        let Some(item_category) = item.category else {
                            break;
                        };
                        let matched = if category == cir::Category::Armor {
                            category == item_category.coerce_armor()
                        } else {
                            category == item_category
                        };
                        if !matched {
                            continue;
                        }
                        // FIXME: we are not checking value_at_least here,
                        // which should be a rarely-used edge case. If we
                        // want to do this properly, the easiest way
                        // is to make a cir::Item with the actor and position meta
                        // then call select_item()
                        return Ok(Some((tab_i, slot)));
                    }
                }
                Ok(None)
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
        enum Selector {
            FromSlot(usize),
            IdxAndSlot(usize, usize),
        }
        // check if the meta specifies the item's position directly
        let selector = match &meta.position {
            None => Selector::FromSlot(1), // match first slot
            Some(cir::ItemPosition::FromSlot(n)) => Selector::FromSlot(*n as usize), // match x-th slot, 1 indexed
            Some(cir::ItemPosition::TabIdxAndSlot(tab_i, slot)) => {
                Selector::IdxAndSlot(*tab_i as usize, (*slot as usize).min(19))
            }
            Some(cir::ItemPosition::TabCategoryAndSlot(spec)) => {
                let Some((tab, slot)) = self.select_item_by_category_and_slot(spec) else {
                    return Ok(None);
                };
                Selector::IdxAndSlot(tab, slot)
            }
        };
        let from_slot = match selector {
            Selector::FromSlot(n) => n.saturating_sub(1),
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
        for (tab_i, tab) in self.tabs.iter().enumerate() {
            for (slot, item) in tab.items.iter().enumerate() {
                let Some(item) = item else {
                    continue;
                };
                // match name
                if item.name != item_name {
                    continue;
                }
                let item_ptr = item.ptr;
                if let Some(wanted_value) = meta.and_then(|x| x.value) {
                    mem! { memory: let actual_value = *(&item_ptr->mValue); };
                    if actual_value != wanted_value {
                        continue;
                    }
                } else if let Some(value_at_least) = value_at_least {
                    mem! { memory: let actual_value = *(&item_ptr->mValue); };
                    if actual_value < value_at_least {
                        continue;
                    }
                }
                macro_rules! do_match {
                    ($memory: ident, $meta_field:ident, $item_field:ident) => {
                        if let Some(wanted) = meta.and_then(|x| x.$meta_field) {
                            mem! { memory: let actual = *(&item_ptr->$item_field); };
                            if actual != wanted {
                                continue;
                            }
                        }
                    };
                    ($memory: ident, $meta_field:ident (), $item_field:ident) => {
                        if let Some(wanted) = meta.and_then(|x| x.$meta_field()) {
                            mem! { memory: let actual = *(&item_ptr->$item_field); };
                            if actual != wanted {
                                continue;
                            }
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
                        continue;
                    }
                }

                // matched
                if count == 0 {
                    return Ok(Some((tab_i, slot)));
                }
                count -= 1;
            }
        }

        Ok(None)
    }

    /// Select an item in the inventory by category and slot position
    ///
    /// Returns the tab index and slot index
    pub fn select_item_by_category_and_slot(
        &self,
        spec: &cir::CategorySpec,
    ) -> Option<(usize, usize)> {
        let slot = ((spec.row * 5) as usize + spec.col as usize).min(19);
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
}
