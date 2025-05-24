use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_csinc(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 4);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let rn = RegisterType::from_str(&collected_args[1])?;
        let rm = RegisterType::from_str(&collected_args[2])?;
        let cond = collected_args[3].clone();

        Ok(Box::new(CsincInstruction { rd, rn, rm, cond }))
    }


#[derive(Clone)]
pub struct CsincInstruction {
    rd: RegisterType,
    rn: RegisterType,
    rm: RegisterType,
    cond: String,
}

impl ExecutableInstruction for CsincInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.csinc(self.rd, self.rn, self.rm, &self.cond)
    }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `csinc rd, rn, rm, condition`
    ///
    /// Evaluates the ternary `condition ? rd = rn : rd = rm + 1`
    pub fn csinc(
        &mut self,
        rd: RegisterType,
        rn: RegisterType,
        rm: RegisterType,
        condition: &str, // A condition code like eq, ne, etc.
    ) -> Result<(), Error> {
        if self.cpu.flags.does_condition_succeed(condition)? {
            let value = self.cpu.read_gen_reg(&rn)?;
            self.cpu.write_gen_reg(&rd, value)?;
        } else {
            let value = self.cpu.read_gen_reg(&rm)?;
            self.cpu.write_gen_reg(&rd, value + 1)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn test_csinc_when_true() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("mov x3, #20"))?;
    core.handle_string_command(&String::from("csinc x1, x2, x3, EQ"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, 10);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn test_csinc_when_false() -> anyhow::Result<()> {
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
    core.handle_string_command(&String::from("mov x3, #20"))?;
    core.handle_string_command(&String::from("csinc x1, x2, x3, EQ"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(1))?, 21);
    Ok(())
}
