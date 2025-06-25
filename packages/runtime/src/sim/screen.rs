use std::sync::Arc;

use blueflame::game::{PouchItem, gdt, singleton_instance};
use blueflame::{linker, memory};
use blueflame::linker::events::GameEvent as _;
use blueflame::memory::{mem, proxy, Memory, Ptr};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;
use teleparse::Span;

use crate::error::{ErrorReport, sim_warning};
use crate::iv;
use crate::sim;

/// Simulation of different screens in the game and transitioning
/// between them
#[derive(Default, Clone)]
pub struct ScreenSystem {
    screen: Arc<Screen>,

    /// If Menu Overload Glitch is active
    menu_overload: bool,

    /// Flag for controlling whether removal of held items
    /// should happen after the dialog when transitioning
    /// from Overworld to a dialog.
    ///
    /// Normally, the game forces you to put away items before you
    /// can talk, but if you setup the smuggle glitch thingy,
    /// you can delay this until the dialog is finished to generate
    /// offsets (i.e broken slots)
    remove_held_item_after_dialog: bool,
}

/// Type of the screen and the data they hold
#[derive(Default, Clone)]
pub enum Screen {
    /// In the overworld, no additional screens
    #[default]
    Overworld,
    /// In the inventory screen
    Inventory(InventoryScreen),
    /// In shop dialog (selling sellable items)
    ShopDialog,
    /// In statue dialog (selling Orbs)
    StatueDialog,
}

/// Simulation of the inventory screen state
#[derive(Debug, Clone)]
pub struct InventoryScreen {
    /// Tabbed item data
    pub tabs: Vec<InventoryScreenTab>,

    /// The current active slot for Prompt Entanglement. i.e.
    /// this item's prompt will be used when performing
    /// an action on an entangle-reachable slot
    pub active_entangle_slot: Option<(usize, usize)>,

    /// Weapon to spawn if changed on inventory close
    pub weapon_to_spawn: InventoryScreenActor,
    /// Bow to spawn if changed on inventory close
    pub bow_to_spawn: InventoryScreenActor,
    /// Shield to spawn if changed on inventory close
    pub shield_to_spawn: InventoryScreenActor,
}

#[derive(Debug, Clone)]
pub struct InventoryScreenTab {
    /// Items in the tab
    ///
    /// The items are `Option`s since the bow tab can have empty
    /// slots between bow and arrows
    pub items: Vec<Option<InventoryScreenItem>>,
    pub category: Option<cir::Category>,
}

/// Simulation of temporary inventory item state
#[derive(Debug, Clone)]
pub struct InventoryScreenItem {
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

/// State of equipment actors in the inventory screen
#[derive(Debug, Default, Clone)]
pub struct InventoryScreenActor {
    /// Actor to be spawned in the overworld
    /// when inventory is closed. None means nothing is equipped
    /// and to delete the overworld actor
    actor: Option<sim::OverworldActor>,
    changed: bool,
}

impl ScreenSystem {
    pub fn transition_to_inventory(
        &mut self,
        ctx: &mut sim::Context<&mut Cpu2>,
        overworld: &mut sim::OverworldSystem,
        warn_if_already: bool,
        errors: &mut Vec<ErrorReport>,
    ) -> Result<(), processor::Error> {
        if matches!(self.screen.as_ref(), Screen::Inventory(_)) {
            if warn_if_already {
                errors.push(sim_warning!(&ctx.span, UselessScreenTransition));
            }
            return Ok(());
        }
        let screen = Arc::make_mut(&mut self.screen);
        // if the screen cannot be transition directly to inventory
        // screen, close it first to go back to overworld
        match screen {
            Screen::ShopDialog | Screen::StatueDialog => {
                let drop_items = self.remove_held_item_after_dialog;
                self.remove_held_item_after_dialog = false;
                screen.transition_to_overworld(ctx, overworld, self.menu_overload, drop_items)?;
            }
            // unreachable: checked above
            Screen::Inventory(_) => unreachable!(),
            Screen::Overworld => {}
        }

        // actually open the inventory
        *screen = Screen::open_new_inventory(ctx.cpu())?;

        Ok(())
    }

    pub fn current_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn current_screen_mut(&mut self) -> &mut Screen {
        Arc::make_mut(&mut self.screen)
    }
}

impl Screen {
    /// Get the type for inventory view binding
    pub fn iv_type(&self) -> iv::Screen {
        match self {
            Screen::Overworld => iv::Screen::Overworld,
            Screen::Inventory(_) => iv::Screen::Inventory,
            Screen::ShopDialog => iv::Screen::Shop,
            Screen::StatueDialog => iv::Screen::Statue,
        }
    }

    pub fn as_inventory(&self) -> Option<&InventoryScreen> {
        match self {
            Screen::Inventory(inv) => Some(inv),
            _ => None,
        }
    }

