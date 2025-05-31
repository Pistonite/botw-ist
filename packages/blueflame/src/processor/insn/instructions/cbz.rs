use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let split = parse::split_args(args, 2);
    let rn = glue::parse_reg_or_panic(&split[0]);
    let label_offset = parse::get_label_val(&split[1])?;
    Some(Box::new(CbzInstruction { rn, label_offset }))
}

#[derive(Clone)]
pub struct CbzInstruction {
    rn: RegisterType,
    label_offset: u64,
}

impl ExecutableInstruction for CbzInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);

        if xn_val == 0 {
            let new_pc = core
                .cpu
                .pc
                .wrapping_add_signed((self.label_offset - 4) as i64);
            core.cpu.pc = new_pc;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{reg, Cpu0, Process};

    #[test]
    pub fn cbz_withzero_dobranch() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.pc = 0x1000;
        cpu.write(reg!(x[10]), 0);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("cbz x10, 50")?;
        assert_eq!(cpu.pc, 0x1050);
        Ok(())
    }

    #[test]
    pub fn cbz_withnonzero_donotbranch() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.pc = 0x1000;
        cpu.write(reg!(x[10]), 2);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("cbz x10, 50")?;
        assert_eq!(cpu.pc, 0x1004);
        Ok(())
    }
}
