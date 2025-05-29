use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let rm = glue::parse_reg_or_panic(&collected_args[2]);
    let cond = collected_args[3].clone();

    Some(Box::new(CselInstruction { rd, rn, rm, cond }))
}

#[derive(Clone)]
pub struct CselInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CselInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        if core.cpu.flags.does_condition_succeed(&self.cond) {
            let value = glue::read_gen_reg(core.cpu, &self.rn);
            glue::write_gen_reg(core.cpu, &self.rd, value);
        } else {
            let value = glue::read_gen_reg(core.cpu, &self.rm);
            glue::write_gen_reg(core.cpu, &self.rd, value);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn test_csel_when_true() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x2, #10")?;
        core.handle_string_command("mov x3, #12")?;
        core.handle_string_command("csel x1, x2, x3, EQ")?;
        assert_eq!(cpu.read::<i64>(reg!(x[1])), 10);
        Ok(())
    }

    #[test]
    pub fn test_csel_when_false() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = false;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x2, #10")?;
        core.handle_string_command("mov x3, #12")?;
        core.handle_string_command("csel x1, x2, x3, EQ")?;
        assert_eq!(cpu.read::<i64>(reg!(x[1])), 12);
        Ok(())
    }
}

