use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 2);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let cond = collected_args[1].clone();

    Some(Box::new(CsetInstruction { rd, cond }))
}

#[derive(Clone)]
pub struct CsetInstruction {
    rd: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CsetInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        if core.cpu.flags.does_condition_succeed(&self.cond) {
            glue::write_gen_reg(core.cpu, &self.rd, 1);
        } else {
            glue::write_gen_reg(core.cpu, &self.rd, 0);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn test_cset_when_true() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("cset x1, EQ")?;
        assert_eq!(cpu.read::<i64>(reg!(x[1])), 1);
        Ok(())
    }

    #[test]
    pub fn test_cset_when_false() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = false;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("cset x1, EQ")?;
        assert_eq!(cpu.read::<i64>(reg!(x[1])), 0);
        Ok(())
    }
}
