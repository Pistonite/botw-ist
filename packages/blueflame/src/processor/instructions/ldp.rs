use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};
use crate::processor::Error;

use crate::processor::RegisterValue;
use crate::Core;

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
            RegisterType::SReg(_) => {
                let val = self.mem.mem_read_f32(address)?;
                self.cpu.write_reg(&xt1, &RegisterValue::SReg(val))?;
                return Result::Ok(());
            }
            RegisterType::DReg(_) => {
                let val = self.mem.mem_read_f64(address)?;
                self.cpu.write_reg(&xt1, &RegisterValue::DReg(val))?;
                return Result::Ok(());
            }
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
            RegisterType::SReg(_) => {
                let val = self.mem.mem_read_f32(address)? as f64;
                self.cpu.write_reg(&xt2, &RegisterValue::HReg(val))?;
                return Result::Ok(());
            }
            RegisterType::DReg(_) => {
                let val = self.mem.mem_read_f64(address)?;
                self.cpu.write_reg(&xt1, &RegisterValue::DReg(val))?;
                return Result::Ok(());
            }
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
