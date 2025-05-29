use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 3);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let wn = glue::parse_reg_or_panic(&collected_args[1]);
    let wm = glue::parse_reg_or_panic(&collected_args[2]);
    Some(Box::new(SmullInstruction { rd, wn, wm }))
}

#[derive(Clone)]
pub struct SmullInstruction {
    rd: RegisterType,
    wn: RegisterType,
    wm: RegisterType,
}

impl ExecutableInstruction for SmullInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.wn);
        let xm_val = glue::read_gen_reg(core.cpu, &self.wm);
        glue::write_gen_reg(core.cpu, &self.rd, xn_val * xm_val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_smull_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w1, #2")?;
        core.handle_string_command("mov w2, #3")?;
        core.handle_string_command("smull x4, w1, w2")?;
        assert_eq!(cpu.read::<i64>(reg!(x[4])), 6);
        Ok(())
    }
}
