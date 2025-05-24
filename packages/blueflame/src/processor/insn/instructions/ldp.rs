use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};
use crate::processor::Error;

use crate::processor::RegisterValue;
use crate::Core;

    fn parse_ldp(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
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
                return Ok(Box::new(LdpInstruction {
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
            Ok(Box::new(LdpPreInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else if collected_args[2].contains("], ") {
            Ok(Box::new(LdpPostInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        } else {
            Ok(Box::new(LdpImmInstruction {
                rt1,
                rt2,
                rn_sp,
                imm_val,
                extra_op,
            }))
        }
    }

#[derive(Clone)]
pub struct LdpPostInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdpPostInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp_post_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}

#[derive(Clone)]
pub struct LdpImmInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdpImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp_imm(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}

#[derive(Clone)]
pub struct LdpPreInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdpPreInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp_pre_idx(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.imm_val,
            self.extra_op.clone(),
        )
    }
}


#[derive(Clone)]
pub struct LdpInstruction {
    rt1: RegisterType,
    rt2: RegisterType,
    rn_sp: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for LdpInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.ldp(
            self.rt1,
            self.rt2,
            self.rn_sp,
            self.rm,
            self.extra_op.clone(),
        )
    }
}

impl Core<'_, '_, '_> {
    pub fn ldp(
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

        let mem_to_read = xn_sp_val + xm_val;
        self.ldp_core(xt1, xt2, mem_to_read as u64)
    }

    pub fn ldp_imm(
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
        let mem_to_read: i64 = xn_sp_val + imm_val;
        self.ldp_core(xt1, xt2, mem_to_read as u64)
    }

    pub fn ldp_pre_idx(
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
        let mem_to_read: i64 = xn_sp_val + imm_val;
        self.ldp_core(xt1, xt2, mem_to_read as u64)?;
        self.cpu.write_gen_reg(&xn_sp, mem_to_read)?;
        Result::Ok(())
    }

    pub fn ldp_post_idx(
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
        self.ldp_core(xt1, xt2, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        self.cpu.write_gen_reg(&xn_sp, new_reg_val)?;
        Result::Ok(())
    }

    fn ldp_core(
        &mut self,
        xt1: RegisterType,
        xt2: RegisterType,
        address: u64,
    ) -> Result<(), Error> {
        // Implements core of ldr, interfacing w/ mem
        let loaded_val: i64 = match xt1 {
            RegisterType::XReg(_) => self.mem.mem_read_i64(address)?,
            RegisterType::WReg(_) => self.mem.mem_read_i32(address)? as i64,
            RegisterType::SReg(_) => self.mem.mem_read_i32(address)? as i64,
            RegisterType::DReg(_) => self.mem.mem_read_i64(address)?,
            _ => {
                return Err(Error::InstructionError(String::from(
                    "Loading into non-general register type",
                )));
            }
        };
        self.cpu.write_gen_reg(&xt1, loaded_val)?;
        let loaded_val: i64 = match xt2 {
            RegisterType::XReg(_) => self.mem.mem_read_i64(address + 8)?,
            RegisterType::WReg(_) => self.mem.mem_read_i32(address + 4)? as i64,
            RegisterType::SReg(_) => self.mem.mem_read_i32(address + 4)? as i64,
            RegisterType::DReg(_) => self.mem.mem_read_i64(address + 8)?,
            _ => {
                return Err(Error::InstructionError(String::from(
                    "Loading into non-general register type",
                )));
            }
        };
        self.cpu.write_gen_reg(&xt2, loaded_val)?;
        Result::Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_ldp_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.mem.mem_write_i32(32, 1234)?;
    core.mem.mem_write_i32(36, 5678)?;
    core.handle_string_command(&String::from("add w0, wzr, #32"))?;
    core.handle_string_command(&String::from("ldp w1, w2, [w0]"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(1))?, 1234);
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(2))?, 5678);
    Ok(())
}
