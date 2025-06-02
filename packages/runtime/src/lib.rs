use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, atomic::AtomicBool},
};

use blueflame::env::{DlcVer, Environment, GameVer};
use blueflame::linker;
use blueflame::program;
use error::MaybeAborted;
use exec::{Executor, Spawner};
use serde::{Deserialize, Serialize};
use skybook_parser::ParseOutput;

/// Executor - handles pooling script execution on multiple emulator cores
pub mod exec;

pub mod error;
/// Inventory View
pub mod iv;
pub mod pointer;

/// External ref counting helpers
pub mod erc;

/// Simulator
pub mod sim;

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


pub async fn run_parsed(
    parsed: Arc<ParseOutput>,
    handle: Arc<RunHandle>,
) -> MaybeAborted<RunOutput> {
    MaybeAborted::Ok(RunOutput { states: vec![] })
}

use blueflame::game::gdt;
use blueflame::processor::Process;


#[derive(Clone)]
pub struct ActorState {
    pub name: String,
    pub life: u32,
    pub modifier_bits: u32,
    pub modifier_value: i32,
}

