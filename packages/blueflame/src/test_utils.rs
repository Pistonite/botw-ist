use std::sync::Arc;

use crate::memory::{Memory, Region, RegionType, SimpleHeap, Proxies};
use crate::processor::Process;
use crate::env::{Environment, GameVer, DlcVer};

pub use blueflame_proc_macros::paste_insn;

pub fn init_log() {
    let mut builder = colog::default_builder();
    builder.init();
}

impl Memory {
    pub fn new_for_test() -> Self {
        let p = Arc::new(Region::new_rw(RegionType::Program, 0x4000, 0));
        let s = Arc::new(Region::new_rw(RegionType::Stack, 0, 0x4000));
        let h = Arc::new(SimpleHeap::new(0x4000, 0, 0));
        Self::new(p, s, h, None, None)
    }
}

impl Process {
    pub fn new_for_test() -> Self {
        let mem = Arc::new(Memory::new_for_test());
        let proxies = Arc::new(Proxies::default());
        Self::new(Environment::new_for_test(), mem, proxies)
    }
}

impl Environment {
    pub fn new_for_test() -> Self {
        Self::new(GameVer::X150, DlcVer::None)
    }
}
