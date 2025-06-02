use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::sim;

pub struct Run {
    handle: Arc<RunHandle>,
    states: Vec<Arc<sim::State>>,
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
