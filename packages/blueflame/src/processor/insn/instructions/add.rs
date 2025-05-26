#[layered_crate::import]
use processor::{
    self::{glue, RegisterType, Error},
    self::insn::Core,
    self::insn::instruction_parse::{
        self as parse, AuxiliaryOperation, ExecutableInstruction,
    },
};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 4);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let rn = glue::parse_reg_or_panic(&collected_args[1]);
    let extra_op = parse::parse_auxiliary(collected_args.get(3))?;
    if collected_args[2].starts_with('#') {
        //Immediate offset
        let imm_val = parse::get_imm_val(&collected_args[2])?;
        Some(Box::new(AddImmInstruction {
            rd,
            rn,
            imm_val,
            extra_op,
        }))
    } else {
        //Register offset
        let rm = glue::parse_reg_or_panic(&collected_args[2]);
        Some(Box::new(AddInstruction {
            rd,
            rn,
            rm,
            extra_op,
        }))
    }
}

#[derive(Clone)]
pub struct AddInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub rm: RegisterType,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AddInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);
        let (xm_val, _) = glue::handle_extra_op(
            core.cpu,
            glue::read_gen_reg(core.cpu, &self.rm),
            self.rm,
            self.rm.get_bitwidth(),
            self.extra_op.as_ref(),
        )?;
        glue::write_gen_reg(core.cpu, &self.rd, xn_val + xm_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct AddImmInstruction {
    pub rd: RegisterType,
    pub rn: RegisterType,
    pub imm_val: i64,
    pub extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AddImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let xn_val = glue::read_gen_reg(core.cpu, &self.rn);
        let (imm_val, _) = glue::handle_extra_op_immbw(
            core.cpu,
            self.imm_val,
            self.rn,
            self.extra_op.as_ref(),
        )?;
        glue::write_gen_reg(core.cpu, &self.rd, xn_val + imm_val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[layered_crate::import]
    use processor::{Cpu0, Process, reg};

    #[test]
    pub fn simple_add_test() -> anyhow::Result<()> {
        let mut cpu = Cpu0::default();
        let mut proc = Process::new_for_test();
        let mut core = Core::new(&mut cpu, &mut proc);
        core.handle_string_command("add w9, wzr, #1")?;
        core.handle_string_command("add x8, xzr, #10")?;
        core.handle_string_command("add x21, x8, w9, sxtw #4")?;
        assert_eq!(cpu.read::<i64>(reg!(x[21])), 26);
        Ok(())
    }
}

