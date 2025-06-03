use std::collections::HashMap;
use std::sync::Arc;
use std::future::Future;

use blueflame::game::gdt;
use blueflame::processor::{CrashReport, Process};
use skybook_parser::cir;
use teleparse::Span;

use crate::{sim, ErrorReport};
use crate::error::{sim_error, Report};

/// The state of the simulator
#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub enum Game {
    /// Game is never started
    #[default]
    Uninit,
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

    /// Running game's process
    process: Process,
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

impl State {
    pub async fn execute_step(
        self, 
        span: Span,
        step: &cir::Step, 
        runtime: &sim::Runtime
    ) -> Report<Self> {
        match &step.command {
            cir::Command::Get(items) => {
                self.handle_get(span, items, runtime).await
            }
            _ => {
                let mut report = Report::new(self);
                report.push(sim_error!(
                    &span,
                    Unimplemented
                ));
                report
            }
        }
    }

    async fn handle_get(
        self,
        span: Span,
        items: &[cir::ItemSpec],
        runtime: &sim::Runtime,
    ) -> Report<Self> {
        self.with_game(span, runtime, async |game| {
            todo!()
        }).await
    }

    /// Ensure the game is already running, or initialize it if not,
    /// then execute the provided function with the game state.
    ///
    /// If the game has crashed previously, it will return an error
    async fn with_game<TFuture, TFn>(
        mut self,
        span: Span,
        runtime: &sim::Runtime,
        f: TFn,
    ) -> Report<Self> 
    where 
        TFuture: Future<Output = Report<Game>>,
        TFn: FnOnce(GameState) -> TFuture,
    {
        match self.game {
            Game::Crashed(_) => {
                let mut report = Report::new(self);
                report.push(sim_error!(
                    &span,
                    PreviousCrash
                ));
                report
            }
            Game::Running(game) => {
                let report = f(game).await;
                self.game = report.value;
                Report::with_errors(self, report.errors)
            }
            Game::Uninit => {
                let process = match runtime.initial_process() {
                    Ok(process) => process,
                    Err(e) => {
                        let mut report = Report::new(self);
                        report.push(ErrorReport::error(&span, e));
                        return report;
                    }
                };
                let game_state = GameState::new(process);
                let report = f(game_state).await;
                self.game = report.value;
                Report::with_errors(self, report.errors)
            }
        }
    }

}

impl GameState {
    pub fn new(process: Process) -> Self {
        Self {
            screen: Screen::Overworld,
            process,
        }
    }
}
