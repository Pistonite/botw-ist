use crate::processor as self_;

use self_::insn::arithmetic_utils;
use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let extra_op = parse::parse_auxiliary(collected_args.get(3))?;

    if collected_args[2].starts_with('#') {
        // Immediate offset
        let imm_val = parse::get_imm_val(&collected_args[2])?;
        Some(Box::new(SubsImmInstruction {
            rd,
            rn,
            imm_val,
            extra_op,
        }))
    } else {
        // Register offset
        let rm = glue::parse_reg_or_panic(&collected_args[2]);
        Some(Box::new(SubsInstruction {
            rd,
            rn,
            rm,
            extra_op,
        }))
    }
}

#[derive(Clone)]
pub struct SubsInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubsInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);
        let (xm_val, _) = glue::handle_extra_op(
            core.cpu,
            glue::read_gen_reg(core.cpu, &self.rm),
            self.rm,
            self.rm.get_bitwidth(),
            self.extra_op.as_ref(),
        )?;
        if self.rn.get_bitwidth() == 32 {
            let xn_val = xn_val as i32;
            let xm_val = xm_val as i32;
            let (result, _) = xn_val.overflowing_sub(xm_val);
            let did_borrow = (xn_val as u32) < (xm_val as u32);
            glue::write_gen_reg(core.cpu, &self.rd, result as i64);
            core.cpu.flags = arithmetic_utils::get_nzcv_flags(result, xn_val, xm_val, did_borrow);
        } else {
            let (result, _) = xn_val.overflowing_sub(xm_val);
            let did_borrow = (xn_val as u64) < (xm_val as u64);
            glue::write_gen_reg(core.cpu, &self.rd, result);
            core.cpu.flags = arithmetic_utils::get_nzcv_flags(result, xn_val, xm_val, did_borrow);
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct SubsImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for SubsImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);
        let (imm_val, _) =
            glue::handle_extra_op_immbw(core.cpu, self.imm_val, self.rn, self.extra_op.as_ref())?;

        if self.rn.get_bitwidth() == 32 {
            let xn_val = xn_val as u32;
            let imm_val = imm_val as u32;
            let result = xn_val.wrapping_sub(imm_val) as i32;
            let did_borrow = xn_val < imm_val;
            glue::write_gen_reg(core.cpu, &self.rd, result as i64);
            core.cpu.flags =
                arithmetic_utils::get_nzcv_flags(result, xn_val as i32, imm_val as i32, did_borrow);
        } else {
            let xn_val = xn_val as u64;
            let imm_val = imm_val as u64;
            let result = xn_val.wrapping_sub(imm_val) as i64;
            let did_borrow = xn_val < imm_val;
            glue::write_gen_reg(core.cpu, &self.rd, result);
            core.cpu.flags = arithmetic_utils::get_nzcv_flags(
                result as i32,
                xn_val as i32,
                imm_val as i32,
                did_borrow,
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{reg, Cpu0, Process};

    #[test]
    pub fn test_subs_reg_when_true() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #10")?;
        core.handle_string_command("mov x2, #10")?;
        core.handle_string_command("subs x3, x1, x2")?;
        assert_eq!(cpu.read::<i64>(reg!(x[3])), 0);
        assert!(cpu.flags.z);
        assert!(cpu.flags.c);
        assert!(!cpu.flags.v);
        assert!(!cpu.flags.n);
        Ok(())
    }

    #[test]
    pub fn test_subs_reg_when_false() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.flags.z = true;
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("mov x1, #10")?;
        core.handle_string_command("mov x2, #11")?;
        core.handle_string_command("subs x3, x1, x2")?;
        assert_eq!(cpu.read::<i64>(reg!(x[3])), -1);
        assert!(!cpu.flags.z);
        assert!(!cpu.flags.c);
        assert!(!cpu.flags.v);
        assert!(cpu.flags.n);
        Ok(())
    }

    #[test]
    pub fn test_subs_cc_cond() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.write(reg!(w[1]), 2);
        cpu.write(reg!(w[2]), 1);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("subs wzr, w1, w2")?;
        core.cpu.write(reg!(w[9]), 1);
        core.cpu.write(reg!(w[10]), 2);
        core.handle_string_command("csel w8, w9, w10, CC")?;
        assert_eq!(cpu.read::<i32>(reg!(w[8])), 2);
        assert!(!cpu.flags.z);
        assert!(cpu.flags.c);
        assert!(!cpu.flags.v);
        assert!(!cpu.flags.n);
        Ok(())
    }
}
