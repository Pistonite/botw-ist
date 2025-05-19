use crate::processor::Error;
use crate::processor::{instruction_registry::RegisterType, RegisterValue};

use crate::Core;

impl Core<'_, '_, '_> {
    pub fn scvtf(&mut self, rd: RegisterType, rn: RegisterType) -> Result<(), Error> {
        match rd {
            RegisterType::SReg(_) => match rn {
                RegisterType::WReg(_) => {
                    let current_val = self.cpu.read_gen_reg(&rn)?;
                    let new_val = current_val as f32;
                    let register_value: RegisterValue = RegisterValue::SReg(new_val);
                    self.cpu.write_reg(&rd, &register_value)?;
                    Ok(())
                }
                RegisterType::SReg(_) => {
                    let current_val = self.cpu.read_float_reg(&rn)? as f32;
                    let new_val = i32::from_le_bytes(current_val.to_le_bytes());
                    let register_value: RegisterValue = RegisterValue::SReg(new_val as f32);
                    self.cpu.write_reg(&rd, &register_value)?;
                    Ok(())
                }
                _ => Err(Error::InstructionError(String::from(
                    "scvtf: Register type for rn is not supported",
                ))),
            },
            _ => Err(Error::InstructionError(String::from(
                "scvtf: Register type for rd is not supported",
            ))),
        }
    }
}

#[cfg(test)]
#[test]
pub fn simple_scvtf_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w0, #4"))?;
    core.handle_string_command(&String::from("scvtf s0, w0"))?;
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(0)),
        RegisterValue::SReg(4.0)
    );

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov s0, #4"))?;
    core.handle_string_command(&String::from("scvtf s0, s0"))?;
    assert_eq!(
        core.cpu.read_reg(&RegisterType::SReg(0)),
        RegisterValue::SReg(4.0)
    );
    Ok(())
}