    pub fn as_inventory_mut(&mut self) -> Option<&mut InventoryScreen> {
        match self {
            Screen::Inventory(inv) => Some(inv),
            _ => None,
        }
    }
    /// Create a new screen state and read inventory data
    fn open_new_inventory(cpu2: &mut Cpu2<'_, '_>) -> Result<Self, processor::Error> {
        log::debug!("opening new inventory screen");
        // this is called but it doesn't do anything for us
        // linker::update_equipped_item_array(cpu2)?;

        let m = cpu2.proc.memory();
        let gdt = gdt::trigger_param_ptr(m)?;
        let bow_slots = {
            let proc = &cpu2.proc;
            proxy! { let trigger_param = *gdt as trigger_param in proc };
            trigger_param
                .by_name::<gdt::fd!(s32)>("BowPorchStockNum")
                .map(|x| x.get())
                .copied()
                .unwrap_or(0) as usize
        };

        let pmdm = singleton_instance!(pmdm(m))?;
        let head_node_ptr = Ptr!(&pmdm->mList1.mStartEnd);

        mem! { m:
            let num_tabs = *(&pmdm->mNumTabs);
            let tab_heads = *(&pmdm->mTabs);
            let tab_types = *(&pmdm->mTabsType);
        };
        let num_tabs = num_tabs.max(0) as usize;
        let mut tabs = Vec::with_capacity(num_tabs);

        // if mCount is 0, the inventory shows up as empty
        mem! { m:
            let m_count = *(&pmdm->mList1.mCount);
        };
        if m_count != 0 {
            for i in 0..num_tabs {
                let mut num_bows_in_curr_tab = 0;
                let mut curr_item_ptr = tab_heads[i];
                let mut slot_i = 0;
                let mut tab = vec![];

                let should_break = |curr_item_ptr: Ptr![PouchItem]| {
                    if curr_item_ptr.is_nullptr() {
                        return true;
                    }
                    if i < num_tabs - 1 {
                        let next_head = tab_heads[i + 1];
                        return curr_item_ptr == next_head;
                    }
                    false
                };

                while !should_break(curr_item_ptr) {
                    mem! { m:
                        let equipped = *(&curr_item_ptr->mEquipped);
                        let in_inventory = *(&curr_item_ptr->mInInventory);
                        let item_type = *(&curr_item_ptr->mType);
                    };

                    // If item is not in inventory (i.e. translucent)
                    // it is displayed as empty
                    if in_inventory {
                        let item_name = Ptr!(&curr_item_ptr->mName).cstr(m)?.load_utf8_lossy(m)?;
                        // adjust the slot for arrows using the type
                        if item_type == 1 {
                            num_bows_in_curr_tab += 1;
                        }
                        let tab_slot = if item_type == 2 {
                            slot_i + bow_slots - num_bows_in_curr_tab
                        } else {
                            slot_i
                        };
                        slot_i += 1;
                        // it could be more than 20 if you have a LOT of arrow slots
                        // (because empty bow slots shift them)
                        if tab_slot < 20 {
                            while tab.len() < tab_slot {
                                tab.push(None);
                            }
                            tab.push(Some(InventoryScreenItem {
                                equipped,
                                ptr: curr_item_ptr,
                                name: item_name,
                                category: sim::view::item_type_to_category(item_type)
                            }));
                        }
                    }

                    // advance to next item
                    mem! { m:
                        let next_node_ptr = *(&curr_item_ptr->mListNode.mNext)
                    };
                    curr_item_ptr = if next_node_ptr.is_nullptr() || next_node_ptr == head_node_ptr {
                        0u64.into()
                    } else {
                        (next_node_ptr.to_raw() - 8).into()
                    };
                    // should_break = curr_item_ptr.is_nullptr()
                    //     || if i == num_tabs - 1 {
                    //         true
                    //     } else {
                    //         let next_head = tab_heads[i + 1];
                    //         curr_item_ptr == next_head
                    //     };
                }
                tabs.push(InventoryScreenTab { items: tab, category: sim::view::item_type_to_category(tab_types[i]) });
            }
        }

        Ok(Self::Inventory(InventoryScreen {
            tabs,
            active_entangle_slot: None,
            weapon_to_spawn: InventoryScreenActor::default(),
            bow_to_spawn: InventoryScreenActor::default(),
            shield_to_spawn: InventoryScreenActor::default(),
        }))
    }

