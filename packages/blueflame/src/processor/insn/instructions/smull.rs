use crate::processor::instruction_registry::RegisterType;

use crate::processor::Error;
use crate::Core;

    fn parse_smull(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 3);
        let rd = RegisterType::from_str(&collected_args[0])?;
        let wn = RegisterType::from_str(&collected_args[1])?;
        let wm = RegisterType::from_str(&collected_args[2])?;
        Ok(Box::new(SmullInstruction { rd, wn, wm }))
    }


#[derive(Clone)]
pub struct SmullInstruction {
    rd: RegisterType,
    wn: RegisterType,
    wm: RegisterType,
}

impl ExecutableInstruction for SmullInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.smull(self.rd, self.wn, self.wm)
    }
}


impl Core<'_, '_, '_> {
    pub fn smull(
        &mut self,
        xd: RegisterType,
        wn: RegisterType,
        wm: RegisterType,
    ) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&wn)?;
        let xm_val = self.cpu.read_gen_reg(&wm)?;
        self.cpu.write_gen_reg(&xd, xn_val * xm_val)?;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_smull_test() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.handle_string_command(&String::from("mov w1, #2"))?;
    core.handle_string_command(&String::from("mov w2, #3"))?;
    core.handle_string_command(&String::from("smull x4, w1, w2"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(4))?, 6);
    Ok(())
}
