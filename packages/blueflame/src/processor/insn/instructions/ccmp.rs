use crate::processor as self_;

use self_::insn::Core;
use self_::insn::arithmetic_utils;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::{Error, RegisterType, glue};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rn = glue::parse_reg_or_panic(&collected_args[0]);
    let nzcv_val = parse::get_imm_val(&collected_args[2])? as u8;
    let cond = collected_args[3].clone();

    if collected_args[1].starts_with('#') {
        // Immediate value case
        let imm_val = parse::get_imm_val(&collected_args[1])? as u8;
        Some(Box::new(CcmpImmInstruction {
            rn,
            imm_val,
            nzcv_val,
            cond,
        }))
    } else {
        // Register value case
        let rm = glue::parse_reg_or_panic(&collected_args[1]);
        Some(Box::new(CcmpInstruction {
            rn,
            rm,
            nzcv_val,
            cond,
        }))
    }
}

#[derive(Clone)]
pub struct CcmpInstruction {
    rn: RegisterType,
    rm: RegisterType,
    nzcv_val: u8,
    cond: String,
}

impl ExecutableInstruction for CcmpInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let mut flags = self_::Flags::from_nzcv(self.nzcv_val);
        if core.cpu.flags.does_condition_succeed(&self.cond) {
            let operand1 = glue::read_gen_reg(core.cpu, &self.rn);
            let operand2 = glue::read_gen_reg(core.cpu, &self.rm);
            flags = if self.rn.get_bitwidth() == 32 {
                arithmetic_utils::signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
            } else {
                arithmetic_utils::signed_add_with_carry64(operand1, !operand2, true)
            };
        }

        core.cpu.flags = flags;
        Ok(())
    }
}

#[derive(Clone)]
pub struct CcmpImmInstruction {
    rn: RegisterType,
    imm_val: u8,
    nzcv_val: u8,
    cond: String,
}

impl ExecutableInstruction for CcmpImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let mut flags = self_::Flags::from_nzcv(self.nzcv_val);
        if core.cpu.flags.does_condition_succeed(&self.cond) {
            let operand1 = glue::read_gen_reg(core.cpu, &self.rn);
            let operand2 = (self.imm_val as u64) as i64; // zero-extend
            flags = if self.rn.get_bitwidth() == 32 {
                arithmetic_utils::signed_add_with_carry32(operand1 as i32, !operand2 as i32, true)
            } else {
                arithmetic_utils::signed_add_with_carry64(operand1, !operand2, true)
            };
        }

        core.cpu.flags = flags;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use self_::{Cpu0, Process, reg};

    #[test]
    pub fn test_ccmp_reg_when_true() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #10")?;
        core.handle_string_command("mov x2, #10")?;
        core.handle_string_command("ccmp x1, x2, #4, EQ")?;
        assert!(cpu.flags.z);
        assert!(cpu.flags.c);
        assert!(!cpu.flags.v);
        assert!(!cpu.flags.n);
        Ok(())
    }

    #[test]
    pub fn test_ccmp_reg_when_false() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #10")?;
        core.handle_string_command("mov x2, #11")?;
        core.handle_string_command("ccmp x1, x2, #4, EQ")?;
        assert!(!cpu.flags.z);
        assert!(!cpu.flags.c);
        assert!(!cpu.flags.v);
        assert!(cpu.flags.n);
        Ok(())
    }
}
