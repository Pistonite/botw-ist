use crate::processor as self_;

use self_::insn::instruction_parse::{self as parse, AuxiliaryOperation, ExecutableInstruction};
use self_::insn::Core;
use self_::{glue, Error, RegisterType};

pub fn parse(args: &str) -> Option<Box<dyn ExecutableInstruction>> {
    let collected_args = parse::split_args(args, 3);
    let rd = glue::parse_reg_or_panic(&collected_args[0]);
    let extra_op = parse::parse_auxiliary(collected_args.get(2))?;
    if collected_args[1].starts_with('#') {
        let imm_val = parse::get_imm_val(&collected_args[1])?;
        Some(Box::new(MovImmInstruction {
            rd,
            imm_val,
            extra_op,
        }))
    } else {
        let rn = glue::parse_reg_or_panic(&collected_args[1]);
        Some(Box::new(MovInstruction { rd, rn, extra_op }))
    }
}

#[derive(Clone)]
pub struct MovInstruction {
    rd: RegisterType,
    rn: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        // updates xd
        // restores xm
        let (xm_val, _) = glue::handle_extra_op(
            core.cpu,
            glue::read_gen_reg(core.cpu, &self.rn),
            self.rn,
            self.rn.get_bitwidth(),
            self.extra_op.as_ref(),
        )?;
        glue::write_gen_reg(core.cpu, &self.rd, xm_val);
        Ok(())
    }
}

#[derive(Clone)]
pub struct MovImmInstruction {
    rd: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovImmInstruction {
    fn exec_on(&self, core: &mut Core) -> Result<(), Error> {
        let (imm_val, _) =
            glue::handle_extra_op_immbw(core.cpu, self.imm_val, self.rd, self.extra_op.as_ref())?;
        glue::write_gen_reg(core.cpu, &self.rd, imm_val);
        Ok(())
    }
}
