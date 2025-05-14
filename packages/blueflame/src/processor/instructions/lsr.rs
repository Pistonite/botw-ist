use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::Error;
use crate::Core;

impl Core<'_, '_, '_> {
    pub fn lsr(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        xm: RegisterType,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)? as u64;
        let (xm_val, _) = self.cpu.handle_extra_op(
            self.cpu.read_gen_reg(&xm)?,
            xm,
            xm.get_bitwidth(),
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, (xn_val >> xm_val) as i64)?;
        Ok(())
    }

    pub fn lsr_imm(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)? as u64;
        let (imm_val, _) = self.cpu.handle_extra_op(
            imm,
            xn,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, (xn_val >> imm_val) as i64)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_lsr_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov x1, #1"))?;
    core.handle_string_command(&String::from("lsr x1, x1, #1"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, 0);
    Ok(())
}
