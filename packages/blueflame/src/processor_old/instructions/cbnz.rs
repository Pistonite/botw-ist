use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;

impl Core<'_, '_, '_> {
    /// Processes the ARM64 command `cbnz xn, label`
    ///
    /// Branches to a PC relative label if xn is not zero
    pub fn cbnz(&mut self, xn: RegisterType, label_offset: u64) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;

        if xn_val != 0 {
            let new_pc = self.cpu.pc.wrapping_add_signed((label_offset - 4) as i64);
            self.cpu.pc = new_pc;
            // self.cpu.pc = label_offset - 4;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn cbnz_withnonzero_dobranch() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(10), 2)?;
    core.handle_string_command(&String::from("cbnz x10, 50"))?;
    assert_eq!(core.cpu.pc, 0x1050);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn cbnz_withzero_donotbranch() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(10), 0)?;
    core.handle_string_command(&String::from("cbnz x10, 50"))?;
    assert_eq!(core.cpu.pc, 0x1004);
    Ok(())
}
