use crate::processor::arithmetic_utils::{signed_add_with_carry32, signed_add_with_carry64};
use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::Error;
use crate::Core;

    fn parse_adds(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        if collected_args[2].starts_with('#') {
            //Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(AddsImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            //Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(AddsInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }


#[derive(Clone)]
pub struct AddsInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub rm: RegisterType,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AddsInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.adds(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AddsImmInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub imm_val: i64,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AddsImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.adds_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `adds xd, xn, xm` with optional shift
    pub fn adds(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        xm: RegisterType,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;
        let (xm_val, _) = self.cpu.handle_extra_op(
            self.cpu.read_gen_reg(&xm)?,
            xm,
            xm.get_bitwidth(),
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, xn_val + xm_val)?;
        let flags = if xn.get_bitwidth() == 32 {
            signed_add_with_carry32(xn_val as i32, xm_val as i32, false)
        } else {
            signed_add_with_carry64(xn_val, xm_val, false)
        };
        self.cpu.flags = flags;
        Ok(())
    }

    /// Processes ARM64 command `adds xd, xn, imm` with optional shift
    pub fn adds_imm(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;
        let (imm_val, _) = self.cpu.handle_extra_op(
            imm,
            xn,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, xn_val + imm_val)?;
        let flags = if xn.get_bitwidth() == 32 {
            signed_add_with_carry32(xn_val as i32, imm_val as i32, false)
        } else {
            signed_add_with_carry64(xn_val, imm_val, false)
        };
        self.cpu.flags = flags;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_adds_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("add w9, wzr, #1"))?;
    core.handle_string_command(&String::from("add x8, xzr, #10"))?;
    core.handle_string_command(&String::from("adds x21, x8, w9, sxtw #4"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(21))?, 26);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.n);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn zero_adds_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("adds w9, wzr, wzr"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(9))?, 0);
    assert!(!core.cpu.flags.c);
    assert!(core.cpu.flags.z);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.n);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn negative_adds_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("adds w9, wzr, #-5"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(9))?, -5);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.v);
    assert!(core.cpu.flags.n);
    Ok(())
}
