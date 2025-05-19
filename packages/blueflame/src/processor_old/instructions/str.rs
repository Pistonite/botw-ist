use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};
use crate::processor::Error;

use crate::Core;

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
