use crate::processor::arithmetic_utils::{signed_add_with_carry32, signed_add_with_carry64};
use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_cmp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&collected_args[0])?;

        if Self::is_imm(&collected_args[1]) {
            let imm_val = Self::get_imm_val(&collected_args[1])? as u8;
            Ok(Box::new(CmpImmInstruction { rn, imm_val }))
        } else {
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(CmpInstruction { rn, rm }))
        }
    }


#[derive(Clone)]
pub struct CmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for CmpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cmp(self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct CmpImmInstruction {
    rn: RegisterType,
    imm_val: u8,
}

impl ExecutableInstruction for CmpImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cmp_imm(self.rn, self.imm_val)
    }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `cmp rn, rm`
    ///
    /// Sets cpu flags based on the result of `rn - rm`
    pub fn cmp(&mut self, rn: RegisterType, rm: RegisterType) -> Result<(), Error> {
        let operand1 = self.cpu.read_gen_reg(&rn)?;
        let operand2 = self.cpu.read_gen_reg(&rm)?;
        let flags = if rn.get_bitwidth() == 32 {
            signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
        } else {
            signed_add_with_carry64(operand1, !operand2, true)
        };
        self.cpu.flags = flags;
        Ok(())
    }

    /// Processes ARM64 command `cmp rn, imm`
    ///
    /// Sets cpu flags based on the result of `rn - imm`
    pub fn cmp_imm(
        &mut self,
        rn: RegisterType,
        imm: u8, // 5 bits
    ) -> Result<(), Error> {
        let operand1 = self.cpu.read_gen_reg(&rn)?;
        let operand2 = (imm as u64) as i64; // zero-extend
        let flags = if rn.get_bitwidth() == 32 {
            signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
        } else {
            signed_add_with_carry64(operand1, !operand2, true)
        };
        self.cpu.flags = flags;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn test_cmp_reg_when_true() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("cmp x1, x2"))?;
    assert!(core.cpu.flags.z);
    assert!(core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.n);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn test_cmp_reg_when_false() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("cmp x1, x2"))?;
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(core.cpu.flags.n);
    Ok(())
}
