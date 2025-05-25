use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 2);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    if collected_args[1].starts_with("#") {
        let float_val = parse::convert_to_f64(&collected_args[1])?;
        Some(Box::new(FmovImmInstruction { rd, float_val }))
    } else {
        let rn = glue::parse_reg_or_panic(&collected_args[1]);
        Some(Box::new(FmovInstruction { rd, rn }))
    }
}

#[derive(Clone)]
pub struct FmovInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for FmovInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let rn_val = glue::read_float_reg(core.cpu, &self.rn);
        glue::write_float_reg(core.cpu, &self.rd, rn_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct FmovImmInstruction {
    rd: RegisterType,
    float_val: f64,
}

impl ExecutableInstruction for FmovImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        glue::write_float_reg(core.cpu, &self.rd, self.float_val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_fmov_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("fmov	s1, #3.000000000000000000e+01")?;
        core.handle_string_command("fmov	s2, #-3.000000000000000000e+01")?;
        core.handle_string_command("fmov	s3, #1.250000000000000000e+01")?;
        core.handle_string_command("fmov	s4, #30.00000000000000000e-01")?;
        core.handle_string_command("fmov	s5, #3")?;
        core.handle_string_command("fmov	s6, #3.5")?;
        assert_eq!(cpu.read::<f32>(reg!(s[1])), 30.0);
        assert_eq!(cpu.read::<f32>(reg!(s[2])), -30.0);
        assert_eq!(cpu.read::<f32>(reg!(s[3])), 12.5);
        assert_eq!(cpu.read::<f32>(reg!(s[4])), 3.0);
        assert_eq!(cpu.read::<f32>(reg!(s[5])), 3.0);
        assert_eq!(cpu.read::<f32>(reg!(s[6])), 3.5);
        Ok(())
    }
}