    fn transition_to_overworld(
        &mut self,
        ctx: &mut sim::Context<&mut Cpu2>,
        overworld: &mut sim::OverworldSystem,
        menu_overload: bool,
        drop_items: bool,
    ) -> Result<(), processor::Error> {
        match self {
            Self::Overworld => {
                log::warn!("transition_to_overworld called but screen is already overworld");
                return Ok(());
            }
            Self::Inventory(inv_screen) => {
                if !menu_overload {
                    log::debug!("updating overworld equiments");
                    if inv_screen.weapon_to_spawn.changed {
                        overworld.weapon = inv_screen.weapon_to_spawn.actor.take();
                    }
                    if inv_screen.bow_to_spawn.changed {
                        overworld.bow = inv_screen.bow_to_spawn.actor.take();
                    }
                    if inv_screen.shield_to_spawn.changed {
                        overworld.shield = inv_screen.bow_to_spawn.actor.take();
                    }
                } else {
                    log::debug!("not updating overworld equipments because of menu overload");
                }
                #[derive(Default)]
                struct State {
                    actors: Vec<String>,
                    menu_overload: bool,
                }
                let state = linker::events::CreateHoldingItem::execute_subscribed(
                    ctx.cpu(),
                    State {
                        actors: vec![],
                        menu_overload,
                    },
                    |state, name| {
                        if !state.menu_overload {
                            state.actors.push(name);
                        }
                    },
                    linker::create_holding_items,
                )?;
                log::debug!("spawning overworld holding items: {:?}", state.actors);
                overworld.spawn_held_items(state.actors);
            }
            Self::ShopDialog | Self::StatueDialog => {}
        }
        log::debug!("removing translucent items on returning to overworld");
        linker::delete_removed_items(ctx.cpu())?;

        if drop_items {
            log::debug!("dropping held items on returning to overworld");
            linker::remove_held_items(ctx.cpu())?;
            overworld.drop_held_items();
        }

        *self = Self::Overworld;
        Ok(())
    }
}

impl InventoryScreen {
    /// Get list of items that are displayed as equipped
    ///
    /// The returned list is sorted so you can binary search
    pub fn get_equipped_item_ptrs(&self) -> Vec<u64> {
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
    pub fn get_accessible_item_ptrs(&self) -> Vec<u64> {
        let mut out = vec![];
        for tab in &self.tabs {
            for item in &tab.items {
                if let Some(item) = item {
                    out.push(item.ptr.to_raw());
                };
            }
        }
        out.sort();
        out
    }

    /// Select an item in the inventory.
    ///
    /// Returns the tab index and slot index
    pub fn select(&self, 
        item: &cir::ItemOrCategory, 
        value_at_least: Option<i32>,
        memory: &Memory, 
        span: Span, 
        errors: &mut Vec<ErrorReport>) -> Result<Option<(usize, usize)>, memory::Error> {
        match item {
            cir::ItemOrCategory::Category(category) => {
                let category = *category;
                // selecting category selects the first item in a matching category
                for (tab_i, tab) in self.tabs.iter().enumerate() {
                    for (slot, item) in tab.items.iter().enumerate() {
                        let Some(item) = item else {
                            break;
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
                return Ok(None);
            }
            cir::ItemOrCategory::Item(item) => {
                self.select_item(item, value_at_least, memory, span, errors)
            }
        }
    }

    /// Select an item in the inventory.
    ///
    /// Returns the tab index and slot index
    pub fn select_item(&self, item: &cir::Item, 
        value_at_least: Option<i32>,
        memory: &Memory, span: Span, errors: &mut Vec<ErrorReport>) -> Result<Option<(usize, usize)>, memory::Error> {
        let meta = match &item.meta {
            None => return self.select_item_by_name_meta(&item.actor, None, value_at_least, 0, memory),
            Some(x) => x
        };
        enum Selector {
            FromSlot(usize),
            IdxAndSlot(usize, usize)
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
                    errors.push(sim_warning!(&span, ItemMismatch(item2.name.clone(), item.actor.clone())));
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
                            if actual != wanted { continue; }
                        }
                    };
                    ($memory: ident, $meta_field:ident (), $item_field:ident) => {
                        if let Some(wanted) = meta.and_then(|x| x.$meta_field()) {
                            mem! { memory: let actual = *(&item_ptr->$item_field); };
                            if actual != wanted { continue; }
                        }
                    }
                }
                do_match!(memory, equip, mEquipped);
                do_match!(memory, life_recover, mHealthRecover);
                do_match!(memory, effect_duration, mEffectDuration);
                do_match!(memory, sell_price, mSellPrice);
                do_match!(memory, effect_id_f32(), mEffectId);
                do_match!(memory, effect_level_f32(), mEffectLevel);
                if let Some(wanted) = meta.map(|x| &x.ingredients) {
                    if !wanted.is_empty() {
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
                }

                // matched
                if count == 0 {
                    return Ok(Some((tab_i, slot)));
                }
                count-=1;
            }
        }

        Ok(None)
    }

    /// Select an item in the inventory by category and slot position
    ///
    /// Returns the tab index and slot index
    pub fn select_item_by_category_and_slot(&self, spec: &cir::CategorySpec) -> Option<(usize, usize)> {
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
    pub fn update(&mut self, 
        tab: usize, 
        slot: usize, 
        update_equipped: Option<bool>,
        memory: &Memory) -> Result<bool, memory::Error> {
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

    pub fn get_value(&self, tab: usize, slot: usize, memory: &Memory) -> Result<Option<i32>, memory::Error> {
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
}
