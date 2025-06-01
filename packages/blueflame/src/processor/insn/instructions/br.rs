use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::ExecutableInstruction;
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let rn = glue::parse_reg_or_panic(args);
    Some(Box::new(BrInstruction { rn }))
}

#[derive(Clone)]
pub struct BrInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for BrInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn) as u64;
        core.cpu.pc = xn_val - 4;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};
    #[test]
    pub fn simple_br_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.pc = 0x1000;
        cpu.write(reg!(x[10]), 0x50);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command(&String::from("br x10"))?;
        assert_eq!(core.cpu.pc, 0x50);
        Ok(())
    }
}
