use blueflame::game::{PouchCategory, PouchItem, PouchItemType, gdt, singleton_instance};
use blueflame::memory::{self, Ptr, mem, proxy};
use blueflame::processor::{Cpu2, Process};
use blueflame::{linker, processor};
use derive_more::{Deref, DerefMut};

use crate::sim;

/// Simulation of the inventory screen state
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct PouchScreen {
    /// Tabbed item data
    #[deref]
    #[deref_mut]
    items: sim::ScreenItems,

    /// The current active slot for Prompt Entanglement. i.e.
    /// this item's prompt will be used when performing
    /// an action on an entangle-reachable slot
    active_entangle_slot: Option<(usize, usize)>,

    /// Weapon state to change on inventory close
    pub weapon_state: PouchScreenEquipState,
    /// Bow state to change on inventory close
    pub bow_state: PouchScreenEquipState,
    /// Shield state to change on inventory close
    pub shield_state: PouchScreenEquipState,
}

/// State of equipment actors in the inventory screen
#[derive(Debug, Default, Clone)]
pub struct PouchScreenEquipState {
    /// Item to be spawned in the overworld on the player
    /// when inventory is closed. Nullptr means don't create anything
    pub item: Ptr![PouchItem],
    /// If true, delete the item on the player instead of creating
    /// or do nothing.
    pub to_delete: bool,
}

impl PouchScreen {
    /// Create a new pouch screen state and read inventory data
    ///
    /// If `force_accessible` is true, the inventory will be accessible even
    /// when mCount is 0
    pub fn open(cpu2: &mut Cpu2<'_, '_>, force_accessible: bool) -> Result<Self, processor::Error> {
        // this is needed for "drop" to know if the dropped weapon
        // should be dropped from player, or created by CreatePlayerTrashActorMgr.
        linker::update_equipped_item_array(cpu2)?;

        Ok(Self::open_no_exec(cpu2.proc, force_accessible)?)
    }

    /// Create a new pouch screen state without execution
    pub fn open_no_exec(proc: &Process, force_accessible: bool) -> Result<Self, memory::Error> {
        Ok(Self {
            items: do_open(proc, force_accessible)?,
            active_entangle_slot: None,
            weapon_state: Default::default(),
            bow_state: Default::default(),
            shield_state: Default::default(),
        })
    }

    /// Activate Prompt Entanglement for the tab and slot, and any slots 3 tabs apart
    pub fn activate_pe(&mut self, tab: usize, slot: usize) {
        self.active_entangle_slot = Some((tab % 3, slot));
    }

    /// Deactivate Prompt Entanglement
    pub fn deactivate_pe(&mut self) {
        self.active_entangle_slot = None;
    }

    pub fn active_pe_slot(&self) -> Option<(usize, usize)> {
        self.active_entangle_slot
    }

    /// Get list of item pointers that are currently activated
    ///
    /// The returned list is sorted so you can binary search
    pub fn pe_activated_items(&self) -> Vec<u64> {
        match self.active_entangle_slot {
            None => vec![],
            Some((tab, slot)) => self.items.pe_reachable_item_ptrs(tab, slot),
        }
    }

    /// Check if the item in (tab, slot) is an activated PE slot, meaning
    /// the prompt of this item can be used on the PE target item
    pub fn is_pe_activated_slot(&self, tab: usize, slot: usize, is_target: bool) -> bool {
        let Some((active_tab, active_slot)) = self.active_entangle_slot else {
            // PE is not activated
            return false;
        };

        if active_tab != tab % 3 || slot != active_slot {
            if tab != 0 {
                return false;
            }
            // see comment in ScreenItems::pe_reachable_item_ptrs
            let is_default_to_zero_case =
                active_tab % 3 == 2 || active_tab % 3 == self.tabs.len() % 3;
            if !is_default_to_zero_case {
                return false;
            }
        }

        if !is_target && self.items.is_translucent_or_empty(tab, slot) {
            // cannot use prompt from a translucent item, since it's usually
            // not interactable, unless it's also the first slot
            return slot == 0;
        }

        true
    }

    /// Get the actual slot to use when performing action on the target slot
    ///
    /// If the target slot is empty, it will return 0 (i.e. targets the first
    /// item in the tab). If the target slot is translucent, it will also
    /// return 0 unless `allow_translucent` is true, in which case it will
    /// just use that slot
    pub fn get_pe_target_slot(&self, tab: usize, slot: usize, allow_translucent: bool) -> usize {
        match self.items.get(tab, slot) {
            sim::ScreenItemState::Empty => 0,
            sim::ScreenItemState::Translucent(_) => {
                if allow_translucent {
                    slot
                } else {
                    0
                }
            }
            _ => slot,
        }
    }

    /// Get the equipment state tracked by pouch screen based on the item type
    pub fn equipment_state_mut(&mut self, pouch_item_type: i32) -> &mut PouchScreenEquipState {
        match pouch_item_type {
            1 => &mut self.bow_state,
            3 => &mut self.shield_state,
            _ => &mut self.weapon_state,
        }
    }

