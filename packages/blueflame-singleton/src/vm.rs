use crate::Bytecode;

use blueflame_utils::{DataType, ProxyType};

/// Trait implemented by external consumers to visit the singleton creation process
pub trait VirtualMachine {
    type Error;

    /// Simulate a branch-and-link to the target address
    /// (relative to start of the main module).
    /// This is always the first function called
    fn enter(&mut self, target: u32) -> Result<(), Self::Error>;

    /// Set a general purpose register to a 64-bit value
    ///
    /// The register number is 0-30 for X0 to X30,
    /// and 32-63 for S0 to S31
    fn set_reg(&mut self, reg: u8, value: u64) -> Result<(), Self::Error>;

    /// Copy the value of one register to another
    ///
    /// The register number is 0-30 for X0 to X30, and 32-63 for S0 to S31,
    /// when copying between X and S, the bits should be copied exactly
    fn copy_reg(&mut self, from: u8, to: u8) -> Result<(), Self::Error>;

    /// Execute the program until when the next instruction
    /// is at `target`. Target is relative to the start of the main module
    fn execute_until(&mut self, target: u32) -> Result<(), Self::Error>;

    /// Set PC relative to the start of the main module without doing anything else
    /// (i.e. the next instruction to execute is at `pc`)
    fn jump(&mut self, target: u32) -> Result<(), Self::Error>;

    /// Allocate `bytes` bytes of memory from heap, and put the address in X0
    fn allocate_memory(&mut self, bytes: u32) -> Result<(), Self::Error>;

    /// Allocate a proxy object of the type, and put the address in X0
    fn allocate_proxy(&mut self, proxy: ProxyType) -> Result<(), Self::Error>;

    /// Allocate raw data in heap, and put the address in X0
    fn allocate_data(&mut self, data: DataType) -> Result<(), Self::Error>;

    /// Simulate allocating space for the singleton. Put the address in X0
    ///
    /// No allocation is really happening here, since the singleton addresses
    /// are pre-determined in BlueFlame and the spaces are already preserved.
    /// The executor only needs to convert the relative start to the physical address
    fn allocate_singleton(&mut self, rel_start: u32, _size: u32) -> Result<(), Self::Error> {
        self.get_singleton(0, rel_start)
    }

    /// Put the singleton address in the register
    fn get_singleton(&mut self, reg: u8, rel_start: u32) -> Result<(), Self::Error>;

    /// Execute until jumping out of function jumped into from `Enter`
    ///
    /// This is not guaranteed to be called for the creation process. However,
    /// the next function called after this is always `finish`
    fn execute_to_complete(&mut self) -> Result<(), Self::Error>;

    /// Indicate that the singleton creation process is complete
    ///
    /// This is always the last function called for the singleton creation process
    fn finish(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Provided method to run a bytecode program
    fn execute_bytecode_program(&mut self, program: &[Bytecode], rel_start: u32, size: u32) -> Result<(), Self::Error> {
        if program.is_empty() {
            return Ok(());
        }
        let mut prev_lo_value = 0u32;
        for bytecode in program {
            match *bytecode {
                Bytecode::Enter(target) => {
                    self.enter(target)?
                },
                Bytecode::SetRegHi(reg, value) => {
                    self.set_reg(reg, ((value as u64) << 32) | prev_lo_value as u64)?;
                    prev_lo_value = 0;
                }
                Bytecode::SetRegLo(reg, value) => {
                    self.set_reg(reg, value as u64)?
                }
                Bytecode::RegLoNextHi(value) => {
                    prev_lo_value = value;
                },
                Bytecode::CopyReg(from, to) => {
                    self.copy_reg(from, to)?
                },
                Bytecode::ExecuteUntil(target) => {
                    self.execute_until(target)?
                }
                Bytecode::ExecuteUntilThenSkipOne(target) => {
                    self.execute_until(target)?;
                    self.jump(target + 4)?
                }
                Bytecode::ExecuteUntilThenAllocSingletonSkipOne(target) => {
                    self.execute_until(target)?;
                    self.allocate_singleton(rel_start, size)?;
                    self.jump(target + 4)?
                }
                Bytecode::Jump(target) => {
                    self.jump(target)?
                },
                Bytecode::JumpExecute(target) => {
                 self.jump(target)?;
                    self.execute_until(target + 4)?
                },
                Bytecode::Allocate(bytes) => {
                    self.allocate_memory(bytes)?
                },
                Bytecode::AllocateProxy(proxy_type) => {
                    self.allocate_proxy(proxy_type)?
                },
                Bytecode::AllocateData(data_type) => {
                    self.allocate_data(data_type)?
                }
                Bytecode::AllocateSingleton => {
                    self.allocate_singleton(rel_start, size)?
                }
                Bytecode::GetSingleton(reg) => {
                    self.get_singleton(reg, rel_start)?
                }
                Bytecode::ExecuteToComplete => {
                    self.execute_to_complete()?
                }
            }
        }
        self.finish()
    }

}
