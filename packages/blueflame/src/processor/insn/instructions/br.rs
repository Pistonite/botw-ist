use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;

    fn parse_br(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let rn = RegisterType::from_str(args)?;
        Ok(Box::new(BrInstruction { rn }))
    }

#[derive(Clone)]
pub struct BrInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for BrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.br(self.rn)
    }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `br xn`
    pub fn br(&mut self, xn: RegisterType) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)? as u64;

        self.cpu.pc = xn_val - 4;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_br_test() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 0x1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(10), 0x50)?;
    core.handle_string_command(&String::from("br x10"))?;
    assert_eq!(core.cpu.pc, 0x50);
    Ok(())
}
