use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error, reg};

pub    fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
        if args.is_empty() {
            Some(Box::new(RetInstruction))
        } else {
            let rn = glue::parse_reg_or_panic(args);
            Some(Box::new(RetArgsInstruction { rn }))
        }
    }

#[derive(Clone)]
pub struct RetInstruction;

#[derive(Clone)]
pub struct RetArgsInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for RetArgsInstruction {
    // NOTE: Seems to function the same as br, but has a "hint" that this is a subroutine return
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        core.cpu.retr(self.rn.to_regname())
    }
}

impl ExecutableInstruction for RetInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        core.cpu.ret()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_ret_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        cpu.stack_trace.push_bl(1000, 1u64);
        cpu.pc = 1000;
        cpu.write(reg!(lr), 5u64);
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command(&String::from("ret"))?;
        assert_eq!(core.cpu.pc, 5);
        Ok(())
    }

    #[test]
    pub fn simple_ret_test_with_arg() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        cpu.stack_trace.push_bl(1000, 0x50 - 4);
        cpu.pc = 1000;
        cpu.write::<u64>(reg!(x[10]), 0x50);
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command(&String::from("ret x10"))?;
        assert_eq!(core.cpu.pc, 0x50);
        Ok(())
    }
}
