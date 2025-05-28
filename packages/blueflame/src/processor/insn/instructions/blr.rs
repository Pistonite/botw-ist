use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, RegisterType, Error, reg};

use blueflame_deps::trace_call;

pub    fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
        let rn = glue::parse_reg_or_panic(args);
        Some(Box::new(BlrInstruction { rn }))
    }

#[derive(Clone)]
pub struct BlrInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for BlrInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let pc = core.cpu.pc;
        let regname = self.rn.to_regname();
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn) as u64 - 4;
        trace_call!(
            "main+0x{:08x} blr {:3} >>>>> main+0x{:08x}", 
            core.cpu.pc - core.proc.main_start(),
            regname.to_string(),
            xn_val + 4 - core.proc.main_start()
        );
        let lr = pc + 4;

        let target = xn_val + 4;

        core.cpu.stack_trace.push_blr(target, regname, pc);
        core.cpu.pc = xn_val; // before incrementing
        core.cpu.write(reg!(lr), lr);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn simple_blr_test() -> anyhow::Result<()> {
         let mut cpu = Cpu0::default();
        cpu.pc = 0x1000;
        cpu.write(reg!(lr), 5);
        cpu.write(reg!(x[10]), 0x50);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command(&String::from("blr x10"))?;
        assert_eq!(core.cpu.pc, 0x50);
        assert_eq!(cpu.read::<u64>(reg!(lr)), 0x1004);
        Ok(())
    }
}
