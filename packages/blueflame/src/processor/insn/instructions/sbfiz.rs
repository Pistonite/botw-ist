use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let lsb = parse::get_imm_val(&collected_args[2])?;
    let width = parse::get_imm_val(&collected_args[3])?;

    Some(Box::new(SbfizInstruction { rd, rn, lsb, width }))
}

#[derive(Clone)]
pub struct SbfizInstruction {
    rd: RegisterType,
    rn: RegisterType,
    lsb: i64,
    width: i64,
}

impl ExecutableInstruction for SbfizInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn) as u64;
        let lsb_val = self.lsb as u64;
        let width_val = self.width as u64;

        let mask = (1u64 << width_val) - 1;

        let extracted = (xn_val & mask) << lsb_val;
        let shift = 64 - (lsb_val + width_val);

        glue::write_gen_reg(core.cpu, &self.rd, ((extracted << shift) >> shift) as i64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_sbfiz_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #0x000000000000007F")?;
        core.handle_string_command("sbfiz x2, x1, #16, #8")?;
        //Makes this into 0x00000000007F0000
        assert_eq!(cpu.read::<i64>(reg!(x[2])), 8323072);
        Ok(())
    }
}
