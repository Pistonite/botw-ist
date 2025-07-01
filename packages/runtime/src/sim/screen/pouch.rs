use blueflame::game::{PouchItem, gdt, singleton_instance};
use blueflame::memory::{self, Ptr, mem, proxy};
use blueflame::processor::Cpu2;
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
    pub active_entangle_slot: Option<(usize, usize)>,

    /// Weapon to spawn if changed on inventory close
    pub weapon_to_spawn: PouchScreenActor,
    /// Bow to spawn if changed on inventory close
    pub bow_to_spawn: PouchScreenActor,
    /// Shield to spawn if changed on inventory close
    pub shield_to_spawn: PouchScreenActor,
}

/// State of equipment actors in the inventory screen
#[derive(Debug, Default, Clone)]
pub struct PouchScreenActor {
    /// Actor to be spawned in the overworld
    /// when inventory is closed. None means nothing is equipped
    /// and to delete the overworld actor
    pub actor: Option<sim::OverworldActor>,
    pub changed: bool,
}

impl PouchScreen {
    /// Create a new pouch screen state and read inventory data
    ///
    /// If `force_accessible` is true, the inventory will be accessible even
    /// when mCount is 0
    pub fn open(cpu2: &mut Cpu2<'_, '_>, force_accessible: bool) -> Result<Self, memory::Error> {
        log::debug!("opening new inventory screen");
        // this is called but it doesn't do anything for us
        // linker::update_equipped_item_array(cpu2)?;

        Ok(Self {
            items: do_open(cpu2, force_accessible)?,
            active_entangle_slot: None,
            weapon_to_spawn: Default::default(),
            bow_to_spawn: Default::default(),
            shield_to_spawn: Default::default(),
        })
    }

    /// Refresh the item states in this pouch screen while still maintaining
    /// the other states
    pub fn update_all_items(
        &mut self,
        cpu2: &mut Cpu2<'_, '_>,
        force_accessible: bool,
    ) -> Result<(), memory::Error> {
        self.items = do_open(cpu2, force_accessible)?;
        Ok(())
    }
}

fn do_open(
    cpu2: &mut Cpu2<'_, '_>,
    force_accessible: bool,
) -> Result<sim::ScreenItems, memory::Error> {
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
    if m_count != 0 || force_accessible {
        for i in 0..num_tabs {
            let mut num_bows_in_curr_tab = 0;
            let mut curr_item_ptr = tab_heads[i];
            let mut slot_i = 0;
            let mut tab = vec![];

            let should_break = |curr_item_ptr: Ptr![PouchItem]| {
                sim::screen_util::should_go_to_next_tab(curr_item_ptr, i, num_tabs, &tab_heads)
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
                        tab.push(Some(sim::ScreenItem {
                            equipped,
                            ptr: curr_item_ptr,
                            name: item_name,
                            category: sim::util::item_type_to_category(item_type),
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
            }
            tabs.push(sim::ScreenTab {
                items: tab,
                category: sim::util::item_type_to_category(tab_types[i]),
            });
        }
    }

    Ok(sim::ScreenItems { tabs })
}
