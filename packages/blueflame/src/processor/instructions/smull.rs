use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

impl Core<'_, '_, '_> {
    pub fn smull(
        &mut self,
        xd: RegisterType,
        wn: RegisterType,
        wm: RegisterType,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&wn)?;
        let xm_val = self.cpu.read_gen_reg(&wm)?;
        self.cpu.write_gen_reg(&xd, xn_val * xm_val)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_smull_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w1, #2"))?;
    core.handle_string_command(&String::from("mov w2, #3"))?;
    core.handle_string_command(&String::from("smull x4, w1, w2"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(4))?, 6);
    Ok(())
}
