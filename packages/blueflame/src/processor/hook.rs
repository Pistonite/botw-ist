use std::any::Any;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;

use derive_more::Constructor;

use crate::env::Environment;
use crate::processor::{Error, Execute};

pub trait HookProvider: Any + Send + Sync + UnwindSafe + RefUnwindSafe {
    /// Hook execution at PC. Return the execute function and the byte
    /// size of the hook
    #[allow(clippy::type_complexity)]
    fn fetch(&self, main_offset: u32, env: Environment) -> Result<Option<Hook>, Error>;
}

pub enum Hook {
    /// Execute extra code at the address
    Start(Box<dyn Execute>),
    /// Replace code, the second arg is the byte size of the hook
    Replace(Box<dyn Execute>, u32),
}

#[derive(Constructor)]
pub struct HookChain {
    outer: Arc<dyn HookProvider>,
    inner: Arc<dyn HookProvider>,
}

impl HookChain {
    pub fn inner(&self) -> Arc<dyn HookProvider> {
        Arc::clone(&self.inner)
    }
}

impl HookProvider for HookChain {
    fn fetch(&self, main_offset: u32, env: Environment) -> Result<Option<Hook>, Error> {
        match self.outer.fetch(main_offset, env)? {
            None => self.inner.fetch(main_offset, env),
            x => Ok(x),
        }
    }
}
