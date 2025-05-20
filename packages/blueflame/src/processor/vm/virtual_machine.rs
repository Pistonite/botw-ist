use crate::processor::crate_;

use crate_::env::{DataId, ProxyId};

/// Trait implemented by the processor to help navigate and execute
/// gadgets in the existing program. For example to call a function
/// that is partially stubbed
///
/// The VM work with relative addresses to the start of the main module
/// instead of the physical addresses. All functions are prefixed
/// with `v_` to indicate it's implemented for the VM
pub trait VirtualMachine {
    type Error;

    /// Simulate a branch-and-link to the target address
    /// (relative to start of the main module).
    fn v_enter(&mut self, target: u32) -> Result<(), Self::Error>;

    // TODO --cleanup: reg type use enum

    /// Set a general purpose register to a 64-bit value
    ///
    /// The register number is 0-30 for X0 to X30,
    /// and 32-63 for S0 to S31
    fn v_reg_set(&mut self, reg: u8, value: u64) -> Result<(), Self::Error>;

    /// Copy the value of one register to another
    ///
    /// The register number is 0-30 for X0 to X30, and 32-63 for S0 to S31,
    /// when copying between X and S, the bits should be copied exactly
    fn v_reg_copy(&mut self, from: u8, to: u8) -> Result<(), Self::Error>;

    /// Execute the program until when the next instruction
    /// is at `target`. Target is relative to the start of the main module
    fn v_execute_until(&mut self, target: u32) -> Result<(), Self::Error>;

    /// Equivalent to `v_execute_until(X); v_singleton_alloc(A, B); v_jump(X + 4)`
    fn v_execute_until_then_single_alloc_skip_one(&mut self, target: u32, rel_start: u32, size: u32) -> Result<(), Self::Error> {
        self.v_execute_until(target)?;
        self.v_singleton_alloc(rel_start, size)?;
        self.v_jump(target + 4)
    }

    /// Equivalent to `v_execute_until(X); v_jump(X + 4)`
    fn v_execute_until_then_skip_one(&mut self, target: u32) -> Result<(), Self::Error> {
        self.v_execute_until(target)?;
        self.v_jump(target + 4)
    }

    /// Set PC relative to the start of the main module without doing anything else
    /// (i.e. the next instruction to execute is at `pc`)
    fn v_jump(&mut self, target: u32) -> Result<(), Self::Error>;

    /// Equivalent to `v_jump(X); v_execute_until(X + 4)`
    fn v_jump_execute(&mut self, target: u32) -> Result<(), Self::Error> {
        self.v_jump(target)?;
        self.v_execute_until(target + 4)
    }

    /// Allocate `bytes` bytes of memory from heap, and put the address in X0
    fn v_mem_alloc(&mut self, bytes: u32) -> Result<(), Self::Error>;

    /// Allocate a proxy object of the type, and put the address in X0
    fn v_proxy_alloc(&mut self, proxy: ProxyId) -> Result<(), Self::Error>;

    /// Allocate raw data in heap, and put the address in X0
    fn v_data_alloc(&mut self, data: DataId) -> Result<(), Self::Error>;

    /// Simulate allocating space for the singleton. Put the address in X0
    fn v_singleton_alloc(&mut self, rel_start: u32, _size: u32) -> Result<(), Self::Error> {
        // No allocation is really happening here, since the singleton addresses
        // are pre-determined in BlueFlame and the spaces are already preserved.
        // The executor only needs to convert the relative start to the physical address
        self.v_singleton_get(0, rel_start)
    }

    /// Put the singleton address in the register
    fn v_singleton_get(&mut self, reg: u8, rel_start: u32) -> Result<(), Self::Error>;

    /// Execute until jumping out of function jumped into from `Enter`
    ///
    /// This is not guaranteed to be called for the creation process. However,
    /// the next function called after this is always `finish`
    fn v_execute_to_complete(&mut self) -> Result<(), Self::Error>;

    // TODO --cleanup: do we need this?
    // /// Indicate that the singleton creation process is complete
    // ///
    // /// This is always the last function called for the singleton creation process
    // fn finish(&mut self) -> Result<(), Self::Error> {
    //     Ok(())
    // }

    // TODO --cleanup
    // /// Provided method to run a bytecode program
    // fn execute_bytecode_program(&mut self, program: &[Bytecode], singleton_heap_rel_start: u32, singleton_size: u32) -> Result<(), Self::Error> {
    //     if program.is_empty() {
    //         return Ok(());
    //     }
    //     let mut prev_lo_value = 0u32;
    //     for bytecode in program {
    //         log::debug!("Executing bytecode: {:?}", bytecode);
    //         match *bytecode {
    //             Bytecode::Enter(target) => {
    //                 self.enter(target)?
    //             },
    //             Bytecode::SetRegHi(reg, value) => {
    //                 self.set_reg(reg, ((value as u64) << 32) | prev_lo_value as u64)?;
    //                 prev_lo_value = 0;
    //             }
    //             Bytecode::SetRegLo(reg, value) => {
    //                 self.set_reg(reg, value as u64)?
    //             }
    //             Bytecode::RegLoNextHi(value) => {
    //                 prev_lo_value = value;
    //             },
    //             Bytecode::CopyReg(from, to) => {
    //                 self.copy_reg(from, to)?
    //             },
    //             Bytecode::ExecuteUntil(target) => {
    //                 self.execute_until(target)?
    //             }
    //             Bytecode::ExecuteUntilThenSkipOne(target) => {
    //                 self.execute_until(target)?;
    //                 self.jump(target + 4)?
    //             }
    //             // TODO --cleanup: too big bytecode
    //             // Bytecode::ExecuteUntilThenAllocSingletonSkipOne(target) => {
    //             //     self.execute_until(target)?;
    //             //     self.allocate_singleton(singleton_heap_rel_start, singleton_size)?;
    //             //     self.jump(target + 4)?
    //             // }
    //             Bytecode::Jump(target) => {
    //                 self.jump(target)?
    //             },
    //             Bytecode::JumpExecute(target) => {
    //              self.jump(target)?;
    //                 self.execute_until(target + 4)?
    //             },
    //             Bytecode::Allocate(bytes) => {
    //                 self.allocate_memory(bytes)?
    //             },
    //             Bytecode::AllocateProxy(proxy_type) => {
    //                 self.allocate_proxy(proxy_type)?
    //             },
    //             Bytecode::AllocateData(data_type) => {
    //                 self.allocate_data(data_type)?
    //             }
    //             Bytecode::AllocateSingleton => {
    //                 self.allocate_singleton(singleton_heap_rel_start, singleton_size)?
    //             }
    //             Bytecode::GetSingleton(reg) => {
    //                 self.get_singleton(reg, singleton_heap_rel_start)?
    //             }
    //             Bytecode::ExecuteToComplete => {
    //                 self.execute_to_complete()?
    //             }
    //         }
    //     }
    //     self.finish()
    // }

}
