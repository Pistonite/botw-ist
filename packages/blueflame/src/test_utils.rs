use std::sync::Arc;

use crate::env::{DlcVer, Environment, GameVer};
use crate::game::Proxies;
use crate::memory::Memory;
use crate::processor::{Execute, HookProvider, Process};

impl Memory {
    pub fn new_for_test() -> Self {
        let env = Environment::new_for_test();
        Self::new(env, 0x4000, 0x2000, 0x4000)
    }
}

struct EmptyHookProvider;
impl HookProvider for EmptyHookProvider {
    fn fetch(
        &self,
        _: u32,
        _: Environment,
    ) -> Result<Option<(Box<dyn Execute>, u32)>, crate::processor::Error> {
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
