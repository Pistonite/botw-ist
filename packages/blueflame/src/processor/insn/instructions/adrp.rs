use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args: Vec<String> = parse::split_args(args, 2);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let label_offsetess = parse::get_label_val(&collected_args[1])?;
    Some(Box::new(AdrpInstruction {
        rd,
        label_offsetess,
    }))
}

#[derive(Clone)]
pub struct AdrpInstruction {
    rd: RegisterType,
    label_offsetess: u64,
}

impl ExecutableInstruction for AdrpInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        // zero out bottom 12 bits to get to a page offset
        let label_page = self.label_offsetess & 0xFFFF_FFFF_FFFF_F000; // Align label address to 4 KB boundary
        let pc_page = core.cpu.pc & 0xFFFF_FFFF_FFFF_F000;
        let offset = label_page.wrapping_sub(pc_page);
        let page_val = pc_page.wrapping_add(offset);

        let result = pc_page + page_val;
        glue::write_gen_reg(core.cpu, &self.rd, result as i64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::{reg, Cpu0, Process};

    #[test]
    pub fn adrp_simple() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        cpu.pc = 0x4050;
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("adrp x0, 0x1000")?;
        assert_eq!(cpu.read::<i64>(reg!(x[0])), 0x5000);
        Ok(())
    }
}
