use std::{collections::{HashMap, VecDeque}, future::Future, sync::{atomic::{AtomicBool, AtomicU64}, Arc}};

use skybook_parser::{search::QuotedItemResolver, ParseOutput};
use blueflame::{error::Error, memory::{Memory, Proxies}};

mod scheduler;
use scheduler::Scheduler;

pub mod inventory;

pub struct RunOutput {
    /// State at each simulation step
    pub states: Vec<Arc<State>>
}

impl RunOutput {
    // TODO: error
    pub fn get_inventory_list_view(&self, step: usize) -> inventory::InventoryListView {
        // mock data

        let items = vec![
            inventory::ItemSlotInfo {
                actor_name: "Weapon_Sword_070".to_string(),
                item_type: 0,
                item_use: 0,
                value: 4000,
                is_equipped: true,
                is_in_inventory: true,
                mod_effect_value: 0,
                mod_effect_duration: 0,
                mod_sell_price: 0,
                mod_effect_id: 0f32,
                mod_effect_level: 0f32,
                ingredient_actor_names: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
                unaligned: false,
                prev_node_ptr: 0.into(),
                next_node_ptr: 0.into(),
                list_pos: 0,
                unallocated: false,
                pool_pos: 419,
                is_in_broken_slot: false,
                holding_count: 0,
                prompt_entangled: false
            },
            inventory::ItemSlotInfo {
                actor_name: "Item_Fruit_A".to_string(),
                item_type: 7,
                item_use: 8,
                value: 5,
                is_equipped: false,
                is_in_inventory: true,
                mod_effect_value: 0,
                mod_effect_duration: 0,
                mod_sell_price: 0,
                mod_effect_id: 0f32,
                mod_effect_level: 0f32,
                ingredient_actor_names: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
                unaligned: false,
                prev_node_ptr: 0.into(),
                next_node_ptr: 0.into(),
                list_pos: 1,
                unallocated: false,
                pool_pos: 418,
                is_in_broken_slot: false,
                holding_count: 0,
                prompt_entangled: false
            }
        ];

        inventory::InventoryListView {
            info: inventory::InventoryInfo {
                num_tabs: 2,
                tabs: vec![
                    inventory::TabInfo {
                        item_idx: Some(0),
                        item_ptr: 0.into(),
                        tab_type: 0
                    },
                    inventory::TabInfo {
                        item_idx: Some(1),
                        item_ptr: 0.into(),
                        tab_type: 5
                    },
                ]
            },
            items,
        }


        // let Some(state) = self.get_state_by_step(step) else {
        //     return Ok(vec![]);
        // };
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
