use crate::processor::instruction_registry::RegisterType;
use crate::processor::Error;

use crate::Core;

impl Core<'_, '_, '_> {
    pub fn fmov(&mut self, rd: RegisterType, rn: RegisterType) -> Result<(), Error> {
        let rn_val = self.cpu.read_float_reg(&rn)?;
        self.cpu.write_float_reg(&rd, rn_val)?;
        Ok(())
    }

    pub fn fmov_imm(&mut self, rd: RegisterType, float_value: f64) -> Result<(), Error> {
        self.cpu.write_float_reg(&rd, float_value)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_fmov_test() -> anyhow::Result<()> {
    use crate::processor::RegisterValue;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("fmov	s1, #3.000000000000000000e+01"))?;
    core.handle_string_command(&String::from("fmov	s2, #-3.000000000000000000e+01"))?;
    core.handle_string_command(&String::from("fmov	s3, #1.250000000000000000e+01"))?;
    core.handle_string_command(&String::from("fmov	s4, #30.00000000000000000e-01"))?;
    core.handle_string_command(&String::from("fmov	s5, #3"))?;
    core.handle_string_command(&String::from("fmov	s6, #3.5"))?;
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(1)),
        RegisterValue::SReg(30.0)
    );
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(2)),
        RegisterValue::SReg(-30.0)
    );
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(3)),
        RegisterValue::SReg(12.5)
    );
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(4)),
        RegisterValue::SReg(3.0)
    );
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(5)),
        RegisterValue::SReg(3.0)
    );
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(6)),
        RegisterValue::SReg(3.5)
    );
    Ok(())
}
