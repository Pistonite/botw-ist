use blueflame::processor::{CrashReport, Process};
use skybook_parser::cir;

use crate::error::{Report, sim_error};
use crate::sim::ScreenSystem;
use crate::{exec, sim};

/// The state of the simulator
#[derive(Clone, Default)]
pub struct State {
    /// Current game state
    pub game: Game,
    // /// named save data
    // saves: HashMap<String, Arc<gdt::TriggerParam>>,
    // /// The "manual" or "default" save (what is used if a name is not specified when saving)
    // manual_save: Option<gdt::TriggerParam>,
    
    /// If the screen was manually changed by a command
    ///
    /// If so, the simulator will not automatically change screen
    /// until the screen returns to overworld
    pub is_screen_manually_changed: bool,

    // /// If inventory/dialog screen is activated manually,
    // /// so auto-scoping will be disabled until returned to overworld screen
    // is_manual_scope: bool,
    //
    // /// If auto scope is enabled at all
    // enable_auto_scope: bool,
}

#[derive(Clone, Default)]
pub enum Game {
    /// Game is never started
    #[default]
    Uninit,
    /// Game is running
    Running(GameState),
    /// Game has crashed (must manually reboot)
    Crashed(CrashReport),
}

/// The state of the running game in the simulator
#[derive(Clone)]
pub struct GameState {
    /// Simulation of screens in the game
    pub screen: sim::ScreenSystem,
    /// Running game's process
    pub process: Process,
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
}

impl State {
    pub async fn execute_step(
        self,
        ctx: sim::Context<&sim::Runtime>,
        step: &cir::Step,
    ) -> Result<Report<Self>, exec::Error> {
        match &step.command {
            cir::Command::Get(items) => self.handle_get(ctx, items).await,
            _ => Ok(Report::error(self, sim_error!(&ctx.span, Unimplemented))),
        }
    }

    async fn handle_get(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling GET command");
        self.with_game(rt, async move |game, rt| {
            let items = items.to_vec();
            rt.execute(move |cpu| {
                cpu.execute(game, |mut cpu2| sim::actions::get_items(&mut cpu2, &items))
            }).await
        })
        .await
    }
}

impl GameState {
    pub fn new(process: Process) -> Self {
        Self {
            screen: ScreenSystem::default(),
            process,
        }
    }
}
