use crate::processor::arithmetic_utils::{signed_add_with_carry32, signed_add_with_carry64};
use crate::processor::Error;
use crate::processor::{instruction_registry::RegisterType, Flags};

use crate::Core;

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `ccmp rn, rm, nzcv, condition`
    ///
    /// If the condition is true, sets the cpu flags to result of comparison between rn and rm
    pub fn ccmp(
        &mut self,
        rn: RegisterType,
        rm: RegisterType,
        nzcv: u8,        // 4 bits
        condition: &str, // A condition code like eq, ne, etc.
    ) -> Result<(), Error> {
        let mut flags = Flags::from_nzcv(nzcv);
        if self.cpu.flags.does_condition_succeed(condition)? {
            let operand1 = self.cpu.read_gen_reg(&rn)?;
            let operand2 = self.cpu.read_gen_reg(&rm)?;
            flags = if rn.get_bitwidth() == 32 {
                signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
            } else {
                signed_add_with_carry64(operand1, !operand2, true)
            };
        }

        self.cpu.flags = flags;
        Ok(())
    }

    /// Processes ARM64 command `ccmp rn, imm, nzcv, condition`
    ///
    /// If the condition is true, sets the cpu flags to result of comparison between rn and imm
    pub fn ccmp_imm(
        &mut self,
        rn: RegisterType,
        imm: u8,         // 5 bits
        nzcv: u8,        // 4 bits
        condition: &str, // A condition code like eq, ne, etc.
    ) -> Result<(), Error> {
        let mut flags = Flags::from_nzcv(nzcv);
        if self.cpu.flags.does_condition_succeed(condition)? {
            let operand1 = self.cpu.read_gen_reg(&rn)?;
            let operand2 = (imm as u64) as i64; // zero-extend
            flags = if rn.get_bitwidth() == 32 {
                signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
            } else {
                signed_add_with_carry64(operand1, !operand2, true)
            };
        }

        self.cpu.flags = flags;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn test_ccmp_reg_when_true() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("mov x2, #10"))?;
    core.handle_string_command(&String::from("ccmp x1, x2, #4, EQ"))?;
    assert!(core.cpu.flags.z);
    assert!(core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.n);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn test_ccmp_reg_when_false() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("mov x2, #11"))?;
    core.handle_string_command(&String::from("ccmp x1, x2, #4, EQ"))?;
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(core.cpu.flags.n);
    Ok(())
}
