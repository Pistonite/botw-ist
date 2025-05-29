use std::{
    collections::{HashMap, VecDeque},
    future::Future,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64},
    },
};

use blueflame::env::{DlcVer, Environment, GameVer};
use blueflame::game::Proxies;
use blueflame::linker;
use blueflame::memory::Memory;
use blueflame::processor::Process;
use blueflame::program;
use error::MaybeAborted;
use exec::{Executor, Spawner};
use serde::{Deserialize, Serialize};
use skybook_parser::{ParseOutput, search::QuotedItemResolver};

/// Executor - handles pooling script execution on multiple emulator cores
pub mod exec;

pub mod error;
/// Inventory View
pub mod iv;
pub mod pointer;

/// External ref counting helpers
pub mod erc;

pub struct RunOutput {
    /// State at each simulation step
    pub states: Vec<Arc<State>>,
}

impl RunOutput {
    // TODO: error
    pub fn get_pouch_list(&self, step: usize) -> iv::PouchList {
        // mock data
        let mock_item1 = iv::PouchItem {
            common: iv::CommonItem {
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
            ingredients: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            holding_count: 0,
            prompt_entangled: false,
            node_addr: 0.into(),
            node_valid: true,
            node_pos: 419,
            node_prev: 0.into(),
            node_next: 0.into(),
            allocated_idx: 0,
            unallocated_idx: -1,
            tab_idx: 0,
            tab_slot: 0,
            accessible: true,
            dpad_accessible: true,
        };

        let mock_item2 = iv::PouchItem {
            common: iv::CommonItem {
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
            ingredients: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            holding_count: 0,
            prompt_entangled: false,
            node_addr: 0.into(),
            node_valid: true,
            node_pos: 418,
            node_prev: 0.into(),
            node_next: 0.into(),
            allocated_idx: 1,
            unallocated_idx: -1,
            tab_idx: 1,
            tab_slot: 0,
            accessible: true,
            dpad_accessible: true,
        };

        let mock_item3 = iv::PouchItem {
            common: iv::CommonItem {
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
            ingredients: [
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            holding_count: 0,
            prompt_entangled: false,
            node_addr: 0.into(),
            node_valid: true,
            node_pos: 418,
            node_prev: 0.into(),
            node_next: 0.into(),
            allocated_idx: 2,
            unallocated_idx: -1,
            tab_idx: 3,
            tab_slot: 0,
            accessible: true,
            dpad_accessible: true,
        };

        let mut items = vec![];
        items.push(mock_item1);
        if step >= 1 {
            items.push(mock_item2);
        }
        if step >= 2 {
            items.push(mock_item3);
        }

        let tabs = match step {
            0 => vec![iv::PouchTab {
                item_idx: 0,
                tab_type: 0, // sword
            }],
            1 => vec![
                iv::PouchTab {
                    item_idx: 0,
                    tab_type: 0, // sword
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
                    tab_type: 0, // sword
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
            ],
        };

        iv::PouchList {
            count: items.len() as i32 - 1,
            items,
            are_tabs_valid: true,
            num_tabs: tabs.len() as i32,
            tabs,
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
                    common: iv::CommonItem {
                        actor_name: "Weapon_Sword_070".to_string(),
                        value: 4000,
                        is_equipped: true,
                    },
                    idx: 0,
                    data: iv::GdtItemData::Sword {
                        idx: 0,
                        info: iv::WeaponModifier { flag: 0, value: 0 },
                    },
                },
                iv::GdtItem {
                    common: iv::CommonItem {
                        actor_name: "Item_Fruit_A".to_string(),
                        value: 999,
                        is_equipped: false,
                    },
                    idx: 1,
                    data: iv::GdtItemData::None,
                },
            ],
            master_sword: iv::GdtMasterSword {
                is_true_form: true,
                add_power: 30,
                add_beam_power: 10,
                recover_time: 0f32,
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
            },
        };
    }

    pub fn get_overworld_items(&self, step: usize) -> iv::Overworld {
        // mock data
        iv::Overworld {
            items: vec![
                iv::OverworldItem::Equipped {
                    actor: "Weapon_Sword_070".to_string(),
                    value: 3000,
                    modifier: Default::default(),
                },
                iv::OverworldItem::Held {
                    actor: "Item_Fruit_A".to_string(),
                },
                iv::OverworldItem::GroundEquipment {
                    actor: "Weapon_Sword_018".to_string(),
                    value: 2600,
                    modifier: iv::WeaponModifier {
                        flag: 0x1,
                        value: 100,
                    },
                },
                iv::OverworldItem::GroundItem {
                    actor: "Item_Fruit_A".to_string(),
                },
            ],
        }
    }

    fn get_state_by_step(&self, step: usize) -> Option<Arc<State>> {
        match self.states.get(step) {
            Some(state) => Some(Arc::clone(state)),
            None => Some(Arc::clone(self.states.last()?)),
        }
    }
}

pub struct Run {
    handle: Arc<RunHandle>,
    states: Vec<Arc<State>>,
}

pub struct Runtime {
    pub env: Mutex<Environment>,
    pub executor: Executor,
    pub initial_process: Mutex<Option<Process>>,
    // TODO: pool + Spawn
    // TODO: initial memory (Mutex<Option<Arc<Memory>>> probably? or Mutex<Option<Memory>>)
}

impl Runtime {
    /// Create the runtime, but do not initialize it yet
    pub fn new(spawner: Spawner) -> Self {
        let executor = Executor::new(spawner);
        Self {
            env: Mutex::new(Environment::new(GameVer::X150, DlcVer::None)),
            executor,
            initial_process: Mutex::new(None),
        }
    }

    pub fn game_version(&self) -> GameVer {
        self.env.lock().unwrap().game_ver
    }
    pub fn dlc_version(&self) -> DlcVer {
        self.env.lock().unwrap().dlc_ver
    }

    /// Initialize the runtime
    pub fn init(
        &self,
        custom_image: Option<(Vec<u8>, CustomImageInitParams)>,
    ) -> Result<(), RuntimeInitError> {
        if let Err(e) = self.executor.ensure_threads(4) {
            log::error!("failed to create threads: {}", e);
            return Err(RuntimeInitError::Executor);
        }
        let Some((bytes, params)) = custom_image else {
            log::error!("must provide custom image for now");
            return Err(RuntimeInitError::BadImage);
        };

        log::debug!(
            "initializing runtime with custom image params: {:?}",
            params
        );

        let mut program_bytes = Vec::new();
        let program = match program::unpack_zc(&bytes, &mut program_bytes) {
            Err(e) => {
                log::error!("failed to unpack blueflame image: {}", e);
                return Err(RuntimeInitError::BadImage);
            }
            Ok(program) => program,
        };

        if program.ver == GameVer::X160 {
            log::error!(">>>> + LOOK HERE + <<<< Only 1.5 is supported for now");
            return Err(RuntimeInitError::BadImage);
        }

        log::debug!("program start: {:#x}", program.program_start);

        // TODO: program should not have DLC version, since
        // it doesn't matter statically
        {
            let mut env = self.env.lock().unwrap();
            env.game_ver = program.ver.into();
            env.dlc_ver = match DlcVer::from_num(params.dlc) {
                Some(dlc) => dlc,
                None => return Err(RuntimeInitError::BadDlcVersion(params.dlc)),
            };
        }

        // TODO: take the param
        log::debug!("initializing memory");

        let process = match linker::init_process(
            &program,
            DlcVer::V300, // TODO: take from param
            0x8888800000,
            0x4000, // stack size
            0x2222200000,
            20000000, // this heap looks hug
        ) {
            Err(e) => {
                log::error!("failed to initialize memory: {}", e);
                // TODO: actual error
                return Err(RuntimeInitError::BadImage);
            }
            Ok(x) => x,
        };

        log::debug!("memory initialized successfully");
        {
            let mut initial_memory = self.initial_process.lock().unwrap();
            *initial_memory = Some(process);
        }

        // todo!()
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct CustomImageInitParams {
    /// DLC version to simulate
    ///
    /// 0 means no DLC, 1-3 means DLC version 1.0, 2.0, or 3.0
    #[serde(default)]
    pub dlc: u32,

    /// Program start address
    ///
    /// The string should look like 0x000000XXXXX00000, where X is a hex digit
    ///
    /// Unspecified (empty string) means the script can run with any program start address
    #[serde(default)]
    pub program_start: String,

    /// Stack start address
    ///
    /// The string should look like 0x000000XXXXX00000, where X is a hex digit
    ///
    /// Unspecified (empty string) means using the internal default
    #[serde(default)]
    pub stack_start: String,

    /// Size of the stack
    ///
    /// Unspecified, or 0, means using the internal default
    #[serde(default)]
    pub stack_size: u32,

    /// Size of the free region of the heap
    ///
    /// Unspecified, or 0, means using the internal default
    #[serde(default)]
    pub heap_free_size: u32,

    /// Physical address of the PauseMenuDataMgr. Used to calculate heap start
    ///
    /// Unspecified (empty string) means using the internal default
    #[serde(default)]
    pub pmdm_addr: String,
}

#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(tag = "type", content = "data")]
pub enum RuntimeInitError {
    #[error("executor error")]
    Executor,
    #[error("invalid DLC version: {0}. Valid versions are 0, 1, 2 or 3")]
    BadDlcVersion(u32),
    #[error("invalid custom image (1.6 is not supported right now)")]
    BadImage,
    #[error("program-start param is invalid")]
    InvalidProgramStart,
    #[error("stack-start param is invalid")]
    InvalidStackStart,
    #[error("pmdm-addr param is invalid")]
    InvalidPmdmAddr,
    #[error(
        "the custom image provided has program-start = {0}, which does not match the one requested by the environment = {0}"
    )]
    ProgramStartMismatch(String, String),
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInitOutput {
    /// "1.5" or "1.6"
    pub game_version: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub enum ResultInterop<T, E> {
    /// Result<T, Error>
    #[serde(rename = "val")]
    Ok(T),
    /// Result<Error, Error>
    #[serde(rename = "err")]
    Err(E),
}

pub async fn run_parsed(
    parsed: Arc<ParseOutput>,
    handle: Arc<RunHandle>,
) -> MaybeAborted<RunOutput> {
    MaybeAborted::Ok(RunOutput { states: vec![] })
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
    // /Game has crashed (must manually reboot)
    // Crashed(Error) // TODO: more crash info (dump, stack trace, etc)
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
    // // just a placeholder
    // // probably some kind of macro to generate these
    // pub async fn get_item_with_pool<S: pool::Spawn>(
    //     mut self, pool: &mut pool::Pool<S>, item: String
    // ) -> Result<GameState, crate::error::Error> {
    //     let state = pool.execute(move |p| {
    //         let mut core = p.attach(&mut self.memory, &mut self.proxies);
    //         // todo: real function
    //         core.pmdm_item_get(&item, 0, 0,0).unwrap();
    //         self
    //     }).await?;
    //
    //     Ok(state)
    // }

    // // this kind would already have the processor
    // pub fn get_item<S: pool::Spawn>(
    //     &mut self, cpu: &mut Processor, item: &str
    // ) -> Result<(), crate::error::Error> {
    //     let mut core = cpu.attach(&mut self.memory, &mut self.proxies);
    //     // todo: real function
    //     core.pmdm_item_get(item, 0, 0,0).unwrap();
    //
    //     Ok(())
    // }
}

#[repr(transparent)]
pub struct RunHandle {
    is_aborted: AtomicBool,
}

impl RunHandle {
    pub fn new() -> Self {
        Self {
            is_aborted: AtomicBool::new(false),
        }
    }
    pub fn is_aborted(&self) -> bool {
        self.is_aborted.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn abort(&self) {
        self.is_aborted
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn into_raw(s: Arc<Self>) -> *const Self {
        return Arc::into_raw(s);
    }
    pub fn from_raw(ptr: *const Self) -> Arc<Self> {
        if ptr.is_null() {
            // make sure it's safe
            return Arc::new(Self::new());
        }
        return unsafe { Arc::from_raw(ptr) };
    }
}
