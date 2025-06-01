use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let rm = glue::parse_reg_or_panic(&collected_args[2]);
    let xa = glue::parse_reg_or_panic(&collected_args[3]);
    Some(Box::new(MaddInstruction { rd, rn, rm, xa }))
}

#[derive(Clone)]
pub struct MaddInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    xa: RegisterType,
}

impl ExecutableInstruction for MaddInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);
        let xm_val = glue::read_gen_reg(core.cpu, &self.rm);
        let xa_val = glue::read_gen_reg(core.cpu, &self.xa);
        glue::write_gen_reg(core.cpu, &self.rd, xn_val * xm_val + xa_val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_madd_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #2")?;
        core.handle_string_command("mov x2, #3")?;
        core.handle_string_command("mov x3, #4")?;
        core.handle_string_command("madd x4, x1, x2, x3")?;
        assert_eq!(cpu.read::<i64>(reg!(x[4])), 10);
        Ok(())
    }
}
