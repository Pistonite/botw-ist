use crate::{
    processor::{
        instruction_registry::{AuxiliaryOperation, RegisterType},
        Error,
    },
    Core,
};

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `eor xd, xn, xm` with optional shift
    pub fn bic(
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
        self.cpu.write_gen_reg(&xd, xn_val & !xm_val)?;
        Ok(())
    }

    /// Processes ARM64 command `eor xd, xn, imm` with optional shift
    pub fn bic_imm(
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
        self.cpu.write_gen_reg(&xd, xn_val & !imm_val)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_bic_test() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.write_gen_reg(&RegisterType::XReg(5), 0xFFFFFFFF)?;
    core.handle_string_command(&String::from("bic x5, x5, #191"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(5))?, 0xFFFFFF40);
    Ok(())
}
