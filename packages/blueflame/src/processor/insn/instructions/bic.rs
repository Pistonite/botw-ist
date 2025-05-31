use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let extra_op = parse::parse_auxiliary(collected_args.get(3))?;
    if collected_args[2].starts_with('#') {
        //Immediate offset
        let imm_val = parse::get_imm_val(&collected_args[2])?;
        Some(Box::new(BicImmInstruction {
            rd,
            rn,
            imm_val,
            extra_op,
        }))
    } else {
        //Register offset
        let rm = glue::parse_reg_or_panic(&collected_args[2]);
        Some(Box::new(BicInstruction {
            rd,
            rn,
            rm,
            extra_op,
        }))
    }
}

#[derive(Clone)]
pub struct BicInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for BicInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);
        let (xm_val, _) = glue::handle_extra_op(
            core.cpu,
            glue::read_gen_reg(core.cpu, &self.rm),
            self.rm,
            self.rm.get_bitwidth(),
            self.extra_op.as_ref(),
        )?;
        glue::write_gen_reg(core.cpu, &self.rd, xn_val & !xm_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct BicImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for BicImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);
        let (imm_val, _) =
            glue::handle_extra_op_immbw(core.cpu, self.imm_val, self.rn, self.extra_op.as_ref())?;
        glue::write_gen_reg(core.cpu, &self.rd, xn_val & !imm_val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_::{reg, Cpu0, Process};

    #[test]
    pub fn simple_bic_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        cpu.write(reg!(x[5]), 0xFFFFFFFFu32);
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("bic x5, x5, #191")?;
        assert_eq!(cpu.read::<i64>(reg!(x[5])), 0xFFFFFF40);
        Ok(())
    }
}
