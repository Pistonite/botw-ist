use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::Error;
use crate::Core;

    fn parse_asr(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;

        if collected_args[2].starts_with('#') {
            // Immediate offset
            let imm_val = Self::get_imm_val(&collected_args[2])?;
            Ok(Box::new(AsrImmInstruction {
                rd,
                rn,
                imm_val,
                extra_op,
            }))
        } else {
            // Register offset
            let rm = RegisterType::from_str(&collected_args[2])?;
            Ok(Box::new(AsrInstruction {
                rd,
                rn,
                rm,
                extra_op,
            }))
        }
    }


#[derive(Clone)]
pub struct AsrInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AsrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.asr(self.rd, self.rn, self.rm, self.extra_op.clone())
    }
}

#[derive(Clone)]
pub struct AsrImmInstruction {
    rd: RegisterType,
    rn: RegisterType,
    imm_val: i64,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for AsrImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.asr_imm(self.rd, self.rn, self.imm_val, self.extra_op.clone())
    }
}

impl Core<'_, '_, '_> {
    /// Processes the ARM64 command `asr xd, xn, xm` with optional shift
    pub fn asr(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        xm: RegisterType,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;
        let (xm_val, _) = self.cpu.handle_extra_op(
            self.cpu.read_gen_reg(&xm)?,
            xm,
            xm.get_bitwidth(),
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, xn_val >> xm_val)?;
        Ok(())
    }

    /// Processes the ARM64 command `asr xd, xn, imm` with optional shift
    pub fn asr_imm(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        imm: i64,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;
        let (imm_val, _) = self.cpu.handle_extra_op(
            imm,
            xn,
            crate::processor::arithmetic_utils::IMMEDIATE_BITWIDTH,
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, xn_val >> imm_val)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_lsr_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov x1, #-1"))?;
    core.handle_string_command(&String::from("asr x1, x1, #1"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, -1);
    Ok(())
}
