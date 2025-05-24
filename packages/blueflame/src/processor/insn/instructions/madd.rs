use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_madd(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let xa = RegisterType::from_str(&collected_args[3])?;
        Ok(Box::new(MaddInstruction { rd, rn, rm, xa }))
    }


#[derive(Clone)]
pub struct MaddInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    xa: RegisterType,
}

impl ExecutableInstruction for MaddInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.madd(self.rd, self.rn, self.rm, self.xa)
    }
}

impl Core<'_, '_, '_> {
    pub fn madd(
        &mut self,
        xd: RegisterType,
        xn: RegisterType,
        xm: RegisterType,
        xa: RegisterType,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;
        let xm_val = self.cpu.read_gen_reg(&xm)?;
        let xa_val = self.cpu.read_gen_reg(&xa)?;
        self.cpu.write_gen_reg(&xd, xn_val * xm_val + xa_val)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_madd_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov x1, #2"))?;
    core.handle_string_command(&String::from("mov x2, #3"))?;
    core.handle_string_command(&String::from("mov x3, #4"))?;
    core.handle_string_command(&String::from("madd x4, x1, x2, x3"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(4))?, 10);
    Ok(())
}
