use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 3);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let rm = glue::parse_reg_or_panic(&collected_args[2]);

    Some(Box::new(FdivInstruction { rd, rn, rm }))
}

#[derive(Clone)]
pub struct FdivInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for FdivInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let value_n = glue::read_float_reg(core.cpu, &self.rn) as f32;
        let value_m = glue::read_float_reg(core.cpu, &self.rm) as f32;

        let result = value_n / value_m;
        glue::write_float_reg(core.cpu, &self.rd, result as f64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_fdiv_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("fmov s0, #1")?;
        core.handle_string_command("fmov s1, #3")?;
        core.handle_string_command("fdiv s0, s0, s1")?;
        let result = cpu.read::<f32>(reg!(s[0]));
        assert!((result - 0.333f32).abs() < 0.01);
        Ok(())
    }
}
