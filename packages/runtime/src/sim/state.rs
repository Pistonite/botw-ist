use blueflame::processor::{CrashReport, Process};
use skybook_parser::cir;

use crate::error::{Report, sim_error};
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
}

#[derive(Clone, Default)]
pub enum Game {
    /// Game is never started
    #[default]
    Uninit,
    /// Game is running
    Running(Box<GameState>),
    /// Game has crashed (must manually reboot)
    Crashed(CrashReport),
}

/// The state of the running game in the simulator
#[derive(Clone)]
pub struct GameState {
    /// Running game's process
    pub process: Process,
    /// Simulated systems in the game
    pub systems: GameSystems,
}

#[derive(Default, Clone)]
pub struct GameSystems {
    /// Simulation of screens in the game
    pub screen: sim::ScreenSystem,
    /// Simulation of the overworld
    pub overworld: sim::OverworldSystem,
}

impl State {
    pub async fn execute_step(
        self,
        ctx: sim::Context<&sim::Runtime>,
        step: &cir::Step,
    ) -> Result<Report<Self>, exec::Error> {
        match step.command() {
            cir::Command::Get(items) => self.handle_get(ctx, items).await,
            cir::Command::Hold(items) => self.handle_hold(ctx, items).await,
            _ => Ok(Report::error(self, sim_error!(ctx.span, Unimplemented))),
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
            })
            .await
        })
        .await
    }

    async fn handle_hold(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD command");
        self.with_game(rt, async move |game, rt| {
            let items = items.to_vec();
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sim::actions::hold_items(&mut cpu2, sys, errors, &items)
                })
            })
            .await
        })
        .await
    }
}

impl GameState {
    pub fn new(process: Process) -> Self {
        Self {
            process,
            systems: GameSystems::default(),
        }
    }
}
