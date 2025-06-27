use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use skybook_parser::ParseOutput;

use crate::error::{ErrorReport, MaybeAborted};
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
        parsed: Arc<ParseOutput>,
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
        parsed: Arc<ParseOutput>,
        runtime: &sim::Runtime,
        mut notify_fn: F,
    ) -> MaybeAborted<sim::RunOutput>
    where
        F: FnMut(usize, &sim::RunOutput) -> TFuture,
        TFuture: std::future::Future,
    {
        self.output.states.reserve(parsed.steps.len());

        let mut state = sim::State::default();
        let mut commands = Vec::with_capacity(parsed.steps.len());
        let mut ctx = sim::Context::new(self.handle, runtime);

        for i in 0..parsed.steps.len() {
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
                            if ctx.is_aborted() {
                                log::warn!("the run is aborted, so the error is ignored");
                                return MaybeAborted::Aborted;
                            }
                            self.output
                                .errors
                                .push(ErrorReport::error(ctx.span, crate::Error::Executor));
                            return MaybeAborted::Ok(self.output);
                        }
                        Ok(report) => report,
                    };

                    // update the cache
                    runtime.set_cache(&commands, &report);

                    // check if the run is aborted
                    // note we don't check if there is a cache hit -
                    // which is really fast anyway
                    if ctx.is_aborted() {
                        return MaybeAborted::Aborted;
                    }

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
    pub fn into_raw(s: Arc<Self>) -> *const Self {
        Arc::into_raw(s)
    }
    /// Convert the handle from a raw pointer back into rust object.
    ///
    /// The pointer must be one that's previously [`into_raw`](Self::into_raw)-ed
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn from_raw(ptr: *const Self) -> Arc<Self> {
        if ptr.is_null() {
            // make sure it's safe
            return Arc::new(Self::new());
        }
        unsafe { Arc::from_raw(ptr) }
    }
}
