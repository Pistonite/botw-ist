use std::sync::mpsc;

use blueflame::processor::Cpu1;

mod error;
pub use error::Error;
pub mod executor;
mod thread;
pub use thread::*;

pub type JobSender = mpsc::Sender<Job>;
pub type Job = Box<dyn FnOnce(&mut Cpu1) + Send + 'static>;

/// Trait for platform-specific thread implementation to provide
/// method to spawn and join threads
pub trait Spawn {
    /// The join handle type for the thread implementation
    type Joiner: Join;
    /// Spawn a thread with the slot (id)
    fn spawn(&mut self, slot: usize) -> Result<(Self::Joiner, JobSender), Error>;
}

/// Trait for platform-specific thread join handles to implement
pub trait Join {
    /// Join the thread
    fn join(self) -> Result<(), Error>;
}

/// Implementation of executor pool using std threads
#[cfg(not(feature = "wasm"))]
mod impl_native;
#[cfg(not(feature = "wasm"))]
use impl_native as __impl;
/// Implementation of executor pool using wasm-bindgen-spawn
#[cfg(feature = "wasm")]
mod impl_wasm;
#[cfg(feature = "wasm")]
use impl_wasm as __impl;

pub type Spawner = __impl::Spawner;

pub type Executor = executor::ExecutorImpl<Spawner>;
static_assertions::assert_impl_all!(Executor: Send , Sync);
