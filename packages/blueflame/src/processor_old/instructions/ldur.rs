use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};
use crate::processor::Error;

use crate::processor::RegisterValue;
use crate::Core;

impl Core<'_, '_, '_> {
    pub fn ldur(
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
        self.ldur_core(xt, memory_to_read as u64)
    }

    pub fn ldur_imm(
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
        self.ldur_core(xt, memory_to_read as u64)
    }

    pub fn ldur_pre_idx(
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
        self.ldur_core(xt, memory_to_read as u64)?;
        self.cpu.write_gen_reg(&xn_sp, memory_to_read)?;
        Result::Ok(())
    }

    pub fn ldur_post_idx(
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
        self.ldur_core(xt, xn_sp_val as u64)?;
        let new_reg_val = xn_sp_val + imm_val;
        self.cpu.write_gen_reg(&xn_sp, new_reg_val)?;
        Result::Ok(())
    }

    fn ldur_core(&mut self, xd: RegisterType, address: u64) -> Result<(), Error> {
        // Implements core of ldur, interfacing w/ memory
        let loaded_val: i64 = match xd {
            RegisterType::XReg(_) => self.mem.mem_read_i64(address)?,
            RegisterType::WReg(_) => self.mem.mem_read_i32(address)? as i64,
            RegisterType::SReg(_) => {
                let val = self.mem.mem_read_val::<f32>(address)?;
                self.cpu.write_reg(&xd, &RegisterValue::SReg(val))?;
                return Result::Ok(());
            }
            RegisterType::DReg(_) => {
                let val = self.mem.mem_read_f64(address)?;
                self.cpu.write_reg(&xd, &RegisterValue::DReg(val))?;
                return Result::Ok(());
            }
            _ => {
                return Err(Error::InvalidRegisterWrite("integer", xd));
            }
        };
        self.cpu.write_gen_reg(&xd, loaded_val)?;
        Result::Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_ldur_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.mem.mem_write_i32(32, 1234)?;
    core.handle_string_command(&String::from("add w0, wzr, #32"))?;
    core.handle_string_command(&String::from("ldur w1, [w0]"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(1))?, 1234);
    Ok(())
}
