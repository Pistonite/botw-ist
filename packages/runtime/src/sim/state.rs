use std::collections::HashMap;
use std::sync::Arc;

use blueflame::game::gdt;
use blueflame::processor::CrashReport;
use skybook_parser::cir;

use crate::sim;

/// The state of the simulator
#[derive(Clone)]
pub struct State {
    /// Current game state
    game: Game,
    /// named save data
    saves: HashMap<String, Arc<gdt::TriggerParam>>,
    /// The "manual" or "default" save (what is used if a name is not specified when saving)
    manual_save: Option<gdt::TriggerParam>,



    // /// If inventory/dialog screen is activated manually,
    // /// so auto-scoping will be disabled until returned to overworld screen
    // is_manual_scope: bool,
    //
    // /// If auto scope is enabled at all
    // enable_auto_scope: bool,
}

#[derive(Clone)]
pub enum Game {
    /// Game is not booted
    Off,
    /// Game is running
    Running(GameState),
    /// Game has crashed (must manually reboot)
    Crashed(CrashReport)
}

/// The state of the running game in the simulator
#[derive(Clone)]
pub struct GameState {
    /// Current screen, only valid if game is running
    screen: Screen,

    // /// Running game's process
    // process: Process,
    //
    // /// Current actors in the overworld
    // /// TODO: make this copy on write and Arc
    // ovwd_weapon: Option<ActorState>,
    // ovwd_shield: Option<ActorState>,
    // ovwd_bow: Option<ActorState>,
    // ovwd_armor_head: Option<ActorState>,
    // ovwd_armor_upper: Option<ActorState>,
    // ovwd_armor_lower: Option<ActorState>,
    //
    // ovwd_dropped_materials: VecDeque<ActorState>,
    // ovwd_dropped_equipments: VecDeque<ActorState>,
    //
    // ovwd_holding_materials: VecDeque<ActorState>,
    //
    // entangled_slots: Vec<u32>,
}

#[derive(Clone)]
pub enum Screen {
    /// In the overworld, no additional screens
    Overworld,
    /// In the inventory screen
    Inventory,
    /// In an unknown dialog (could be sell/statue, or other)
    Dialog,
    // /// In sell dialog
    // DialogSell,
    // /// In statue dialog
    // DialogStatue,
}
