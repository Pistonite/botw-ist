use std::ops::ControlFlow;
use std::sync::Arc;

use derive_more::derive::Constructor;

use crate::env::Environment;
use crate::game::Proxies;
use crate::memory::{Memory, ProxyGuardMut, ProxyList, ProxyObject, access};
use crate::processor::insn::{HookedInsnVec, InsnVec};
use crate::processor::{Error, Execute, Hook, HookProvider};

/// The Process is the container for everything the core tracks
/// that is not in the Processor.
///
/// Cloning the process will `fork` the process, and the memory will
/// be shared (clone on write)
#[derive(Clone, Constructor)]
pub struct Process {
    /// Main memory of the game
    memory: Arc<Memory>,
    /// Proxy implementation for some game objects
    proxies: Arc<Proxies>,
    /// Hooks for this process
    hook_provider: Arc<dyn HookProvider>,
}
static_assertions::assert_impl_all!(Process: Send, Sync);

impl std::fmt::Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process").finish()
    }
}

impl Process {
    /// Get the environment configuration
    pub fn env(&self) -> Environment {
        self.memory.env()
    }

    /// Access the main memory of the process
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Access the main memory of the process for mutation
    pub fn memory_mut(&mut self) -> &mut Memory {
        Arc::make_mut(&mut self.memory)
    }

    /// Access proxies. Usually this is easier with the [`proxy`](crate::memory::proxy) macro
    pub fn proxies(&self) -> &Proxies {
        &self.proxies
    }

    /// Access proxies for mutation. Usually this is easier with the [`proxy`](crate::memory::proxy) macro
    pub fn proxies_mut<T: ProxyObject, F: FnOnce(&mut Proxies) -> &mut ProxyList<T>>(
        &mut self,
        f: F,
    ) -> ProxyGuardMut<'_, '_, T> {
        let memory = Arc::make_mut(&mut self.memory);
        let proxies = Arc::make_mut(&mut self.proxies);
        let proxy_list = f(proxies);

        proxy_list.write(memory)
    }

    /// Get the physical starting address of the main module
    pub fn main_start(&self) -> u64 {
        self.memory.main_start()
    }

    /// Get the hook provider
    pub fn hook_provider_mut(&mut self) -> &mut Arc<dyn HookProvider> {
        &mut self.hook_provider
    }

    /// Fetch a block of code for execution
    ///
    /// If `max_bytes` is `Some(n)`, then the function will
    /// make sure the block fetched is not larger than the size, even
    /// if it's a hook.
    ///
    /// If `max_bytes` is `None`, then the function will only fetch
    /// one instruction max, and will ignore the check if a hook is found
    pub fn fetch_execute_block(
        &self,
        pc: u64,
        max_bytes: Option<u32>,
    ) -> Result<(Box<dyn Execute>, u32), Error> {
        // find execute hook at this location
        let main_offset: u32 = (pc - self.main_start()) as u32;

        if main_offset == 0x976920 {
            log::debug!("fetching at 976920");
        }
        if main_offset == 0x9766d8 {
            log::debug!("fetching at 9766d8");
        }

        let (ignore_hook_size_check, max_bytes) = match max_bytes {
            Some(n) => (false, n),
            None => (true, 4), // fetch one instruction
        };
        let opt_hook_exec = match self.hook_provider.fetch(main_offset, self.memory.env())? {
            Some(Hook::Replace(hook_exec, bytes)) => {
                if !ignore_hook_size_check && bytes > max_bytes {
                    return Err(Error::TooBigHook(main_offset));
                }
                return Ok((hook_exec, bytes));
            }
            Some(Hook::Start(hook_exec)) => Some(hook_exec),
            None => None,
        };

        // if no hook, fetch the instructions from memory
        let mut reader = self.memory.read(pc, access!(execute))?;
        let mut insns = InsnVec::new();

        for _ in 0..(max_bytes / 4) {
            let insn_raw = reader.read_u32()?;

            if let ControlFlow::Break(_) = insns.disassemble(insn_raw) {
                break;
            }
        }

        let size = insns.byte_size();
        if size > max_bytes {
            log::error!(
                "instruction cache size greater than max, this is a bug. fetching at pc=0x{pc:016x}, size={size}, max_bytes={max_bytes}"
            );
            return Err(Error::TooBigHook(main_offset));
        }
        match opt_hook_exec {
            None => Ok((Box::new(insns), size)),
            Some(hook) => Ok((Box::new(HookedInsnVec::new(hook, insns)), size)),
        }
    }
}
