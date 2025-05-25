use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args: Vec<String> = parse::split_args(args, 2);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    Some(Box::new(SxtwInstruction { rd, rn }))
}

#[derive(Clone)]
pub struct SxtwInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for SxtwInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let value = glue::read_gen_reg(core.cpu, &self.rn) as i32;
        // Cast i32 to i64 which sign-extends
        glue::write_gen_reg(core.cpu, &self.rd, value as i64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn sxtw_with_positive_zero_extends() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w1, #1")?;
        core.handle_string_command("sxtw x3, w1")?;
        assert_eq!(cpu.read::<i64>(reg!(x[3])), 1);
        Ok(())
    }

    #[test]
    pub fn sxtw_with_negative_one_extends() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w1, #-1")?;
        core.handle_string_command("sxtw x3, w1")?;
        assert_eq!(cpu.read::<i64>(reg!(x[3])), -1);
        Ok(())
    }
}

