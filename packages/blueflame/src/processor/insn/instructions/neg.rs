use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let extra_op = parse::parse_auxiliary(collected_args.get(3))?;
    Some(Box::new(NegInstruction { rd, rn, extra_op }))
}

#[derive(Clone)]
pub struct NegInstruction {
    rd: RegisterType,
    rn: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for NegInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let (xn_val, _) = glue::handle_extra_op(
            core.cpu,
            glue::read_gen_reg(core.cpu, &self.rn),
            self.rn,
            self.rn.get_bitwidth(),
            self.extra_op.as_ref(),
        )?;
        glue::write_gen_reg(core.cpu, &self.rd, -xn_val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_neg_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w1, #75")?;
        core.handle_string_command("neg w2, w1")?;
        assert_eq!(cpu.read::<i32>(reg!(w[2])), -75);
        Ok(())
    }
}
