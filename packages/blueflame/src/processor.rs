use std::collections::HashMap;

use crate::{memory::Memory, Core};

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
    pub fn attach<'p, 'm>(&'p mut self, mem: &'m mut Memory) -> Core<'p, 'm> {
        Core {
            cpu: self,
            mem,
        }
    }
}
