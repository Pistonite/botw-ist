use std::{collections::{HashMap, VecDeque}, future::Future, sync::{atomic::{AtomicBool, AtomicU64}, Arc}};

use skybook_parser::{search::QuotedItemResolver, ParseOutput};
use blueflame::{error::Error, memory::{Memory, Proxies}};

mod scheduler;
use scheduler::Scheduler;

/// Inventory View
pub mod iv;
pub mod pointer;

pub struct RunOutput {
    /// State at each simulation step
    pub states: Vec<Arc<State>>
}

impl RunOutput {
    // TODO: error
    pub fn get_pouch_list(&self, step: usize) -> iv::PouchList {
        // mock data
        let mock_item1 = iv::PouchItem {
                common: iv::CommonItem{
                    actor_name: "Weapon_Sword_070".to_string(),
                    value: 4000,
                    is_equipped: true,
                },
                item_type: 0,
                item_use: 0,
                is_in_inventory: true,
                is_no_icon: false,
                data: iv::ItemData {
                    effect_value: 0,
                    effect_duration: 0,
                    sell_price: 0,
                    effect_id: 0f32,
                    effect_level: 0f32,
                },
                ingredients: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
                holding_count: 0,
                prompt_entangled: false,
                node_addr: 0.into(),
                node_valid: true,
                node_pos: 419,
                node_prev: 0.into(),
                node_next: 0.into(),
                allocated_idx: 0,
                unallocated_idx: -1,
                tab_idx: 0,tab_slot: 0,
                accessible: true,
                dpad_accessible: true,
            };

        let mock_item2 = 
            iv::PouchItem {
                common: iv::CommonItem{
                    actor_name: "NormalArrow".to_string(),
                    value: 25,
                    is_equipped: false,
                },
                item_type: 2,
                item_use: 8,
                is_in_inventory: true,
                is_no_icon: false,
                data: iv::ItemData {
                    effect_value: 0,
                    effect_duration: 0,
                    sell_price: 0,
                    effect_id: 0f32,
                    effect_level: 0f32,
                },
                ingredients: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
                holding_count: 0,
                prompt_entangled: false,
                node_addr: 0.into(),
                node_valid: true,
                node_pos: 418,
                node_prev: 0.into(),
                node_next: 0.into(),
                allocated_idx: 1,
                unallocated_idx: -1,
                tab_idx: 1,tab_slot: 0,
                accessible: true,
                dpad_accessible: true,
            };

        let mock_item3 = 
            iv::PouchItem {
                common: iv::CommonItem{
                    actor_name: "Item_Fruit_A".to_string(),
                    value: 999,
                    is_equipped: false,
                },
                item_type: 7,
                item_use: 8,
                is_in_inventory: true,
                is_no_icon: false,
                data: iv::ItemData {
                    effect_value: 0,
                    effect_duration: 0,
                    sell_price: 0,
                    effect_id: 0f32,
                    effect_level: 0f32,
                },
                ingredients: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
                holding_count: 0,
                prompt_entangled: false,
                node_addr: 0.into(),
                node_valid: true,
                node_pos: 418,
                node_prev: 0.into(),
                node_next: 0.into(),
                allocated_idx: 2,
                unallocated_idx: -1,
                tab_idx: 3,tab_slot: 0,
                accessible: true,
                dpad_accessible: true,
            };

        let mut items = vec![ ];
            items.push(mock_item1);
        if step >= 1 {
            items.push(mock_item2);
        }
        if step >= 2 {
            items.push(mock_item3);
        }

        let tabs = match step {
            0 => vec![
                iv::PouchTab {
                    item_idx: 0,
                    tab_type: 0 // sword
                },
            ],
            1 => vec![
                iv::PouchTab {
                    item_idx: 0,
                    tab_type: 0 // sword
                },
                iv::PouchTab {
                    item_idx: 1,
                    tab_type: 1, //bow
                },
                iv::PouchTab {
                    item_idx: -1,
                    tab_type: 3, //shield
                },
            ],
            _ => vec![
                iv::PouchTab {
                    item_idx: 0,
                    tab_type: 0 // sword
                },
                iv::PouchTab {
                    item_idx: 1,
                    tab_type: 1, //bow
                },
                iv::PouchTab {
                    item_idx: -1,
                    tab_type: 3, //shield
                },
                iv::PouchTab {
                    item_idx: 2,
                    tab_type: 7, //material
                },
            ]
        };

        iv::PouchList {
            count: items.len() as i32 - 1,
            items,
            are_tabs_valid: true,
            num_tabs: tabs.len() as i32,
            tabs
        }


        // let Some(state) = self.get_state_by_step(step) else {
        //     return Ok(vec![]);
        // };
    }

