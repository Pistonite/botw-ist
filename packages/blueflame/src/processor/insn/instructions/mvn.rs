use crate::processor::instruction_registry::{AuxiliaryOperation, RegisterType};

use crate::processor::Error;
use crate::Core;

    fn parse_mvn(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let extra_op = Self::parse_auxiliary(collected_args.get(3))?;
        Ok(Box::new(MvnInstruction { rd, rn, extra_op }))
    }


#[derive(Clone)]
pub struct MvnInstruction {
    rd: RegisterType,
    rn: RegisterType,
    extra_op: Option<AuxiliaryOperation>,
}

impl ExecutableInstruction for MvnInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.mvn(self.rd, self.rn, self.extra_op.clone())
    }
}


impl Core<'_, '_, '_> {
    pub fn mvn(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        extra_op: Option<AuxiliaryOperation>,
    ) -> Result<(), Error> {
        let (xn_val, _) = self.cpu.handle_extra_op(
            self.cpu.read_gen_reg(&xn)?,
            xn,
            xn.get_bitwidth(),
            extra_op,
        )?;
        self.cpu.write_gen_reg(&xd, !xn_val)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_mvn_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w1, #0"))?;
    core.handle_string_command(&String::from("mvn w2, w1"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::WReg(2))?, -1);
    Ok(())
}
