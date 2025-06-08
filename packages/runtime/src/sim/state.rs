use std::collections::HashMap;
use std::sync::Arc;
use std::future::Future;

use blueflame::game::gdt;
use blueflame::linker;
use blueflame::processor::{self, Cpu1, Cpu2, CrashReport, Process};
use skybook_parser::cir;
use teleparse::Span;

use crate::{exec, sim};
use crate::error::{sim_error, sim_warning, Report};
use super::util;

/// The state of the simulator
#[derive(Clone, Default)]
pub struct State {
    /// Current game state
    pub game: Game,
    // /// named save data
    // saves: HashMap<String, Arc<gdt::TriggerParam>>,
    // /// The "manual" or "default" save (what is used if a name is not specified when saving)
    // manual_save: Option<gdt::TriggerParam>,



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

trait IntoGameReport {
    fn into_report(self, span: Span) -> Report<Game>;
}

impl IntoGameReport for Result<GameState, CrashReport> {
    fn into_report(self, span: Span) -> Report<Game> {
        match self {
            Ok(game_state) => Report::new(Game::Running(game_state)),
            Err(crash_report) => {
                Report::error(Game::Crashed(crash_report), 
                    sim_error!( &span, Crash))
            }
        }
    }
}

#[derive(Clone, Default)]
pub enum TakeGame {
    /// Game is never started
    #[default]
    Uninit,
    /// Game is running
    Running(GameState),
    /// Game has crashed (must manually reboot)
    Crashed
}

/// The state of the running game in the simulator
#[derive(Clone)]
pub struct GameState {
    /// Current screen, only valid if game is running
    screen: Screen,

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
    ) -> Result<Report<Self>, exec::Error> {
        match &step.command {
            cir::Command::Get(items) => {
                self.handle_get(span, items, runtime).await
            }
            _ => {
                Ok(Report::error(
                    self,
                    sim_error!(
                        &span,
                        Unimplemented
                    )
                ))
            }
        }
    }

    async fn handle_get(
        self,
        span: Span,
        items: &[cir::ItemSpec],
        runtime: &sim::Runtime,
    ) -> Result<Report<Self>, exec::Error> {
        log::debug!("Handling GET command");
        let x = self.with_game(span, runtime, async move |game| {
            let items = items.to_vec();
            runtime.execute(move |cpu| {
                game.cmd_get(cpu, &items).into_report(span)
            }).await
        }).await;
        log::debug!("Done with GET command");
        x
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
    ) -> Result<Report<Self>, exec::Error>
    where 
        TFuture: Future<Output = Result<Report<Game>, exec::Error>>,
        TFn: FnOnce(GameState) -> TFuture,
    {
        match self.game.take() {
            TakeGame::Crashed => {
                Ok(Report::error(self, sim_warning!(
                    &span,
                    PreviousCrash
                )))
            }
            TakeGame::Running(game) => {
                let report = f(game).await?;
                Ok(report.map(|game| {
                    self.game = game;
                    self
                }))
            }
            TakeGame::Uninit => {
                let process = match runtime.initial_process() {
                    Ok(process) => process,
                    Err(e) => {
                        return Ok(Report::spanned(
                            self,
                            &span, e
                        ));
                    }
                };
                let game_state = GameState::new(process);
                let report = f(game_state).await?;
                Ok(report.map(|game| {
                    self.game = game;
                    self
                }))
            }
        }
    }

}

impl Game {
    /// Take out the game state if it's running, leaving Uninit in its place
    pub fn take(&mut self) -> TakeGame {
        match std::mem::take(self) {
            Game::Uninit => TakeGame::Uninit,
            Game::Running(game) => {
                TakeGame::Running(game)
            }
            Game::Crashed(_) => TakeGame::Crashed,
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

    pub fn cmd_get(self, cpu: &mut Cpu1, items: &[cir::ItemSpec]) -> Result<Self, CrashReport> {
        self.cpu_exec(cpu, |cpu2| {
            for item in items {
                let amount = item.amount;
                let item = &item.item;
                let is_cook_item = item.is_cook_item();
                for _ in 0..amount {
                    let meta = item.meta.as_ref();
                    if is_cook_item {
                        linker::get_cook_item(
                            cpu2, 
                            &item.actor,
                            meta.map(|m| m.ingredients.as_slice()).unwrap_or(&[]),
                            meta.and_then(|m| m.life_recover_f32()),
                            meta.and_then(|m| m.effect_duration),
                            meta.and_then(|m| m.sell_price),
                            meta.and_then(|m| m.effect_id),
                            meta.and_then(|m| m.effect_level),
                        )?;
                        continue;
                    };
                    let modifier = util::modifier_from_meta(meta);
                    linker::get_item(cpu2, &item.actor, 
                        meta.and_then(|m| m.value),
                        modifier)?;
                }
            }
            Ok(())
        })
    }

    fn cpu_exec<F>(mut self, cpu: &mut Cpu1, f: F) -> Result<Self, CrashReport>
    where F: FnOnce(&mut Cpu2) -> Result<(), processor::Error>
    {
        let mut cpu2 = Cpu2::new(cpu, &mut self.process);
        cpu2.with_crash_report(f)?;
        Ok(self)
    }
}
