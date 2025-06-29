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
    /// Game has crashed in the last step (must manually reboot)
    Crashed(CrashReport),
    /// Game has crashed in a previous step
    PreviousCrash,
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
            cir::Command::Get(items) => self.handle_get(ctx, items, false).await,
            cir::Command::GetPause(items) => self.handle_get(ctx, items, true).await,
            cir::Command::Hold(items) => self.handle_hold(ctx, items, false).await,
            cir::Command::HoldAttach(items) => self.handle_hold(ctx, items, true).await,
            cir::Command::Drop(items) => self.handle_drop(ctx, items).await,
            cir::Command::DropHeld => self.handle_drop_held(ctx).await,
            cir::Command::Unhold => self.handle_unhold(ctx).await,
            cir::Command::OpenInv => self.handle_pause(ctx).await,
            cir::Command::CloseInv => self.handle_unpause(ctx).await,
            _ => Ok(Report::error(self, sim_error!(ctx.span, Unimplemented))),
        }
    }

    async fn handle_get(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSpec],
        pause_after: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling GET command");
        self.with_game(rt, async move |game, rt| {
            let items = items.to_vec();
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sim::actions::get_items(&mut cpu2, sys, errors, &items, pause_after)?;
                    sys.overworld.despawn_items();
                    Ok(())
                })
            })
            .await
        })
        .await
    }

    async fn handle_hold(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
        attached: bool,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD command");
        self.with_game(rt, async move |game, rt| {
            let items = items.to_vec();
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sim::actions::hold_items(&mut cpu2, sys, errors, &items, attached)
                })
            })
            .await
        })
        .await
    }

    async fn handle_unhold(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD command");
        self.with_game(rt, async move |game, rt| {
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sim::actions::unhold(&mut cpu2, sys, errors)
                })
            })
            .await
        })
        .await
    }
    async fn handle_drop(
        self,
        rt: sim::Context<&sim::Runtime>,
        items: &[cir::ItemSelectSpec],
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling HOLD -> DROP command");
        self.with_game(rt, async move |game, rt| {
            let items = items.to_vec();
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sim::actions::hold_items(&mut cpu2, sys, errors, &items, false)?;
                    sim::actions::drop_held(&mut cpu2, sys, errors)
                })
            })
            .await
        })
        .await
    }

    async fn handle_drop_held(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling DROP command");
        self.with_game(rt, async move |game, rt| {
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sim::actions::drop_held(&mut cpu2, sys, errors)
                })
            })
            .await
        })
        .await
    }

    async fn handle_pause(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling PAUSE command");
        self.with_game(rt, async move |game, rt| {
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sys.screen
                        .transition_to_inventory(&mut cpu2, &mut sys.overworld, true, errors)
                })
            })
            .await
        })
        .await
    }

    async fn handle_unpause(
        self,
        rt: sim::Context<&sim::Runtime>,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling UNPAUSE command");
        self.with_game(rt, async move |game, rt| {
            rt.execute(move |cpu| {
                cpu.execute_reporting(game, |mut cpu2, sys, errors| {
                    sys.screen.transition_to_overworld(
                        &mut cpu2,
                        &mut sys.overworld,
                        true,
                        errors,
                    )?;
                    sys.overworld.despawn_items();
                    Ok(())
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
