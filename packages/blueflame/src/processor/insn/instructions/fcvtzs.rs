use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 2);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);

    Some(Box::new(FcvtzsInstruction { rd, rn }))
}

#[derive(Clone)]
pub struct FcvtzsInstruction {
    rd: RegisterType,
    rn: RegisterType,
}

impl ExecutableInstruction for FcvtzsInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        match (self.rn, self.rd) {
            (RegisterType::SReg(_), RegisterType::WReg(_)) => {
                let current_val = glue::read_float_reg(core.cpu, &self.rn);
                let new_val = current_val as i32;
                glue::write_gen_reg(core.cpu, &self.rd, new_val as i64);
                Ok(())
            }
            _ => {
                log::error!(
                    "fcvtzs: Register type for rn or rd is not supported: rn = {:?}, rd = {:?}",
                    self.rn,
                    self.rd
                );
                Err(Error::BadInstruction(0))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_fcvtzs_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("fmov s0, #32")?;
        core.handle_string_command("fcvtzs w0, s0")?;
        assert_eq!(core.cpu.read::<i32>(reg!(w[0])), 32);

        core.handle_string_command("fmov s0, #32.89")?;
        core.handle_string_command("fcvtzs w0, s0")?;
        assert_eq!(core.cpu.read::<i32>(reg!(w[0])), 32);

        core.handle_string_command("fmov s0, #-32.89")?;
        core.handle_string_command("fcvtzs w0, s0")?;
        assert_eq!(cpu.read::<i32>(reg!(w[0])), -32);
        Ok(())
    }
}
