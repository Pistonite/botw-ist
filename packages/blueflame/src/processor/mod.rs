
use std::collections::HashMap;

use crate::{memory::{Memory, Proxies}, Core};

pub struct Processor {
    // TODO: registers
    pub stub_functions: HashMap<u64, Box<dyn FnMut(&Core) -> Result<(), ()>>>,
}

impl Default for Processor {
    fn default() -> Self {
        todo!()
    }
}

impl Processor {
    /// Attach the processor to a memory instance
    pub fn attach<'p, 'm, 'x>(&'p mut self, mem: &'m mut Memory, proxies: &'x mut Proxies) -> Core<'p, 'm, 'x> {
        Core {
            cpu: self,
            mem,
            proxies,
        }
    }
}
