use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `csel, rn, rm, condition`
    ///
    /// Evalutes the ternary `condition ? rd = rn : rd = rm`
    pub fn csel(
        &mut self,
        rd: RegisterType,
        rn: RegisterType,
        rm: RegisterType,
        condition: &str, // A condition code like eq, ne, etc.
    ) -> Result<(), Error> {
        if self.cpu.flags.does_condition_succeed(condition)? {
            let value = self.cpu.read_gen_reg(&rn)?;
            self.cpu.write_gen_reg(&rd, value)?;
        } else {
            let value = self.cpu.read_gen_reg(&rm)?;
            self.cpu.write_gen_reg(&rd, value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn test_csel_when_true() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = true;
    core.handle_string_command(&String::from("mov x2, #10"))?;
    core.handle_string_command(&String::from("mov x3, #12"))?;
    core.handle_string_command(&String::from("csel x1, x2, x3, EQ"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, 10);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn test_csel_when_false() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = false;
    core.handle_string_command(&String::from("mov x2, #10"))?;
    core.handle_string_command(&String::from("mov x3, #12"))?;
    core.handle_string_command(&String::from("csel x1, x2, x3, EQ"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, 12);
    Ok(())
}
