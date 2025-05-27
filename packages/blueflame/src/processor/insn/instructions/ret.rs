use crate::processor as self_;

use self_::insn::instruction_parse::{ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error, reg};

use blueflame_macros::trace_call;

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
        let regname = self.rn.to_regname();
        if regname != reg!(lr) {
            // check if we actually have any other register
            panic!("RET instruction must use LR register, got {}", regname);
        }
        let xn_val: u64 = core.cpu.read(regname);
        trace_call!(
            "main+0x{:08x} ret     >>>>> main+0x{:08x}", 
            core.cpu.pc - core.proc.main_start(),
            xn_val - core.proc.main_start()
        );
        // instruction executor will increment PC later
        let new_pc = xn_val - 4;
        core.cpu.stack_trace.pop_checked(xn_val)?;
        core.cpu.pc = new_pc as u64;
        Ok(())
    }
}

impl ExecutableInstruction for RetInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        panic!("RET instruction is not used since it's parsed as RET LR")
        // log::trace!("executing ret instruction");
        // let xn_val: u64 = core.cpu.read(reg!(lr));
        // // instruction executor will increment PC later
        // let new_pc = xn_val - 4;
        // core.cpu.stack_trace.pop_checked(xn_val)?;
        // core.cpu.pc = new_pc as u64;
        // Ok(())
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
