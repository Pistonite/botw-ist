use std::ops::ControlFlow;

use disarm64::arm64::InsnOpcode;
use disarm64::decoder::Opcode;

use crate::env::enabled;
use crate::processor::{
    insn::instruction_parse::ExecutableInstruction,
    insn::{instruction_parse, op, Core},
    {Cpu0, Error, Execute, Process, BLOCK_ITERATION_LIMIT},
};

#[derive(Default)]
pub struct InsnVec {
    insns: Vec<Entry>,
}

enum Entry {
    CannotDecode(u32),
    LegacyParse(Opcode, Option<Box<dyn ExecutableInstruction>>),
}
// ensure the size doesn't unexpectedly change
#[cfg(not(target_arch = "wasm32"))] // wasm32 has different usize
static_assertions::const_assert_eq!(std::mem::size_of::<Entry>(), 0x20);

impl InsnVec {
    pub fn new() -> Self {
        Self::default()
    }

    /// Disassemble the instruction and store it in the list.
    ///
    /// Returns Break if we should stop disassembling further instructions,
    /// either because there is a jump or an error occurred.
    pub fn disassemble(&mut self, bits: u32) -> ControlFlow<()> {
        let Some(opcode) = disarm64::decoder::decode(bits) else {
            log::warn!("failed to decode instruction 0x{bits:08x}");
            self.insns.push(Entry::CannotDecode(bits));
            return ControlFlow::Break(());
        };

        let should_continue = !op::is_branch(opcode);

        // decode using the legacy (string-based) decoder
        // and cache the result
        let legacy_insn = instruction_parse::opcode_to_inst(opcode);
        self.insns.push(Entry::LegacyParse(opcode, legacy_insn));
        if should_continue {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
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
            let (opcode, legacy_insn) = match x {
                Entry::CannotDecode(bits) => return Err(Error::BadInstruction(*bits)),
                Entry::LegacyParse(opcode, legacy_insn) => (opcode, legacy_insn.as_ref()),
            };

            match op::execute_opcode(cpu, proc, *opcode) {
                op::ExecResult::Handled => {
                    cpu.inc_pc();
                    continue;
                }
                op::ExecResult::Error(e) => {
                    return Err(e);
                }
                op::ExecResult::NotImplemented => {
                    // try to execute the legacy instruction
                }
            };

            match legacy_insn {
                None => {
                    log::error!(
                        "could not execute instruction, legacy parse failed: {}",
                        opcode.to_string()
                    );
                    return Err(Error::BadInstruction(opcode.bits()));
                }
                Some(x) => {
                    x.exec_on(&mut Core { cpu, proc })?;
                    cpu.inc_pc();
                }
            }
        }
        Ok(())
    }
}
