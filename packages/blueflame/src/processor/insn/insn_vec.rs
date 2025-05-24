use crate::processor::{self as self_, crate_};

use std::ops::ControlFlow;

use crate_::env::enabled;
use self_::{Cpu0, Error, Execute, Process, BLOCK_ITERATION_LIMIT};
use self_::insn::{Core, instruction_parse};
use self_::insn::instruction_registry::ExecutableInstruction;

#[derive(Default)]
pub struct InsnVec {
    // TODO: this will be purely using decoder in the future
    // insns: Vec<disarm64::decoder::Opcode>,
    //
    insns: Vec<Box<dyn ExecutableInstruction>>,
}

impl InsnVec {
    pub fn new() -> Self {
        Self::default()
    }

    /// Disassemble the instruction and store it in the list.
    ///
    /// Returns Break if we should stop disassembling further instructions,
    /// either because there is a jump or an error occurred.
    pub fn disassemble(&mut self, insn: u32) -> ControlFlow<Option<Error>> {
        // returns true if we can keep disassembling the next instruction
        // (i.e. no jump or error)
        let insn = match instruction_parse::byte_to_inst(insn) {
            Err(e) => return ControlFlow::Break(Some(e)),
            Ok(x) => x
        };
        let should_continue = !insn.is_jump();
        self.insns.push(insn);
        if should_continue {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(None)
        }
    }

    pub fn byte_size(&self) -> u32 {
        self.insns.len() as u32 * 4
    }
}

impl Execute for InsnVec {
    fn execute_from(&self, cpu: &mut Cpu0, proc: &mut Process, step: u32) -> Result<(), Error> {
        let limit = if enabled!("limited-block-iteration") {
            BLOCK_ITERATION_LIMIT
        } else {
            usize::MAX
        };
        for (i, x) in self.insns.iter().skip(step as usize).enumerate() {
            if i >= limit {
                return Err(Error::BlockIterationLimitReached);
            }
            x.exec_on(&mut Core {
                cpu,
                proc,
            })?;
        }
        Ok(())
    }
}
