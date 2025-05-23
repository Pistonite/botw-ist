use crate::processor::{self as self_, crate_};

use self_::{Cpu0, Error, Execute, Process};

pub struct InsnVec {
    insns: Vec<disarm64::Opcode>,
}

impl InsnVec {
    pub fn new() -> Self {
        Self {
            insns: Vec::new(),
        }
    }

    pub fn disassemble(&mut self, insn: u32) -> bool {
        // returns true if we can keep disassembling the next instruction
        // (i.e. no jump or error)
        todo!()
    }

    pub fn byte_size(&self) -> u32 {
        return self.insns.len() as u32 * 4;
    }
}

impl Execute for InsnVec {
    fn execute_from(&self, cpu: &mut Cpu0, proc: &mut Process, step: u32) -> Result<(), Error> {
        todo!()
    }
}

// TODO --cleanup: remove this
pub struct Core<'a, 'b> {
    cpu: &'a mut Cpu0,
    proc: &'b mut Process,
}


mod arithmetic_utils;
mod instruction_parse;
mod instruction_registry;
mod instructions;
