use crate::processor::Error;
use crate::processor::{instruction_registry::RegisterType, Flags};

use crate::Core;

    fn parse_fcmp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&collected_args[0])?;
        if collected_args[1].starts_with("#0.0") {
            // Variant where you don't compare register with anything
            Ok(Box::new(FcmpZeroInstruction { rn }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(FcmpInstruction { rn, rm }))
        }
    }


#[derive(Clone)]
pub struct FcmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for FcmpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fcmp(self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct FcmpZeroInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for FcmpZeroInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.fcmp_zero(self.rn)
    }
}

impl Core<'_, '_, '_> {
    pub fn fcmp(&mut self, rn: RegisterType, rm: RegisterType) -> Result<(), Error> {
        let value_n = self.cpu.read_float_reg(&rn)?;
        let value_m = self.cpu.read_float_reg(&rm)?;

        if value_n.is_nan() || value_m.is_nan() {
            self.cpu.flags = Flags {
                n: false,
                z: false,
                c: false,
                v: true,
            }
        }

        let diff = value_n - value_m;

        self.cpu.flags = Flags {
            n: diff < 0.0,
            z: diff == 0.0,
            c: diff > 0.0,
            v: false,
        };

        Ok(())
    }

    pub fn fcmp_zero(&mut self, rn: RegisterType) -> Result<(), Error> {
        let value_n = self.cpu.read_float_reg(&rn)?;

        if value_n.is_nan() {
            self.cpu.flags = Flags {
                n: false,
                z: false,
                c: false,
                v: true,
            }
        }

        let diff = value_n;

        self.cpu.flags = Flags {
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
#[test]
pub fn simple_fcmp_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("fmov s0, #2.111e+01"))?;
    core.handle_string_command(&String::from("fmov s1, #3.111e+01"))?;
    core.handle_string_command(&String::from("fcmp s0, s1"))?;
    assert!(core.cpu.flags.n);
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.v);

    core.handle_string_command(&String::from("fmov s0, #2.111e+01"))?;
    core.handle_string_command(&String::from("fmov s1, #2.111e+01"))?;
    core.handle_string_command(&String::from("fcmp s0, s1"))?;
    assert!(!core.cpu.flags.n);
    assert!(core.cpu.flags.z);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.v);

    core.handle_string_command(&String::from("fmov s0, #5.111e+01"))?;
    core.handle_string_command(&String::from("fmov s1, #2.111e+01"))?;
    core.handle_string_command(&String::from("fcmp s0, s1"))?;
    assert!(!core.cpu.flags.n);
    assert!(!core.cpu.flags.z);
    assert!(core.cpu.flags.c);
    assert!(!core.cpu.flags.v);

    Ok(())
}
