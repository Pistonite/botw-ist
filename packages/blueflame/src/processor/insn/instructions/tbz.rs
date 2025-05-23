use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;

impl Core<'_, '_, '_> {
    // Note: imm is the bit number. Should be between 0 and 63 (or 0 and 31 for W registers)
    pub fn tbz(&mut self, xn: RegisterType, imm: u64, label_offset: u64) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;

        let bit_value = xn_val & (0b1 << imm);

        if bit_value == 0 {
            let new_pc = self.cpu.pc.wrapping_add_signed((label_offset - 4) as i64);
            self.cpu.pc = new_pc;
            // self.cpu.pc = label_offset - 4;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn tbz_does_branch() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(30), 4)?;
    core.handle_string_command(&String::from("tbz x30, 0, 50"))?;
    assert_eq!(core.cpu.pc, 0x1050);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn tbz_does_not_branch() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(30), 4)?;
    core.handle_string_command(&String::from("tbz x30, 2, 50"))?;
    assert_eq!(core.cpu.pc, 0x1004);
    Ok(())
}
