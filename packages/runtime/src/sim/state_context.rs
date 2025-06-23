use std::sync::Arc;

use blueflame::processor::{self, Cpu1, Cpu2, CrashReport, Process};
use teleparse::Span;

use crate::error::{Report, sim_error, sim_warning};
use crate::{ErrorReport, exec, sim};

impl sim::State {
    // these are state context helpers that commands depend on

    /// Ensure the game is already running, or initialize it if not,
    /// then execute the provided function with the game state.
    ///
    /// If the game has crashed previously, it will return an error
    pub async fn with_game<'a, TOutput, TFuture, TFn>(
        mut self,
        ctx: Context<&'a sim::Runtime>,
        f: TFn,
    ) -> Result<Report<Self>, exec::Error>
    where
        TOutput: IntoGameReport,
        TFuture: Future<Output = Result<TOutput, exec::Error>>,
        TFn: FnOnce(sim::GameState, Context<&'a sim::Runtime>) -> TFuture,
    {
        match self.game.take() {
            TakeGame::Crashed => Ok(Report::error(self, sim_warning!(&ctx.span, PreviousCrash))),
            TakeGame::Running(game) => {
                let span = ctx.span;
                let report = f(game, ctx).await?.into_report(span);
                Ok(report.map(|game| {
                    self.game = game;
                    self
                }))
            }
            TakeGame::Uninit => {
                let process = match ctx.runtime().initial_process() {
                    Ok(process) => process,
                    Err(e) => {
                        return Ok(Report::spanned(self, &ctx.span, e));
                    }
                };
                let span = ctx.span;
                let game_state = sim::GameState::new(process);
                let report = f(game_state, ctx).await?.into_report(span);
                Ok(report.map(|game| {
                    self.game = game;
                    self
                }))
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
    Running(sim::GameState),
    /// Game has crashed (must manually reboot)
    Crashed,
}

impl sim::Game {
    /// Take out the game state if it's running
    pub fn take(&mut self) -> TakeGame {
        match std::mem::take(self) {
            sim::Game::Uninit => TakeGame::Uninit,
            sim::Game::Running(game) => TakeGame::Running(game),
            sim::Game::Crashed(report) => {
                *self = sim::Game::Crashed(report);
                TakeGame::Crashed
            }
        }
    }
}

#[derive(Clone)]
pub struct Context<T> {
    /// The span of the current step being ran, used for emitting diagnostics
    pub span: Span,
    /// The handle for checking if the run should be aborted,
    /// if the step has potential long-running operations
    handle: Arc<sim::RunHandle>,
    /// The Runtime used for execution
    pub inner: T,
}

impl<T> Context<T> {
    pub fn new(handle: Arc<sim::RunHandle>, inner: T) -> Self {
        Self {
            span: Span::new(0, 0),
            handle,
            inner,
        }
    }
    pub fn is_aborted(&self) -> bool {
        self.handle.is_aborted()
    }
}

// make we type less
impl Context<&sim::Runtime> {
    pub fn runtime(&self) -> &sim::Runtime {
        self.inner
    }

    pub async fn execute<T, F>(self, f: F) -> Result<T, exec::Error>
    where
        F: FnOnce(Context<&mut Cpu1>) -> T + Send + 'static,
        T: Send + 'static,
    {
        let span = self.span;
        let handle = self.handle;
        self.inner
            .execute(move |cpu| {
                let ctx = Context {
                    span,
                    handle,
                    inner: cpu,
                };
                f(ctx)
            })
            .await
    }
}

impl Context<&mut Cpu1> {
    /// Execute the closure on the CPU and the game process
    pub fn execute<F>(self, mut state: sim::GameState, f: F) -> Result<sim::GameState, CrashReport>
    where
        F: FnOnce(Context<&mut Cpu2>) -> Result<(), processor::Error>,
    {
        let span = self.span;
        let handle = self.handle;
        let cpu1 = self.inner;
        let mut cpu2 = Cpu2::new(cpu1, &mut state.process);
        cpu2.with_crash_report(|cpu2| {
            let ctx = Context {
                span,
                handle,
                inner: cpu2,
            };
            f(ctx)
        })?;
        Ok(state)
    }

    /// Execute the closure on the CPU and the game process, with reporting functionality
    pub fn execute_reporting<F>(
        self,
        mut state: sim::GameState,
        f: F,
    ) -> Result<Report<sim::GameState>, CrashReport>
    where
        F: FnOnce(Context<&mut Cpu2>, &mut Vec<ErrorReport>) -> Result<(), processor::Error>,
    {
        let span = self.span;
        let handle = self.handle;
        let cpu1 = self.inner;
        let mut cpu2 = Cpu2::new(cpu1, &mut state.process);
        let mut errors = vec![];
        cpu2.with_crash_report(|cpu2| {
            let ctx = Context {
                span,
                handle,
                inner: cpu2,
            };
            f(ctx, &mut errors)
        })?;
        Ok(Report::with_errors(state, errors))
    }
}

impl<'a, 'b> Context<&mut Cpu2<'a, 'b>> {
    pub fn cpu(&mut self) -> &mut Cpu2<'a, 'b> {
        &mut self.inner
    }
}

pub trait IntoGameReport {
    fn into_report(self, span: Span) -> Report<sim::Game>;
}

impl IntoGameReport for Result<sim::GameState, CrashReport> {
    fn into_report(self, span: Span) -> Report<sim::Game> {
        match self {
            Ok(game_state) => Report::new(sim::Game::Running(game_state)),
            Err(crash_report) => {
                Report::error(sim::Game::Crashed(crash_report), sim_error!(&span, Crash))
            }
        }
    }
}

impl IntoGameReport for Result<Report<sim::GameState>, CrashReport> {
    fn into_report(self, span: Span) -> Report<sim::Game> {
        match self {
            Ok(report) => report.map(sim::Game::Running),
            Err(crash_report) => {
                Report::error(sim::Game::Crashed(crash_report), sim_error!(&span, Crash))
            }
        }
    }
}

impl IntoGameReport for Report<sim::Game> {
    fn into_report(self, _: Span) -> Report<sim::Game> {
        self
    }
}
