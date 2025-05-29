pub mod arithmetic_utils;
pub mod conditional_checker;
pub mod instruction_registry;
pub mod instructions;

pub mod instruction_parse;

use crate::memory::Memory;
use crate::memory::Proxies;
use crate::Core;
use anyhow::Result;
use instruction_registry::ExecutableInstruction;
use instruction_registry::{AuxiliaryOperation, RegisterType};
use std::collections::HashMap;
use std::panic::UnwindSafe;
use std::sync::Arc;
use std::sync::Mutex;

pub type RegIndex = u32;

type StubCondFunction = dyn Fn(&Processor) -> Result<bool> + Send +UnwindSafe+ 'static;
type StubFunction = dyn Fn(&mut Core) -> Result<()> + Send + UnwindSafe + 'static;
pub struct Stub {
    pub condition: Option<Box<StubCondFunction>>,
    pub func: Box<StubFunction>,
}
impl Stub {
    pub fn simple(func: Box<StubFunction>) -> Self {
        Stub {
            condition: None,
            func,
        }
    }

    pub fn skip() -> Self {
        Stub {
            condition: None,
            func: Box::new(|_| Ok(())),
        }
    }

    pub fn run_and_ret(func: Box<StubFunction>) -> Self {
        Stub {
            condition: None,
            func: Box::new(move |core| {
                let result = func(core);
                core.ret();
                result
            }),
        }
    }

    pub fn ret() -> Self {
        Stub {
            condition: None,
            func: Box::new(move |core| {
                core.ret();
                Ok(())
            }),
        }
    }
}

type InstructionBlock = Vec<Option<Box<dyn ExecutableInstruction>>>;


impl Processor {

    pub fn read_arg(&self, i: RegIndex) -> i64 {
        let v = self.read_reg(&RegisterType::XReg(i));
        match v {
            RegisterValue::XReg(val) => val,
            _ => 0, // Should never get here
        }
    }

    pub fn write_arg(&mut self, i: RegIndex, val: u64) {
        // We know this will not result in an error
        self.write_gen_reg(&RegisterType::XReg(i), val as i64)
            .unwrap();
    }



    /// Change the processor pc, currently unchecked
    pub fn set_pc(&mut self, new_pc: u64) {
        self.pc = new_pc;
    }

    pub fn check_pc(&self, main_offset: u32) -> Option<&Arc<Mutex<Stub>>> {
        self.stub_functions.get(&(self.pc - (main_offset as u64)))
    }

    pub fn register_stub_function(&mut self, addr: u64, stub: Stub) {
        // TODO: hardcode the program start now... need architecture change
        self.stub_functions
            .insert(addr + 0x1234500000, Arc::new(Mutex::new(stub)));
    }
}
