use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::Error;
use crate::Core;

    fn parse_subs(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(SubsImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(SubsInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }


#[derive(Clone)]
pub struct SubsInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubsInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.subs(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct SubsImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubsImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.subs_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

impl Core<'_, '_, '_> {
    pub fn subs(
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
        if xn.get_bitwidth() == 32 {
            let xn_val = xn_val as i32;
            let xm_val = xm_val as i32;
            let (result, _) = xn_val.overflowing_sub(xm_val);
            let did_borrow = (xn_val as u32) < (xm_val as u32);
            self.cpu.write_gen_reg(&xd, (result) as i64)?;
            self.update_nzcv_flags(result, xn_val, xm_val, did_borrow);
        } else {
            let (result, _) = xn_val.overflowing_sub(xm_val);
            let did_borrow = (xn_val as u64) < (xm_val as u64);
            self.cpu.write_gen_reg(&xd, result)?;
            self.update_nzcv_flags(result, xn_val, xm_val, did_borrow);
        }
        Ok(())
    }

    pub fn subs_imm(
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

        if xn.get_bitwidth() == 32 {
            let xn_val = xn_val as u32;
            let imm_val = imm_val as u32;
            let result = xn_val.wrapping_sub(imm_val) as i32;
            let did_borrow = xn_val < imm_val;
            self.cpu.write_gen_reg(&xd, result as i64)?;
            self.update_nzcv_flags(result, xn_val as i32, imm_val as i32, did_borrow);
        } else {
            let xn_val = xn_val as u64;
            let imm_val = imm_val as u64;
            let result = xn_val.wrapping_sub(imm_val) as i64;
            let did_borrow = xn_val < imm_val;
            self.cpu.write_gen_reg(&xd, result)?;
            self.update_nzcv_flags(result as i32, xn_val as i32, imm_val as i32, did_borrow);
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn test_subs_reg_when_true() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("subs x3, x1, x2"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(3))?, 0);
    assert!(core.cpu.flags.z);
    assert!(core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.n);
    Ok(())
}

#[test]
pub fn test_subs_reg_when_false() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("subs x3, x1, x2"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(3))?, -1);
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(core.cpu.flags.n);
    Ok(())
}

#[test]
pub fn test_subs_cc_cond() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.write_arg(1, 2);
    core.cpu.write_arg(2, 1);
    core.handle_string_command(&String::from("subs wzr, w1, w2"))?;
    core.cpu.write_arg(9, 1);
    core.cpu.write_arg(10, 2);
    core.handle_string_command(&String::from("csel w8, w9, w10, CC"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(8))?, 2);
    assert!(!core.cpu.flags.z);
    assert!(core.cpu.flags.c);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.n);
    Ok(())
}
