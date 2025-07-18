use std::sync::Arc;

use blueflame::processor::{self, Cpu1, Cpu2, CrashReport};
use teleparse::Span;

use crate::error::{Report, sim_error, sim_warning};
use crate::{ErrorReport, exec, sim};

impl sim::State {
    // these are state context helpers that commands depend on

    /// Ensure the game is already running, or initialize it if not,
    /// then execute the provided function with the game state.
    ///
    /// If the game has crashed previously, it will return an error
    #[inline]
    pub async fn with_game<'a, TOutput, TFuture, TFn>(
        self,
        ctx: Context<&'a sim::Runtime>,
        f: TFn,
    ) -> Result<Report<Self>, exec::Error>
    where
        TOutput: IntoGameReport,
        TFuture: Future<Output = Result<TOutput, exec::Error>>,
        TFn: FnOnce(sim::GameState, Context<&'a sim::Runtime>) -> TFuture,
    {
        self.with_game_exec_internal(ctx, f, false).await
    }

    /// Ensure the game is already running, or initialize it if not,
    /// then execute the provided function with the game state.
    ///
    /// Will initialize the game no matter how it was closed
    #[inline]
    pub async fn with_game_or_start<'a, TOutput, TFuture, TFn>(
        self,
        ctx: Context<&'a sim::Runtime>,
        f: TFn,
    ) -> Result<Report<Self>, exec::Error>
    where
        TOutput: IntoGameReport,
        TFuture: Future<Output = Result<TOutput, exec::Error>>,
        TFn: FnOnce(sim::GameState, Context<&'a sim::Runtime>) -> TFuture,
    {
        self.with_game_exec_internal(ctx, f, true).await
    }

    async fn with_game_exec_internal<'a, TOutput, TFuture, TFn>(
        mut self,
        ctx: Context<&'a sim::Runtime>,
        f: TFn,
        start_if_closed: bool,
    ) -> Result<Report<Self>, exec::Error>
    where
        TOutput: IntoGameReport,
        TFuture: Future<Output = Result<TOutput, exec::Error>>,
        TFn: FnOnce(sim::GameState, Context<&'a sim::Runtime>) -> TFuture,
    {
        match self.game.take_for_execute(start_if_closed) {
            TakeGame::Crashed => {
                self.game = sim::Game::PreviousCrash;
                Ok(Report::error(self, sim_warning!(ctx.span, PreviousCrash)))
            }
            TakeGame::PreviousCrash | TakeGame::PreviousClosed => Ok(Report::new(self)),
            TakeGame::Closed => {
                self.game = sim::Game::PreviousClosed;
                Ok(Report::error(self, sim_warning!(ctx.span, PreviousClosed)))
            }
            TakeGame::Running(game) => {
                let span = ctx.span;
                let report = f(*game, ctx).await?.into_report(span);
                Ok(report.map(|game| {
                    self.game = game;
                    self
                }))
            }
            TakeGame::Uninit => {
                let process = match ctx.runtime().initial_process() {
                    Ok(process) => process,
                    Err(e) => {
                        return Ok(Report::spanned(self, ctx.span, e));
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

    /// Ensure the game is already running, or initialize it if not,
    /// then execute the provided function without access to the runtime
    pub fn with_game_no_exec<
        TOutput,
        TFn: FnOnce(&mut sim::GameState, Span, &mut Vec<ErrorReport>) -> TOutput,
    >(
        &mut self,
        ctx: Context<&'_ sim::Runtime>,
        f: TFn,
    ) -> Report<Option<TOutput>> {
        if let sim::Game::Uninit = &self.game {
            let process = match ctx.runtime().initial_process() {
                Ok(process) => process,
                Err(e) => {
                    return Report::spanned(None, ctx.span, e);
                }
            };
            self.game = sim::Game::Running(Box::new(sim::GameState::new(process)));
        }
        match &mut self.game {
            sim::Game::Crashed(_) => {
                self.game = sim::Game::PreviousCrash;
                Report::error(None, sim_warning!(ctx.span, PreviousCrash))
            }
            sim::Game::PreviousCrash | sim::Game::PreviousClosed => Report::new(None),
            sim::Game::Closed => {
                self.game = sim::Game::PreviousClosed;
                Report::error(None, sim_warning!(ctx.span, PreviousClosed))
            }
            sim::Game::Running(game) => {
                let mut errors = vec![];
                let output = f(game.as_mut(), ctx.span, &mut errors);
                Report::with_errors(Some(output), errors)
            }
            sim::Game::Uninit => unreachable!(),
        }
    }
}

/// This is to workaround partial borrows when running a step
#[doc(hidden)]
#[derive(Clone, Default)]
pub enum TakeGame {
    #[default]
    Uninit,
    Running(Box<sim::GameState>),
    Crashed,
    PreviousCrash,
    Closed,
    PreviousClosed,
}

impl sim::Game {
    /// Take out the game state if it's running
    ///
    /// If `uninit_if_not_running` is true, it will return Uninit
    /// if the game is not running for any reason
    pub fn take_for_execute(&mut self, uninit_if_not_running: bool) -> TakeGame {
        macro_rules! init_if_can_or {
            ($x:ident) => {
                if uninit_if_not_running {
                    TakeGame::Uninit
                } else {
                    TakeGame::$x
                }
            };
        }
        match self {
            sim::Game::Uninit => TakeGame::Uninit,
            sim::Game::Crashed(_) => init_if_can_or!(Crashed),
            sim::Game::PreviousCrash => init_if_can_or!(PreviousCrash),
            sim::Game::Closed => init_if_can_or!(Closed),
            sim::Game::PreviousClosed => init_if_can_or!(PreviousClosed),
            sim::Game::Running(_) => match std::mem::take(self) {
                sim::Game::Running(game) => TakeGame::Running(game),
                _ => unreachable!(),
            },
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
        F: FnOnce(Context<&mut Cpu2>, &mut sim::GameSystems) -> Result<(), processor::Error>,
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
            f(ctx, &mut state.systems)
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
        F: FnOnce(
            Context<&mut Cpu2>,
            &mut sim::GameSystems,
            &mut Vec<ErrorReport>,
        ) -> Result<(), processor::Error>,
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
            f(ctx, &mut state.systems, &mut errors)
        })?;
        Ok(Report::with_errors(state, errors))
    }
}

impl<'a, 'b> Context<&mut Cpu2<'a, 'b>> {
    pub fn cpu(&mut self) -> &mut Cpu2<'a, 'b> {
        self.inner
    }
}

pub trait IntoGameReport {
    fn into_report(self, span: Span) -> Report<sim::Game>;
}

impl IntoGameReport for Result<sim::GameState, CrashReport> {
    fn into_report(self, span: Span) -> Report<sim::Game> {
        match self {
            Ok(game_state) => Report::new(sim::Game::Running(Box::new(game_state))),
            Err(crash_report) => {
                Report::error(sim::Game::Crashed(crash_report), sim_error!(span, Crash))
            }
        }
    }
}

impl IntoGameReport for Result<Report<sim::GameState>, CrashReport> {
    fn into_report(self, span: Span) -> Report<sim::Game> {
        match self {
            Ok(report) => report.map(|x| sim::Game::Running(Box::new(x))),
            Err(crash_report) => {
                Report::error(sim::Game::Crashed(crash_report), sim_error!(span, Crash))
            }
        }
    }
}

impl IntoGameReport for Report<sim::Game> {
    fn into_report(self, _: Span) -> Report<sim::Game> {
        self
    }
}
