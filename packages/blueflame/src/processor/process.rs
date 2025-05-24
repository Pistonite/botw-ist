use crate::processor::{self as self_, crate_};

use std::collections::HashMap;
use std::sync::Arc;

use derive_more::derive::Constructor;

use crate_::env::{Environment, enabled, GameVer};
use crate_::memory::{Memory, Proxies, access};
use self_::{Execute, Error};
use self_::insn::InsnVec;

/// The Process is the container for everything the core tracks
/// that is not in the Processor.
#[derive(Debug, Constructor)]
pub struct Process {
    /// Game environment
    env: Environment,
    /// Main memory of the game
    memory: Arc<Memory>,
    /// Proxy implementation for some game objects
    proxies: Arc<Proxies>,
}

impl Process {
    pub const fn game_ver(&self) -> GameVer {
        self.env.game_ver
    }
    // /// Prefetch for execution at the given PC
    // pub fn prefetch(&self, pc: u64) -> Option<&dyn Execute> {
    //     self.exec_cache.get(&pc).map(|code| code.as_ref())
    // }
    //
    // pub fn fetch(&mut self, pc: u64) -> Result<(), Error> {
    //     let hook = if enabled!("proc-strict-hook") {
    //     }
    //
    //     // read instructions starting at PC
    //     let mut reader = self.memory.read(pc, access!(execute))?;
    //     let mut insns = InsnVec::new();
    //     // disassemble the block
    //     while insns.disassemble(reader.read_u32()?) {}
    //     // save to cache
    //     self.exec_cache.insert(pc, Box::new(insns));
    //     Ok(())
    // }

    // pub fn register_hook<F: Execute>(
    //     &mut self, 
    //     main_offset: u32, 
    //     size: u32, 
    //     hook: F
    // ) {
    //     let pc = self.main_start() + main_offset as u64;
    //     let lower_bound = self.hooks.binary_search_by_key(&pc, |h| h.start);
    //     self.hooks.push(Hook {
    //         start: pc,
    //         size,
    //         hook: Box::new(hook),
    //     });
    // }

    pub fn main_start(&self) -> u64 {
        self.memory.program_start() + self.env.main_offset() as u64
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

}
