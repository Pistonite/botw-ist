use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::Error;
use crate::Core;

impl Core<'_, '_, '_> {
    pub fn movk(
        &mut self,
        xd: RegisterType,
        imm: u16,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let imm_val = self.cpu.handle_extra_op_unsigned(imm as u64, extra_op)?;
        let xd_val = self.cpu.read_gen_reg(&xd)? as u64;

        let xd_top_bits = xd_val & 0xFFFF_FFFF_FFFF_0000;
        let imm_bottom_bits = imm_val & 0x0000_0000_0000_FFFF;

        self.cpu
            .write_gen_reg(&xd, (xd_top_bits | imm_bottom_bits) as i64)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_movk_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu
        .write_gen_reg(&RegisterType::WReg(23), 0x12345678)?;
    core.handle_string_command(&String::from("movk w23, #0x4321"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(23))?, 0x12344321);
    Ok(())
}

#[test]
pub fn applied_movk_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command("mov w23, #0x40000")?;
    core.handle_string_command("movk w23, #0x4160")?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(23))?, 0x44160);
    Ok(())
}
