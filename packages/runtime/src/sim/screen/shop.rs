use blueflame::game::{singleton_instance, PouchItem};
use blueflame::linker;
use blueflame::memory::{mem, Ptr};
use blueflame::processor::{self, Cpu2};

use crate::sim;

/// Simulation of the screen where you sell stuff
#[derive(Debug, Clone)]
pub enum ShopScreen {
    Buy,
    Sell(sim::ScreenItems),
}

impl ShopScreen {
    pub fn open_sell(cpu2: &mut Cpu2<'_, '_>) -> Result<Self, processor::Error> {
        log::debug!("opening new shop selling screen");

        // TODO: this might not be accurate since no RE is done for shop screen
        // the current implementation is iterating through the tabs,
        // and get the armor/material/food tabs.
        // however, if there are multiple sets of armor/material/food,
        // I am not sure if they are presented like this

        let m = cpu2.proc.memory();
        let pmdm = singleton_instance!(pmdm(m))?;
        let head_node_ptr = Ptr!(&pmdm->mList1.mStartEnd);
        mem! { m:
            let num_tabs = *(&pmdm->mNumTabs);
            let tab_heads = *(&pmdm->mTabs);
            let tab_types = *(&pmdm->mTabsType);
        };
        let num_tabs = num_tabs.max(0) as usize;
        let mut tabs = Vec::with_capacity(num_tabs);

        // note that shop screen can be visible even when mCount is 0

        for i in 0..num_tabs {
            if !matches!(tab_types[i], 4 | 5 | 6 | 7 | 8) {
                continue;
            }
            let mut curr_item_ptr = tab_heads[i];
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
                // it is disabled, and not sellable, so we can
                // just not include it in the screen
                if in_inventory {
                    let item_name = Ptr!(&curr_item_ptr->mName).cstr(m)?.load_utf8_lossy(m)?;
                    tab.push(Some(sim::ScreenItem {
                        equipped,
                        ptr: curr_item_ptr,
                        name: item_name,
                        category: sim::view::item_type_to_category(item_type),
                    }));
                }

                // advance to next item
                mem! { m:
                    let next_node_ptr = *(&curr_item_ptr->mListNode.mNext)
                };
                curr_item_ptr = if next_node_ptr.is_nullptr() || next_node_ptr == head_node_ptr
                    {
                        0u64.into()
                    } else {
                        (next_node_ptr.to_raw() - 8).into()
                    };
            }
            tabs.push(sim::ScreenTab {
                items: tab,
                category: sim::view::item_type_to_category(tab_types[i]),
            });
        }

        Ok(Self::Sell(sim::ScreenItems { tabs }))
    }

    /// Transition to buying screen
    pub fn transition_to_buy(&mut self, cpu: &mut Cpu2<'_, '_>) -> Result<(), processor::Error> {
        if let Self::Sell(_) = self {
            log::debug!("removing translucent items on stop selling");
            linker::delete_removed_items(cpu)?;
            *self = Self::Buy;
        }
        Ok(())
    }


}
