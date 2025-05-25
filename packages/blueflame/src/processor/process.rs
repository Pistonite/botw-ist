use crate::processor::{self as self_, crate_};

use std::collections::HashMap;
use std::panic::UnwindSafe;
use std::sync::Arc;
use std::ops::ControlFlow;

use derive_more::derive::Constructor;

use crate_::env::{Environment, enabled, GameVer};
use crate_::memory::{Memory, access};
use crate_::game::Proxies;
use self_::{Execute, Error};
use self_::insn::InsnVec;

/// The Process is the container for everything the core tracks
/// that is not in the Processor.
#[derive(Constructor)]
pub struct Process {
    /// Main memory of the game
    memory: Arc<Memory>,
    /// Proxy implementation for some game objects
    proxies: Arc<Proxies>,
    /// Hooks for this process
    hook: Arc<dyn HookProvider>,
}

impl std::fmt::Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process").finish()
    }
}

impl Process {
    /// Get the game version
    pub fn game_ver(&self) -> GameVer {
        self.memory.env().game_ver
    }

    /// Access the main memory of the process
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Access the main memory of the process for mutation
    pub fn memory_mut(&mut self) -> &mut Memory {
        Arc::make_mut(&mut self.memory)
    }

    pub fn proxies(&self) -> &Proxies {
        &self.proxies
    }

    pub fn proxies_mut(&mut self) -> &mut Proxies {
        Arc::make_mut(&mut self.proxies)
    }

    /// Get the physical starting address of the main module
    pub fn main_start(&self) -> u64 {
        self.memory.main_start()
    }

    /// Fetch a block of code for execution
    pub fn fetch_execute_block(&self, pc: u64, max_bytes: u32) -> Result<(Box<dyn Execute>, u32), Error> {
        // find execute hook at this location
        let main_offset: u32 = (pc - self.main_start()) as u32;
        if let Some((x, bytes)) = self.hook.fetch(main_offset, self.memory.env())? {
            if bytes > max_bytes {
                return Err(Error::TooBigHook(main_offset))
            }
            return Ok((x, bytes))
        }

        // if no hook, fetch the instructions from memory
        let mut reader = self.memory.read(pc, access!(execute))?;
        let mut insns = InsnVec::new();

        for _ in 0..(max_bytes/4) {
            let insn_raw = reader.read_u32()?;

            if let ControlFlow::Break(_) = insns.disassemble(insn_raw) {
                break;
            }
        }

        let size = insns.byte_size();
        Ok((Box::new(insns), size))
    }


}

pub trait HookProvider: Send + Sync + UnwindSafe {
    /// Hook execution at PC. Return the execute function and the byte
    /// size of the hook
    fn fetch(&self, main_offset: u32, env: Environment) -> Result<Option<(Box<dyn Execute>, u32)>, Error>;
}
