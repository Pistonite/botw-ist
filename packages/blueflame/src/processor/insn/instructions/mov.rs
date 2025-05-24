use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::Error;
use crate::Core;

    fn parse_mov(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(2))?;
        if collected_args[1].starts_with('#') {
            let imm_val = Self::get_imm_val(&collected_args[1])?;
            Ok(Box::new(MovImmInstruction {
                rd,
                imm_val,
                extra_op,
            }))
        } else {
            let rn = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(MovInstruction { rd, rn, extra_op }))
        }
    }


#[derive(Clone)]
pub struct MovInstruction {
    rd: RegisterType,
    rn: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mov(self.rd, self.rn, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct MovImmInstruction {
    rd: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MovImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mov_imm(self.rd, self.imm_val, self.extra_op.clone())
    }
}

impl Core<'_, '_, '_> {
    // updates xd
    // restores xm
    pub fn mov(
        &mut self,
        xd: RegisterType,
        xm: RegisterType,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let (xm_val, _) = self.cpu.handle_extra_op(
            self.cpu.read_gen_reg(&xm)?,
            xm,
            xm.get_bitwidth(),
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, xm_val)?;
        Ok(())
    }

    pub fn mov_imm(
        &mut self,
        xd: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let (imm_val, _) = self.cpu.handle_extra_op(
            imm,
            xd,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, imm_val)?;
        Ok(())
    }
}