    /// Refresh the item states in this pouch screen while still maintaining
    /// the other states
    pub fn update_all_items(
        &mut self,
        cpu2: &mut Cpu2<'_, '_>,
        force_accessible: bool,
    ) -> Result<(), memory::Error> {
        self.items = do_open(cpu2.proc, force_accessible)?;
        Ok(())
    }
}

impl PouchScreenEquipState {
    /// Set the state to be unequip (after an unequip action)
    pub fn set_unequip(&mut self) {
        self.item = 0u64.into();
        self.to_delete = true;
    }
    /// Set the state to be equipped (after an equip action)
    pub fn set_equip(&mut self, item: Ptr![PouchItem]) {
        self.item = item;
        self.to_delete = false;
    }
}

fn do_open(proc: &Process, force_accessible: bool) -> Result<sim::ScreenItems, memory::Error> {
    let m = proc.memory();
    let gdt_ptr = gdt::trigger_param_ptr(m)?;
    let info = {
        proxy! { let gdt = *gdt_ptr as trigger_param in proc };
        sim::view::extract_gdt_info_from_trigger_param(gdt).unwrap_or_default()
    };
    let weapon_slots = info.num_weapon_slots as usize;
    let bow_slots = info.num_bow_slots as usize;
    let shield_slots = info.num_shield_slots as usize;

    let pmdm = singleton_instance!(pmdm(m))?;
    let head_node_ptr = Ptr!(&pmdm->mList1.mStartEnd);

    mem! { m:
        let num_tabs = *(&pmdm->mNumTabs);
        let tab_heads = *(&pmdm->mTabs);
        let tab_types = *(&pmdm->mTabsType);
    };

    // FIXME: need to figure out how to display tabs when tabs are overflown
    let num_tabs = num_tabs.clamp(0, 50) as usize;
    let mut tabs = Vec::with_capacity(num_tabs);

    // if mCount is 0, the inventory shows up as empty
    mem! { m:
        let m_count = *(&pmdm->mList1.mCount);
    };
    if m_count != 0 || force_accessible {
        // FIXME: right now, we don't fully know how arrow slots
        // are displayed (especially when bow num exceeds bow slot num,
        // or when arrow slots are not displayed in the bow tab)
        //
        // it's not worth to fix those cases right now, but definitely
        // need to fix in the future
        for i in 0..num_tabs {
            let mut num_bows_in_curr_tab = 0;
            let mut curr_item_ptr = tab_heads[i];
            let mut slot_i = 0;
            let mut tab = vec![];

            let tab_ty = tab_types[i];
            // if the tab is undiscovered, then all items in it are not accessible
            let tab_accessible = force_accessible
                || match PouchItemType::from_value(tab_ty).map(PouchItemType::to_category) {
                    Some(category) => match category {
                        PouchCategory::Sword => info.sword_tab_discovered,
                        PouchCategory::Bow => info.bow_tab_discovered,
                        PouchCategory::Shield => info.shield_tab_discovered,
                        PouchCategory::Armor => info.armor_tab_discovered,
                        PouchCategory::Material => info.material_tab_discovered,
                        PouchCategory::Food => info.food_tab_discovered,
                        PouchCategory::KeyItem => info.key_item_tab_discovered,
                        PouchCategory::Invalid => false,
                    },
                    None => false,
                };

            let should_break = |curr_item_ptr: Ptr![PouchItem]| {
                sim::util::should_go_to_next_tab(curr_item_ptr, i, num_tabs, &tab_heads)
            };

            while !should_break(curr_item_ptr) {
                mem! { m:
                    let equipped = *(&curr_item_ptr->mEquipped);
                    let in_inventory = *(&curr_item_ptr->mInInventory);
                    let item_type = *(&curr_item_ptr->mType);
                };

                // If item is not in inventory (i.e. translucent)
                // it is displayed as empty
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

                let mut accessible = tab_accessible;
                if accessible && !force_accessible {
                    // FIXME: need more testing with how this works for bows
                    // see FIXME above
                    const SWORD: i32 = PouchItemType::Sword as i32;
                    const SHIELD: i32 = PouchItemType::Shield as i32;
                    let max = match tab_ty {
                        SWORD => weapon_slots,
                        SHIELD => shield_slots,
                        _ => 20,
                    };
                    if tab_slot >= max {
                        accessible = false;
                    }
                }

                // it could be more than 20 if you have a LOT of arrow slots
                // (because empty bow slots shift them)
                if tab_slot < 20 || force_accessible {
                    while tab.len() < tab_slot {
                        tab.push(None);
                    }
                    if accessible {
                        tab.push(Some(sim::ScreenItem {
                            ptr: curr_item_ptr,
                            equipped,
                            in_inventory,
                            name: item_name,
                            category: sim::util::item_type_to_category(item_type),
                        }));
                    } else {
                        tab.push(None);
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
            }
            tabs.push(sim::ScreenTab {
                items: tab,
                category: sim::util::item_type_to_category(tab_ty),
            });
        }
    }

    Ok(sim::ScreenItems { tabs })
}