    /// Get the GDT inventory view for the given step in the script
    ///
    /// Trailing items with empty names are not included
    pub fn get_gdt_inventory(&self, step: usize) -> iv::Gdt {
        // mock data
        //
        return iv::Gdt {
            items: vec![
                iv::GdtItem {
                    common: iv::CommonItem{
                        actor_name: "Weapon_Sword_070".to_string(),
                        value: 4000,
                        is_equipped: true,
                    },
                    idx: 0,
                    data: iv::GdtItemData::Sword {
                        idx: 0,
                        info: iv::WeaponModifier {
                            flag: 0,value: 0
                        }
                    }
                },
                iv::GdtItem {
                    common: iv::CommonItem{
                        actor_name: "Item_Fruit_A".to_string(),
                        value: 999,
                        is_equipped: false,
                    },
                    idx: 1,
                    data: iv::GdtItemData::None
                }
            ],
            master_sword: iv::GdtMasterSword {
                is_true_form: true,
                add_power: 30,
                add_beam_power: 10,
                recover_time: 0f32
            },
            info: iv::GdtInvInfo {
                num_weapon_slots: 8,
                num_bow_slots: 5,
                num_shield_slots: 4,

                sword_tab_discovered: true,
                bow_tab_discovered: true,
                shield_tab_discovered: true,
                armor_tab_discovered: false,
                material_tab_discovered: true,
                food_tab_discovered: false,
                key_item_tab_discovered: false,
            }
        }

    }

    pub fn get_overworld_items(&self, step: usize) -> iv::Overworld {
        // mock data
        iv::Overworld {
            items: vec![
                iv::OverworldItem::Equipped{
                    actor:"Weapon_Sword_070".to_string(), 
                    value:3000,
                    modifier: Default::default(),
                },
                iv::OverworldItem::Held{
                    actor:"Item_Fruit_A".to_string(), 
                },
                iv::OverworldItem::GroundEquipment{
                    actor:"Weapon_Sword_018".to_string(), 
                    value: 2600,
                    modifier: iv::WeaponModifier {
                        flag: 0x1,
                        value: 100,
                    }
                },
                iv::OverworldItem::GroundItem{
                    actor:"Item_Fruit_A".to_string(), 
                },
            ]
        }
    }

    fn get_state_by_step(&self, step: usize) -> Option<Arc<State>> {
        match self.states.get(step) {
            Some(state) => Some(Arc::clone(state)),
            None => Some(Arc::clone(self.states.last()?)),
        }
    }
}

pub async fn run_parsed(parsed: &ParseOutput) -> RunOutput {
    RunOutput { states: vec![] }
}

#[derive(Clone)]
pub struct State {
    // named gamedata in saves
    saves: HashMap<String, u64>,

    // gamedata in manual save
    manual_save: u64,

    /// Current game state
    game: Game,

    /// Current screen, only valid if game is running
    screen: Screen,

    /// If inventory/dialog screen is activated manually,
    /// so auto-scoping will be disabled until returned to overworld screen
    is_manual_scope: bool,

    /// If auto scope is enabled at all
    enable_auto_scope: bool,
}

#[derive(Clone)]
pub enum Game {
    /// Game is not booted
    Off,
    /// Game is running
    Running(GameState),
    /// Game has crashed (must manually reboot)
    Crashed(Error) // TODO: more crash info (dump, stack trace, etc)
}

#[derive(Clone)]
pub enum Screen {
    /// In the overworld, no additional screens
    Overworld,
    /// In the inventory screen
    Inventory,
    /// In an unknown dialog (could be sell/statue, or other)
    Dialog,
    /// In sell dialog
    DialogSell,
    /// In statue dialog
    DialogStatue,
}

/// State available when the game is running
#[derive(Clone)]
pub struct GameState {
    // gamedata TriggerParam*
    gamedata: u64,
    // memory states
    //
    /// Full process memory
    memory: Memory,

    /// Proxy objects in memory
    proxies: Proxies,

    /// Current actors in the overworld
    /// TODO: make this copy on write and Arc
    ovwd_weapon: Option<ActorState>,
    ovwd_shield: Option<ActorState>,
    ovwd_bow: Option<ActorState>,
    ovwd_armor_head: Option<ActorState>,
    ovwd_armor_upper: Option<ActorState>,
    ovwd_armor_lower: Option<ActorState>,

    ovwd_dropped_materials: VecDeque<ActorState>,
    ovwd_dropped_equipments: VecDeque<ActorState>,

    ovwd_holding_materials: VecDeque<ActorState>,

    entangled_slots: Vec<u32>,
}

#[derive(Clone)]
pub struct ActorState {
    pub name: String,
    pub life: u32,
    pub modifier_bits: u32,
    pub modifier_value: i32,
}

impl GameState {

    // just a placeholder
    // probably some kind of macro to generate these
    pub async fn get_item(mut self, scheduler: impl Scheduler, item: &str) -> Result<GameState, Error> {
        scheduler.run_on_core(move |p| {
            let core = p.attach(&mut self.memory, &mut self.proxies);
            // todo: real function
            core.pmdm_item_get(item, 0, 0)?;

            Ok(self)
        }).await
    }
}
