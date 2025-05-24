use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};
use crate::processor::Error;

use crate::Core;

    fn parse_str(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let split_second: Vec<String> = Self::split_bracket_args(&collected_args[1]);
        let rt = RegisterType::from_str(&collected_args[0])?;
        let rn_sp = RegisterType::from_str(&split_second[0])?;
        let extra_op = Self::parse_auxiliary(split_second.get(2))?;
        let imm_val = if let Some(val) = split_second.get(1) {
            if val.starts_with('#') {
                Self::get_imm_val(val)?
            } else {
                let rm = RegisterType::from_str(val)?;
                return Ok(Box::new(StrInstruction {
                    rt,
                    rn_sp,
                    rm,
                    extra_op,
                }));
            }
        } else {
            0
        };
        if Self::ends_with_exclam(&collected_args[1]) {
            Ok(Box::new(StrPreInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[1].contains("], ") {
            Ok(Box::new(StrPostInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(StrImmInstruction {
                rt,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

#[derive(Clone)]
pub struct StrInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str(self.rt, self.rn_sp, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrPreInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str_pre_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrPostInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str_post_idx(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct StrImmInstruction {
    rt: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for StrImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.str_imm(self.rt, self.rn_sp, self.imm_val, self.extra_op.clone())
    }
}

impl Core<'_, '_, '_> {
    pub fn str(
        &mut self,
        xt: RegisterType,
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
        self.str_core(xt, memory_to_read as u64)
    }

    pub fn str_imm(
        &mut self,
        xt: RegisterType,
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
        self.str_core(xt, memory_to_read as u64)
    }

    pub fn str_pre_idx(
        &mut self,
        xt: RegisterType,
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
        let memory_to_read = xn_sp_val + imm_val;
        self.str_core(xt, memory_to_read as u64)?;
        self.cpu.write_gen_reg(&xn_sp, memory_to_read)?;
        Result::Ok(())
    }

    pub fn str_post_idx(
        &mut self,
        xt: RegisterType,
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
        self.str_core(xt, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        self.cpu.write_gen_reg(&xn_sp, new_reg_val)?;
        Result::Ok(())
    }

    fn str_core(&mut self, xt: RegisterType, address: u64) -> Result<(), Error> {
        // Implements core of str, interfacing w/ memory
        if !self.mem.verify_memory_alignment(address) {
            return Result::Ok(());
        }
        let xt_val = self.cpu.read_gen_reg(&xt)?;
        match xt {
            RegisterType::XReg(_) => self.mem.mem_write_i64(address, xt_val)?,
            RegisterType::WReg(_) => self.mem.mem_write_i32(address, xt_val as i32)?,
            RegisterType::XZR => self.mem.mem_write_i64(address, 0)?,
            RegisterType::WZR => self.mem.mem_write_i32(address, 0)?,
            RegisterType::DReg(_) => self.mem.mem_write_i64(address, xt_val)?,
            RegisterType::SReg(_) => self.mem.mem_write_i32(address, xt_val as i32)?,
            _ => return Err(Error::InvalidRegisterWrite("address", xt)),
        };
        Result::Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_str_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w0, #32"))?;
    core.handle_string_command(&String::from("str w0, [w0]"))?;
    assert_eq!(core.mem.mem_read_i32(32)?, 32);
    Ok(())
}
