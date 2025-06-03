use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use skybook_parser::ParseOutput;

use crate::{sim, ErrorReport, MaybeAborted};

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
    pub async fn run_parsed(
        mut self,
        parsed: Arc<ParseOutput>,
        runtime: &sim::Runtime
    ) -> MaybeAborted<sim::RunOutput> {
        self.output.states.reserve(parsed.steps.len());

        for step in &parsed.steps {
            let state = self.output.states.last().cloned().unwrap_or_default();
        }
        // Here we would run the simulation using the parsed output
        // and the handle to check for abortion requests.
        // For now, we return an empty RunOutput.
        MaybeAborted::Ok(RunOutput { states: vec![] })
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
