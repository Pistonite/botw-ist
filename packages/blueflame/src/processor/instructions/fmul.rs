use crate::processor::Error;
use crate::processor::{instruction_registry::RegisterType, RegisterValue};

use crate::Core;

impl Core<'_, '_, '_> {
    pub fn fmul(
        &mut self,
        rd: RegisterType,
        rn: RegisterType,
        rm: RegisterType,
    ) -> Result<(), Error> {
        let value_n = self.cpu.read_float_reg(&rn)? as f32;
        let value_m = self.cpu.read_float_reg(&rm)? as f32;

        let result = value_n * value_m;
        self.cpu.write_reg(&rd, &RegisterValue::SReg(result))?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_fmul_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("fmov s0, #2"))?;
    core.handle_string_command(&String::from("fmov s1, #3"))?;
    core.handle_string_command(&String::from("fmul s0, s0, s1"))?;
    assert_eq!(core.cpu.read_float_reg(&RegisterType::SReg(0))?, 6.0);
    Ok(())
}
