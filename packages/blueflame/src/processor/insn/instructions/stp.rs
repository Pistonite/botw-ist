use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};
use crate::processor::Error;

use crate::Core;

    fn parse_stp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 3);
        let split_third: Vec<String> = Self::split_bracket_args(&collected_args[2]);
        let rt1 = RegisterType::from_str(&collected_args[0])?;
        let rt2 = RegisterType::from_str(&collected_args[1])?;
        let rn_sp = RegisterType::from_str(&split_third[0])?;
        let extra_op = Self::parse_auxiliary(split_third.get(2))?;
        let imm_val = if let Some(val) = split_third.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(StpInstruction {
                    rt1,
                    rt2,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[2]) {
            Ok(Box::new(StpPreInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[2].contains("], ") {
            Ok(Box::new(StpPostInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(StpImmInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

#[derive(Clone)]
pub struct StpInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.rm,
            self.extra_op.clone(),
        )
    }
}

#[derive(Clone)]
pub struct StpPreInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp_pre_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}

#[derive(Clone)]
pub struct StpPostInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp_post_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}

#[derive(Clone)]
pub struct StpImmInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StpImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.stp_imm(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}

impl Core<'_, '_, '_> {
    pub fn stp(
        &mut self,
        xt1: RegisterType,
        xt2: RegisterType,
        xn_sp: RegisterType,
        xm: RegisterType,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_sp_val: i64 = self.cpu.read_gen_reg(&xn_sp)?;
        let (xm_val, _) = self.cpu.handle_extra_op(
            self.cpu.read_gen_reg(&xm)?,
            xm,
            xm.get_bitwidth(),
            extra_op,
        )?;

        let memory_to_read = xn_sp_val + xm_val;
        self.stp_core(xt1, xt2, memory_to_read as u64)
    }

    pub fn stp_imm(
        &mut self,
        xt1: RegisterType,
        xt2: RegisterType,
        xn_sp: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_sp_val: i64 = self.cpu.read_gen_reg(&xn_sp)?;
        let (imm_val, _) = self.cpu.handle_extra_op(
            imm,
            xn_sp,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        self.stp_core(xt1, xt2, memory_to_read as u64)
    }

    pub fn stp_pre_idx(
        &mut self,
        xt1: RegisterType,
        xt2: RegisterType,
        xn_sp: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_sp_val = self.cpu.read_gen_reg(&xn_sp)?;
        let (imm_val, _) = self.cpu.handle_extra_op(
            imm,
            xn_sp,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        let memory_to_read: i64 = xn_sp_val + imm_val;
        self.stp_core(xt1, xt2, memory_to_read as u64)?;
        self.cpu.write_gen_reg(&xn_sp, memory_to_read)?;
        Result::Ok(())
    }

    pub fn stp_post_idx(
        &mut self,
        xt1: RegisterType,
        xt2: RegisterType,
        xn_sp: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_sp_val = self.cpu.read_gen_reg(&xn_sp)?;
        let (imm_val, _) = self.cpu.handle_extra_op(
            imm,
            xn_sp,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        self.stp_core(xt1, xt2, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        self.cpu.write_gen_reg(&xn_sp, new_reg_val)?;
        Result::Ok(())
    }

    fn stp_core(
        &mut self,
        xt1: RegisterType,
        xt2: RegisterType,
        address: u64,
    ) -> Result<(), Error> {
        // Implements core of stp, interfacing w/ memory
        let xt_val = self.cpu.read_gen_reg(&xt1)?;
        match xt1 {
            RegisterType::XReg(_) => self.mem.mem_write_i64(address, xt_val)?,
            RegisterType::WReg(_) => self.mem.mem_write_i32(address, xt_val as i32)?,
            RegisterType::XZR => self.mem.mem_write_i64(address, 0)?,
            RegisterType::WZR => self.mem.mem_write_i32(address, 0)?,
            RegisterType::SReg(_) => self
                .mem
                .mem_write_f32(address, self.cpu.read_float_reg(&xt1)? as f32)?,
            _ => return Err(Error::InvalidRegisterWrite("address", xt1)), //self.mem.mem_write_i64(address, xt_val)?,
        };
        let xt_val = self.cpu.read_gen_reg(&xt2)?;
        match xt2 {
            RegisterType::XReg(_) => self.mem.mem_write_i64(address + 8, xt_val)?,
            RegisterType::WReg(_) => self.mem.mem_write_i32(address + 4, xt_val as i32)?,
            RegisterType::XZR => self.mem.mem_write_i64(address + 8, 0)?,
            RegisterType::WZR => self.mem.mem_write_i32(address + 4, 0)?,
            RegisterType::SReg(_) => self
                .mem
                .mem_write_f32(address + 4, self.cpu.read_float_reg(&xt2)? as f32)?,
            _ => return Err(Error::InvalidRegisterWrite("store", xt2)), //self.mem.mem_write_i64(address + 8, xt_val)?,
        };
        Result::Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_stp_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w0, #32"))?;
    core.handle_string_command(&String::from("mov w1, #64"))?;
    core.handle_string_command(&String::from("stp w0, w1, [w0]"))?;
    assert_eq!(core.mem.mem_read_i32(32)?, 32);
    assert_eq!(core.mem.mem_read_i32(36)?, 64);
    Ok(())
}
