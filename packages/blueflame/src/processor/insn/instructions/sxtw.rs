use crate::processor::Error;
use crate::Core;

use crate::processor::{instruction_registry::RegisterType, RegisterValue};

    fn parse_sxtw(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args: Vec<String> = Self::split_args(args, 2);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        Ok(Box::new(SxtwInstruction { rd, rn }))
    }

#[derive(Clone)]
pub struct SxtwInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for SxtwInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.sxtw(self.rd, self.rn)
    }
}

impl Core<'_, '_, '_> {
    pub fn sxtw(&mut self, xd: RegisterType, wn: RegisterType) -> Result<(), Error> {
        let reg_val: RegisterValue = self.cpu.read_reg(&wn);
        let value = match reg_val {
            RegisterValue::WReg(value) => value,
            _ => {
                return Err(Error::InstructionError(String::from(
                    "sxtw: non-W register used as operand",
                )))
            }
        };

        // Cast i32 to i64 which zero-extends
        self.cpu.write_gen_reg(&xd, value as i64)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn sxtw_with_positive_zero_extends() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w1, #1"))?;
    core.handle_string_command(&String::from("sxtw x3, w1"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(3))?, 1);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn sxtw_with_negative_one_extends() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w1, #-1"))?;
    core.handle_string_command(&String::from("sxtw x3, w1"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(3))?, -1);
    Ok(())
}
