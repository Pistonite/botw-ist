use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;


    fn parse_blr(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let rn = RegisterType::from_str(args)?;
        Ok(Box::new(BlrInstruction { rn }))
    }

#[derive(Clone)]
pub struct BlrInstruction {
    rn: RegisterType,
}

impl ExecutableInstruction for BlrInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.blr(self.rn)
    }

    // fn instruction_type(&self) -> Option<InstructionType> {
    //     Some(InstructionType::Branch)
    // }
}

impl Core<'_, '_, '_> {
    /// Processes ARM64 command `blr xn`
    pub fn blr(&mut self, xn: RegisterType) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)? as u64 - 4;
        let lr = self.cpu.pc + 4;

        self.cpu.stack_trace.push((
            self.compute_ida_addr(self.cpu.pc),
            self.compute_ida_addr(xn_val + 4),
        ));
        self.cpu.pc = xn_val;
        self.cpu.x[30] = lr as i64;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn simple_blr_test() -> anyhow::Result<()> {
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
    core.cpu.write_gen_reg(&RegisterType::XReg(30), 5)?;
    core.cpu.write_gen_reg(&RegisterType::XReg(10), 0x50)?;
    core.handle_string_command(&String::from("blr x10"))?;
    assert_eq!(core.cpu.pc, 0x50);
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(30))?, 0x1004);
    Ok(())
}
