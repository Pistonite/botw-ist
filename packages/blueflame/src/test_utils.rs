use std::sync::Arc;

use crate::game::Proxies;
use crate::memory::{Memory, Region, RegionType, SimpleHeap};
use crate::processor::{Process, HookProvider, Execute};
use crate::env::{Environment, GameVer, DlcVer};

pub fn init_log() {
    let mut builder = colog::default_builder();
    builder.init();
}

impl Memory {
    pub fn new_for_test() -> Self {
        let p = Arc::new(Region::new_rw(RegionType::Program, 0x4000, 0));
        let s = Arc::new(Region::new_rw(RegionType::Stack, 0, 0x4000));
        let h = Arc::new(SimpleHeap::new(0x4000, 0, 0));
        Self::new(Environment::new_for_test(), p, s, h)
    }
}

struct EmptyHookProvider;
impl HookProvider for EmptyHookProvider {
    fn fetch(&self, main_offset: u32, env: Environment) -> Result<Option<(Box<dyn Execute>, u32)>, crate::processor::Error> {
        Ok(None)
    }
}

impl Process {
    pub fn new_for_test() -> Self {
        let mem = Arc::new(Memory::new_for_test());
        let proxies = Arc::new(Proxies::default());
        Self::new(mem, proxies, Arc::new(EmptyHookProvider))
    }
}

impl Environment {
    pub fn new_for_test() -> Self {
        Self::new(GameVer::X150, DlcVer::None)
    }
}
