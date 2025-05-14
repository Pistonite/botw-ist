use crate::processor::arithmetic_utils::{signed_add_with_carry32, signed_add_with_carry64};
use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

impl Core<'_, '_, '_> {
    /// Processes the ARM64 command `cmn rn, rm`
    ///
    /// May be unused in the real ARM64 instruction set
    pub fn cmn(&mut self, rn: RegisterType, rm: RegisterType) -> Result<(), Error> {
        let operand1 = self.cpu.read_gen_reg(&rn)?;
        let operand2 = self.cpu.read_gen_reg(&rm)?;
        let flags = if rn.get_bitwidth() == 32 {
            signed_add_with_carry32(operand1 as i32, operand2 as i32, false)
        } else {
            signed_add_with_carry64(operand1, operand2, false)
        };
        self.cpu.flags = flags;
        Ok(())
    }

    /// Processes the ARM64 command `cmn rn, imm`
    ///
    /// Sets the flags by doing `rn - (-imm)` / `rn + imm`
    pub fn cmn_imm(
        &mut self,
        rn: RegisterType,
        imm: u8, // 5 bits
    ) -> Result<(), Error> {
        let operand1 = self.cpu.read_gen_reg(&rn)?;
        let operand2 = (imm as u64) as i64; // zero-extend
        let flags = if rn.get_bitwidth() == 32 {
            signed_add_with_carry32(operand1 as i32, operand2 as i32, false)
        } else {
            signed_add_with_carry64(operand1, operand2, false)
        };
        self.cpu.flags = flags;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn test_cmn_reg_when_true() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = true;
    core.handle_string_command(&String::from("mov x1, #10"))?;
    core.handle_string_command(&String::from("mov x2, #-10"))?;
    core.handle_string_command(&String::from("cmn x1, x2"))?;
    assert!(core.cpu.flags.z);
    assert!(core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.n);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn test_cmn_reg_when_false() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = true;
    core.handle_string_command(&String::from("mov x1, #10"))?;
    core.handle_string_command(&String::from("mov x2, #-11"))?;
    core.handle_string_command(&String::from("cmn x1, x2"))?;
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(core.cpu.flags.n);
    Ok(())
}
