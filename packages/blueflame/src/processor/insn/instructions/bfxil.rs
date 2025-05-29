use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let lsb = parse::get_imm_val(&collected_args[2])?;
    let width = parse::get_imm_val(&collected_args[3])?;

    Some(Box::new(BfxilInstruction { rd, rn, lsb, width }))
}

#[derive(Clone)]
pub struct BfxilInstruction {
    rd: RegisterType,
    rn: RegisterType,
    lsb: i64,
    width: i64,
}

impl ExecutableInstruction for BfxilInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xd_val = glue::read_gen_reg(core.cpu, &self.rd) as u64;
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn) as u64;
        let lsb_val = self.lsb as u64;
        let width_val = self.width as u64;

        let mask = (1u64 << width_val) - 1;

        let extracted = (xn_val & mask) << lsb_val;

        let cleared_dst = xd_val & !(mask << lsb_val);

        glue::write_gen_reg(core.cpu, &self.rd, (cleared_dst | extracted) as i64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_bxfil_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #0x00000FFF")?;
        core.handle_string_command("mov x2, #0xFFFF0000")?;
        core.handle_string_command("bfxil x2, x1, #4, #8")?;
        //Makes this into 0xFFFF0FF0
        assert_eq!(cpu.read::<i64>(reg!(x[2])), 4294905840);
        Ok(())
    }
}
