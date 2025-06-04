use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use teleparse::Span;
use skybook_parser::ParseOutput;

use crate::sim;
use crate::error::{ErrorReport, MaybeAborted};

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
        mut self,
        parsed: Arc<ParseOutput>,
        runtime: &sim::Runtime
    ) -> MaybeAborted<sim::RunOutput> {
        self.output.states.reserve(parsed.steps.len());

        let mut state = sim::State::default();

        for i in 0..parsed.steps.len() {
            let step = &parsed.steps[i];
            let span_end = parsed.steps.get(i + 1).map(|step| step.pos).unwrap_or(parsed.script_len);
            let span = Span::new(step.pos, span_end);
            let report = match state.execute_step(span, step, runtime).await {
                Err(e) => {
                    log::error!("failed to execute step {}: {}", i, e);
                    if self.handle.is_aborted() {
                        log::warn!("the run is aborted, so the error is ignored");
                        return MaybeAborted::Aborted;
                    }
                    self.output.errors.push(ErrorReport::error(&span, crate::Error::Executor));
                    return MaybeAborted::Ok(self.output);
                }
                Ok(report) => report
            };
            self.output.states.push(report.value.clone());
            self.output.errors.extend(report.errors);
            if self.handle.is_aborted() {
                return MaybeAborted::Aborted;
            }
            state = report.value;
        }
        MaybeAborted::Ok(self.output)
    }
}

/// Handle for a running simulation task.
///
/// See [`Run`] for more information.
#[repr(transparent)]
pub struct RunHandle {
    is_aborted: AtomicBool,
}

impl RunHandle {
    pub fn new() -> Self {
        Self {
            is_aborted: AtomicBool::new(false),
        }
    }
    pub fn is_aborted(&self) -> bool {
        self.is_aborted.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn abort(&self) {
        self.is_aborted
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn into_raw(s: Arc<Self>) -> *const Self {
        return Arc::into_raw(s);
    }
    pub fn from_raw(ptr: *const Self) -> Arc<Self> {
        if ptr.is_null() {
            // make sure it's safe
            return Arc::new(Self::new());
        }
        return unsafe { Arc::from_raw(ptr) };
    }
}
