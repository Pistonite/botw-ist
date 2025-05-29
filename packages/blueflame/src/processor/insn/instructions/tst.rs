use crate::processor as self_;

use self_::insn::Core;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 2);
    let rn = glue::parse_reg_or_panic(&collected_args[0]);

    if parse::is_imm(&collected_args[1]) {
        let imm_val = parse::get_imm_val(&collected_args[1])?;
        Some(Box::new(TstImmInstruction { rn, imm_val }))
    } else {
        let rm = glue::parse_reg_or_panic(&collected_args[1]);
        Some(Box::new(TstInstruction { rn, rm }))
    }
}

#[derive(Clone)]
pub struct TstInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for TstInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let vn = glue::read_gen_reg(core.cpu, &self.rn);
        let vm = glue::read_gen_reg(core.cpu, &self.rm);
        let result = vn & vm;
        core.cpu.flags.n = result < 0;
        core.cpu.flags.z = result == 0;
        Ok(())
    }
}

#[derive(Clone)]
pub struct TstImmInstruction {
    rn: RegisterType,
    imm_val: i64,
}

impl ExecutableInstruction for TstImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let vn = glue::read_gen_reg(core.cpu, &self.rn);
        let vm = self.imm_val;
        let result = vn & vm;
        core.cpu.flags.n = result < 0;
        core.cpu.flags.z = result == 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn tst_zero_result() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        // Check that other flags are unaffected
        cpu.flags.v = true;
        cpu.flags.c = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #0")?;
        core.handle_string_command("mov x2, #111")?;
        core.handle_string_command("tst x1, x2")?;
        assert!(cpu.flags.z);
        assert!(!cpu.flags.n);
        assert!(cpu.flags.c);
        assert!(cpu.flags.v);
        Ok(())
    }

    #[test]
    pub fn tst_negative_result() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        // Check that other flags are unaffected
        cpu.flags.v = true;
        cpu.flags.c = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #-1")?;
        core.handle_string_command("mov x2, #-700")?;
        core.handle_string_command("tst x1, x2")?;
        assert!(!cpu.flags.z);
        assert!(cpu.flags.n);
        assert!(cpu.flags.c);
        assert!(cpu.flags.v);
        Ok(())
    }

    #[test]
    pub fn tst_positive_result() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        // Check that other flags are unaffected
        cpu.flags.v = true;
        cpu.flags.c = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #1")?;
        core.handle_string_command("mov x2, #-701")?;
        core.handle_string_command("tst x1, x2")?;
        assert!(!cpu.flags.z);
        assert!(!cpu.flags.n);
        assert!(cpu.flags.c);
        assert!(cpu.flags.v);
        Ok(())
    }
}
