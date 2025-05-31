use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 2);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    Some(Box::new(ScvtfInstruction { rd, rn }))
}

#[derive(Clone)]
pub struct ScvtfInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for ScvtfInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        match self.rd {
            RegisterType::SReg(_) => match self.rn {
                RegisterType::WReg(_) => {
                    let current_val = glue::read_gen_reg(core.cpu, &self.rn);
                    let new_val = current_val as f32;
                    glue::write_float_reg(core.cpu, &self.rd, new_val as f64);
                    Ok(())
                }
                RegisterType::SReg(_) => {
                    let current_val = glue::read_float_reg(core.cpu, &self.rn) as f32;
                    let new_val = i32::from_le_bytes(current_val.to_le_bytes());
                    glue::write_float_reg(core.cpu, &self.rd, new_val as f32 as f64);
                    Ok(())
                }
                _ => {
                    log::error!(
                        "scvtf: Register type for rn is not supported: {:?}",
                        self.rn
                    );
                    Err(Error::BadInstruction(0))
                }
            },
            _ => {
                log::error!(
                    "scvtf: Register type for rd is not supported: {:?}",
                    self.rn
                );
                Err(Error::BadInstruction(0))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{reg, Cpu0, Process};

    #[test]
    pub fn simple_scvtf_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov w0, #4")?;
        core.handle_string_command("scvtf s0, w0")?;
        assert_eq!(cpu.read::<f32>(reg!(s[0])), 4.0);

        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov s0, #4")?;
        core.handle_string_command("scvtf s0, s0")?;
        assert_eq!(cpu.read::<f32>(reg!(s[0])), 4.0);
        Ok(())
    }
}
