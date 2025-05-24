use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_cinc(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let cond = collected_args[2].clone();

        Ok(Box::new(CincInstruction { rd, rn, cond }))
    }



#[derive(Clone)]
pub struct CincInstruction {
    rd: RegisterType,
    rn: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CincInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cinc(self.rd, self.rn, &self.cond)
    }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `cinc rd, rn, condition`
    ///
    /// Evaluates the ternary `condition ? rd = rn + 1 : rd = rn`
    pub fn cinc(
        &mut self,
        rd: RegisterType,
        rn: RegisterType,
        condition: &str, // A condition code like eq, ne, etc.
    ) -> Result<(), Error> {
        let value = self.cpu.read_gen_reg(&rn)?;
        if self.cpu.flags.does_condition_succeed(condition)? {
            self.cpu.write_gen_reg(&rd, value + 1)?;
        } else {
            self.cpu.write_gen_reg(&rd, value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn test_cinc_when_true() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = true;
    core.handle_string_command(&String::from("mov x2, #10"))?;
    core.handle_string_command(&String::from("cinc x1, x2, EQ"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, 11);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn test_cinc_when_false() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = false;
    core.handle_string_command(&String::from("mov x2, #10"))?;
    core.handle_string_command(&String::from("cinc x1, x2, EQ"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, 10);
    Ok(())
}
