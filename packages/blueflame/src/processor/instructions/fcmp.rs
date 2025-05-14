use crate::processor::Error;
use crate::processor::{instruction_registry::RegisterType, Flags};

use crate::Core;

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
