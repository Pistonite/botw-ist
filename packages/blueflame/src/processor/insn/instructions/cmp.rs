use crate::processor as self_;

use self_::insn::Core;
use self_::insn::arithmetic_utils;
use self_::insn::instruction_parse::{self as parse, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 2);
    let rn = glue::parse_reg_or_panic(&collected_args[0]);

    if parse::is_imm(&collected_args[1]) {
        let imm_val = parse::get_imm_val(&collected_args[1])? as u8;
        Some(Box::new(CmpImmInstruction { rn, imm_val }))
    } else {
        let rm = glue::parse_reg_or_panic(&collected_args[1]);
        Some(Box::new(CmpInstruction { rn, rm }))
    }
}

#[derive(Clone)]
pub struct CmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for CmpInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let operand1 = glue::read_gen_reg(core.cpu, &self.rn);
        let operand2 = glue::read_gen_reg(core.cpu, &self.rm);
        let flags = if self.rn.get_bitwidth() == 32 {
            arithmetic_utils::signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
        } else {
            arithmetic_utils::signed_add_with_carry64(operand1, !operand2, true)
        };
        core.cpu.flags = flags;
        Ok(())
    }
}

#[derive(Clone)]
pub struct CmpImmInstruction {
    rn: RegisterType,
    imm_val: u8,
}

impl ExecutableInstruction for CmpImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let operand1 = glue::read_gen_reg(core.cpu, &self.rn);
        let operand2 = (self.imm_val as u64) as i64; // zero-extend
        let flags = if self.rn.get_bitwidth() == 32 {
            arithmetic_utils::signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
        } else {
            arithmetic_utils::signed_add_with_carry64(operand1, !operand2, true)
        };
        core.cpu.flags = flags;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{Cpu0, Process};

    #[test]
    pub fn test_cmp_reg_when_true() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #10")?;
        core.handle_string_command("mov x2, #10")?;
        core.handle_string_command("cmp x1, x2")?;
        assert!(cpu.flags.z);
        assert!(cpu.flags.c);
        assert!(!cpu.flags.v);
        assert!(!cpu.flags.n);
        Ok(())
    }

    #[test]
    pub fn test_cmp_reg_when_false() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #10")?;
        core.handle_string_command("mov x2, #11")?;
        core.handle_string_command("cmp x1, x2")?;
        assert!(!cpu.flags.z);
        assert!(!cpu.flags.c);
        assert!(!cpu.flags.v);
        assert!(cpu.flags.n);
        Ok(())
    }
}
