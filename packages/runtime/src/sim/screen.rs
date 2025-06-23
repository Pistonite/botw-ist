use std::sync::Arc;

use blueflame::game::{PouchItem, gdt, singleton_instance};
use blueflame::linker;
use blueflame::linker::events::GameEvent as _;
use blueflame::memory::{Ptr, mem, proxy};
use blueflame::processor::{self, Cpu2};

use crate::error::{ErrorReport, sim_warning};
use crate::sim;
use crate::iv;

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
#[derive(Clone)]
pub struct InventoryScreen {
    /// Tabbed item data
    pub tabs: Vec<[Option<Box<InventoryScreenItem>>; 20]>,

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

/// Simulation of temporary inventory item state
#[derive(Clone)]
pub struct InventoryScreenItem {
    /// PouchItem pointer for access actual item data in game
    pub ptr: Ptr![PouchItem],

    /// If the item is equipped on screen.
    ///
    /// You can have item equipped on screen but not actually
    /// in inventory using Prompt Entanglement
    pub equipped: bool,
}

/// State of equipment actors in the inventory screen
#[derive(Default, Clone)]
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
            _ => None
        }
    }
    /// Create a new screen state and read inventory data
    fn open_new_inventory(cpu2: &mut Cpu2<'_, '_>) -> Result<Self, processor::Error> {
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

        mem! { m:
            let num_tabs = *(&pmdm->mNumTabs);
            let tab_heads = *(&pmdm->mTabs);
        };
        let num_tabs = num_tabs.min(0) as usize;
        let mut tabs = Vec::with_capacity(num_tabs);

        // if mCount is 0, the inventory shows up as empty
        mem ! { m:
            let m_count = *(&pmdm->mList1.mCount);
        };
        if m_count != 0 {
            for i in 0..num_tabs {
                let mut num_bows_in_curr_tab = 0;
                let mut curr_item_ptr = tab_heads[i];
                let mut should_break = curr_item_ptr.is_nullptr()
                || if i == num_tabs - 1 {
                    true
                } else {
                    let next_head = tab_heads[i + 1];
                    curr_item_ptr == next_head
                };
                let mut slot_i = 0;
                let mut tab = [const { None }; 20];
                while !should_break {
                    mem! { m:
                        let equipped = *(&curr_item_ptr->mEquipped);
                        let item_type = *(&curr_item_ptr->mType);
                    };
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
                    if tab_slot < 20 {
                        tab[tab_slot] = Some(Box::new(InventoryScreenItem {
                            equipped,
                            ptr: curr_item_ptr,
                        }));
                    }

                    // advance to next item
                    mem! { m:
                        let next_node_ptr = *(&curr_item_ptr->mListNode.mNext)
                    };
                    curr_item_ptr = if next_node_ptr.is_nullptr() {
                        0u64.into()
                    } else {
                            (next_node_ptr.to_raw() - 8).into()
                        };
                    should_break = curr_item_ptr.is_nullptr()
                        || if i == num_tabs - 1 {
                            true
                        } else {
                            let next_head = tab_heads[i + 1];
                            curr_item_ptr == next_head
                        };
                }
                tabs.push(tab);
            }
        }


        Ok(Self::Inventory(InventoryScreen{
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
                log::warn!(
                    "transition_to_overworld called but screen is already overworld"
                );
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
                    State { actors: vec![], menu_overload },
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
            Self::ShopDialog | Self::StatueDialog => {
            }
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
    /// The list is sorted so you can binary search
    pub fn get_equipped_item_ptrs(&self) -> Vec<u64> {
        let mut out = vec![];
        for tab in &self.tabs {
            for item in tab {
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
}
