use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 2);
    let rn = glue::parse_reg_or_panic(&collected_args[0]);
    if collected_args[1].starts_with("#0.0") {
        // Variant where you don't compare register with anything
        Some(Box::new(FcmpZeroInstruction { rn }))
    } else {
        //Register offset
        let rm = glue::parse_reg_or_panic(&collected_args[1]);
        Some(Box::new(FcmpInstruction { rn, rm }))
    }
}

#[derive(Clone)]
pub struct FcmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for FcmpInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let value_n = glue::read_float_reg(core.cpu, &self.rn);
        let value_m = glue::read_float_reg(core.cpu, &self.rm);

        if value_n.is_nan() || value_m.is_nan() {
            core.cpu.flags = self_::Flags {
                n: false,
                z: false,
                c: false,
                v: true,
            }
        }

        let diff = value_n - value_m;

        core.cpu.flags = self_::Flags {
            n: diff < 0.0,
            z: diff == 0.0,
            c: diff > 0.0,
            v: false,
        };

        Ok(())
    }
}

#[derive(Clone)]
pub struct FcmpZeroInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for FcmpZeroInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let value_n = glue::read_float_reg(core.cpu, &self.rn);

        if value_n.is_nan() {
            core.cpu.flags = self_::Flags {
                n: false,
                z: false,
                c: false,
                v: true,
            }
        }

        let diff = value_n;

        core.cpu.flags = self_::Flags {
            n: diff < 0.0,
            z: diff == 0.0,
            c: diff > 0.0,
            v: false,
        };

        Ok(())
    }
}

// TODO: Write test for overflow
#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process};

    #[test]
    pub fn simple_fcmp_test_less() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("fmov s0, #2.111e+01")?;
        core.handle_string_command("fmov s1, #3.111e+01")?;
        core.handle_string_command("fcmp s0, s1")?;
        assert!(cpu.flags.n);
        assert!(!cpu.flags.z);
        assert!(!cpu.flags.c);
        assert!(!cpu.flags.v);
        Ok(())
    }

    #[test]
    pub fn simple_fcmp_test_equal() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("fmov s0, #2.111e+01")?;
        core.handle_string_command("fmov s1, #2.111e+01")?;
        core.handle_string_command("fcmp s0, s1")?;
        assert!(!cpu.flags.n);
        assert!(cpu.flags.z);
        assert!(!cpu.flags.c);
        assert!(!cpu.flags.v);
        Ok(())
    }

    #[test]
    pub fn simple_fcmp_test_greater() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);

        core.handle_string_command("fmov s0, #5.111e+01")?;
        core.handle_string_command("fmov s1, #2.111e+01")?;
        core.handle_string_command("fcmp s0, s1")?;
        assert!(!cpu.flags.n);
        assert!(!cpu.flags.z);
        assert!(cpu.flags.c);
        assert!(!cpu.flags.v);

        Ok(())
    }
}
