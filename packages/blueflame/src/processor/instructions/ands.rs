use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::{Error, Flags};
use crate::Core;

impl Core<'_, '_, '_> {
    /// Processes the ARM64 command `ands xd, xn, xm` with optional shift
    pub fn ands(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        xm: RegisterType,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;
        let (xm_val, carry) = self.cpu.handle_extra_op(
            self.cpu.read_gen_reg(&xm)?,
            xm,
            xm.get_bitwidth(),
            extra_op,
        )?;
        let result = xn_val & xm_val;
        self.cpu.write_gen_reg(&xd, result)?;
        self.cpu.flags = Flags {
            n: result < 0,
            z: result == 0,
            c: carry,
            v: self.cpu.flags.v,
        };
        Ok(())
    }

    /// Processes the ARM64 command `ands xd, xn, imm` with optional shift
    pub fn ands_imm(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;
        let (imm_val, carry) = self.cpu.handle_extra_op(
            imm,
            xn,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        let result = xn_val & imm_val;
        self.cpu.write_gen_reg(&xd, result)?;
        self.cpu.flags = Flags {
            n: result < 0,
            z: result == 0,
            c: carry,
            v: self.cpu.flags.v,
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_ands_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov x1, #17"))?;
    core.handle_string_command(&String::from("mov x2, #14"))?;
    core.handle_string_command(&String::from("ands x3, x1, x2"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(3))?, 0);
    assert!(core.cpu.flags.z);
    assert!(!core.cpu.flags.n);
    assert!(!core.cpu.flags.v);
    assert!(!core.cpu.flags.c);
    Ok(())
}
