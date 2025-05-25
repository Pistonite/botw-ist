use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error, reg};

pub    fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
        let label_offset = parse::get_label_val(args)?;
        Some(Box::new(BlInstruction { label_offset }))
    }

#[derive(Clone)]
pub struct BlInstruction {
    label_offset: u64,
}

impl ExecutableInstruction for BlInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let pc = core.cpu.pc;
        // Save to next instruction, 4 bytes past current instruction
        core.cpu.write(reg!(lr), pc + 4);
        let func_address = pc.wrapping_add_signed((self.label_offset - 4) as i64);
        core.cpu.stack_trace.push_bl(func_address + 4, pc);
        core.cpu.pc = func_address;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};
    #[test]
    pub fn simple_bl_test() -> anyhow::Result<()> {
         let mut cpu = Cpu0::default();
        cpu.pc = 0x1000;
        cpu.write(reg!(lr), 5);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);

        core.handle_string_command(&String::from("bl 50"))?;
        assert_eq!(cpu.pc, 0x1050);
        assert_eq!(cpu.read::<u64>(reg!(lr)), 0x1004);
        Ok(())
    }
}
