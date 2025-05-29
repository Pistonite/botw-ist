use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args: Vec<String> = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let immr = parse::get_imm_val(&collected_args[2])?;
    let imms = parse::get_imm_val(&collected_args[3])?;
    Some(Box::new(UbfmInstruction { rn, rd, immr, imms }))
}

fn select_bits(value: i64, m: i64, n: i64) -> i64 {
    (value >> m) & ((1 << (n - m)) - 1)
}

#[derive(Clone)]
pub struct UbfmInstruction {
    rn: RegisterType,
    rd: RegisterType,
    immr: i64,
    imms: i64,
}

impl ExecutableInstruction for UbfmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let val = glue::read_gen_reg(core.cpu, &self.rn);
        if self.imms >= self.immr {
            let start = self.immr;
            let end = self.imms + 1;
            let bits = select_bits(val, start, end);
            glue::write_gen_reg(core.cpu, &self.rd, bits);
        } else {
            let start = 0;
            let end = self.imms + 1;
            let bits = select_bits(val, start, end);
            let regsz = match &self.rd {
                RegisterType::XReg(_) => 64,
                RegisterType::WReg(_) => 32,
                _ => {
                    log::error!("Invalid register type for UBFM: {:?}", self.rd);
                    return Err(Error::BadInstruction(0));
                }
            };
            let bits = bits << (regsz - self.immr);
            glue::write_gen_reg(core.cpu, &self.rd, bits);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_ubfm_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("add w9, wzr, #11")?; // b0...01011
        core.handle_string_command("ubfm w8, w9, #1, #3")?; // should end up with b101 (5)
        assert_eq!(cpu.read::<i32>(reg!(w[8])), 5);
        Ok(())
    }
}
