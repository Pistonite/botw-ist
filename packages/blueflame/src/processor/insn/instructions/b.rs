use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error};

pub     fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let label_offset = parse::get_label_val(args)?;
    Some(Box::new(BInstruction { label_offset }))
}

#[derive(Clone)]
pub struct BInstruction {
    /// The label address is pc relative
    label_offset: u64,
}

impl ExecutableInstruction for BInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let new_pc = (core.cpu.pc as i64) + (self.label_offset as i64) - 4;
        core.cpu.pc = new_pc as u64;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg}; 
    #[test]
    pub fn simple_b_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.cpu.pc = 0x1000;
        core.handle_string_command("b 50")?;
        assert_eq!(core.cpu.pc, 0x1050); // after incrementing
        Ok(())
    }
}
