use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use skybook_parser::ParseOutput;

use crate::error::MaybeAborted;
use crate::sim;

pub struct Run {
    /// Handle for the running task
    handle: Arc<RunHandle>,
    /// Data produced by the run
    output: sim::RunOutput,
}

impl Run {
    pub fn new(handle: Arc<RunHandle>) -> Self {
        Run {
            handle,
            output: Default::default(),
        }
    }

    /// Execute the parsed simulation script
    ///
    /// All errors that happened, including internal (e.g. game crash) or
    /// external are collected in the `RunOutput`.
    pub async fn run_parsed(
        self,
        parsed: &ParseOutput,
        runtime: &sim::Runtime,
    ) -> MaybeAborted<sim::RunOutput> {
        self.run_parsed_with_notify(parsed, runtime, |_, _| async {})
            .await
    }

    /// See [`run_parsed`](Self::run_parsed). In addition, `notify_fn` will be called
    /// after each step with the end byte pos of the step and the state after that step.
    ///
    /// Use this if you need to use partial output before the whole run is finished.
    /// Note that if the run is aborted, then the notification will not be sent after the run is aborted.
    ///
    /// The notification will not be sent after the last step
    pub async fn run_parsed_with_notify<TFuture, F>(
        mut self,
        parsed: &ParseOutput,
        runtime: &sim::Runtime,
        mut notify_fn: F,
    ) -> MaybeAborted<sim::RunOutput>
    where
        F: FnMut(usize, &sim::RunOutput) -> TFuture,
        TFuture: std::future::Future,
    {
        self.output.states.reserve(parsed.steps.len());

        let process = match runtime.initial_process() {
            Ok(x) => x,
            Err(e) => {
                log::error!("unexpected: fail to get initial process from runtime: {e}");
                return MaybeAborted::Aborted;
            }
        };

        let mut state = sim::State::new(process);
        let mut commands = Vec::with_capacity(parsed.steps.len());
        let mut ctx = sim::Context::new(self.handle, runtime);

        for i in 0..parsed.steps.len() {
            if ctx.is_aborted() {
                return MaybeAborted::Aborted;
            }
            let step = &parsed.steps[i];
            let pos = step.pos();
            let percentage = pos as f32 / parsed.script_len as f32 * 100.0;
            log::debug!(
                "running: byte_pos {}/{} ({:.2}%)",
                pos,
                parsed.script_len,
                percentage
            );

            // notify only if it's not the first step
            // this is because the first step may not be byte pos 0,
            // and may cause the "initial" state to be sent in notification
            // while actually what we need to send is the state after the first step
            if i > 0 {
                notify_fn(pos, &self.output).await;
            }

            commands.push(step.command.clone());

            let report = match runtime.find_cached(&commands) {
                Some(report) => report,
                None => {
                    ctx.span = step.span();

                    let report = match state.execute_step(ctx.clone(), step).await {
                        Err(e) => {
                            log::error!("failed to execute step {i}: {e}");
                            return MaybeAborted::Aborted;
                        }
                        Ok(report) => report,
                    };
                    // check if the run is aborted
                    if ctx.is_aborted() {
                        return MaybeAborted::Aborted;
                    }

                    // update the cache
                    // note we must only update the cache if the run
                    // is not aborted, since it could abort
                    // in the middle of a step (i.e. partially executed)
                    runtime.set_cache(&commands, &report);

                    report
                }
            };

            self.output.states.push(report.value.clone());
            self.output.errors.extend(report.errors);
            state = report.value;
        }

        MaybeAborted::Ok(self.output)
    }
}

/// Handle for a running simulation task.
///
/// See [`Run`] for more information.
#[derive(Default)]
#[repr(transparent)]
pub struct RunHandle {
    is_aborted: AtomicBool,
}

impl RunHandle {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_aborted(&self) -> bool {
        self.is_aborted.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn abort(&self) {
        self.is_aborted
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    /// Convert the handle into a raw pointer to interface with external code
    #[cfg(feature = "unsafe-leak")]
    pub fn leak(s: Arc<Self>) -> *const Self {
        Arc::into_raw(s)
    }
    /// Convert the handle from a raw pointer back into rust object.
    ///
    /// If add_ref is true, it will leak the pointer again, not consuming the Arc
    ///
    /// # Safety
    ///
    /// The pointer must be one previously leaked with [`leak`](Self::leak)
    #[cfg(feature = "unsafe-leak")]
    pub unsafe fn from_raw(ptr: *const Self, add_ref: bool) -> Arc<Self> {
        if ptr.is_null() {
            // make sure it's safe
            return Arc::new(Self::new());
        }
        let x = unsafe { Arc::from_raw(ptr) };
        if add_ref {
            let x2 = Arc::clone(&x);
            let x_old = Arc::into_raw(x);
            assert!(
                std::ptr::eq(ptr, x_old),
                "re-leaked pointer must be equal to prevent losing memory"
            );
            return x2;
        }
        x
    }
}
