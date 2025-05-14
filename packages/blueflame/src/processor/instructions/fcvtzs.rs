use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

impl Core<'_, '_, '_> {
    // rd is either an X or W register
    // rn is either an S or D register
    // Code seems to only use sn -> wn
    // Used to cast the floating point register to an integer
    pub fn fcvtzs(&mut self, rd: RegisterType, rn: RegisterType) -> Result<(), Error> {
        match (rn, rd) {
            (RegisterType::SReg(_), RegisterType::WReg(_)) => {
                let current_val = self.cpu.read_float_reg(&rn)?;
                let new_val = current_val as i32;
                self.cpu.write_gen_reg(&rd, new_val as i64)?;
                Ok(())
            }
            _ => Err(Error::InstructionError(String::from(
                "fcvtzs: Register type for rn or rd is not supported",
            ))),
        }
    }
}

#[cfg(test)]
#[test]
pub fn simple_fcvtzs_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("fmov s0, #32"))?;
    core.handle_string_command(&String::from("fcvtzs w0, s0"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(0))?, 32);

    core.handle_string_command(&String::from("fmov s0, #32.89"))?;
    core.handle_string_command(&String::from("fcvtzs w0, s0"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(0))?, 32);

    core.handle_string_command(&String::from("fmov s0, #-32.89"))?;
    core.handle_string_command(&String::from("fcvtzs w0, s0"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(0))?, -32);
    Ok(())
}
