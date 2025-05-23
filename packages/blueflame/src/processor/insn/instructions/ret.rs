use crate::processor::Error;
use crate::Core;

use crate::processor::{instruction_registry::RegisterType, RegisterValue};

impl Core<'_, '_, '_> {
    // NOTE: Seems to function the same as br, but has a "hint" that this is a subroutine return
    pub fn ret_with_arg(&mut self, xn: RegisterType) -> Result<(), Error> {
        let xn_val = match crate::Processor::read_reg(self.cpu, &xn) {
            RegisterValue::XReg(v) => v,
            _ => return Err(Error::InvalidRegisterRead("return address", xn)),
        };

        let new_pc = xn_val - 4;
        self.cpu.pc = new_pc as u64;
        let _ = self.cpu.stack_trace.pop();
        Ok(())
    }

    pub fn ret(&mut self) {
        // X30 is the return address
        self.cpu.pc = self.cpu.x[30] as u64 - 4;
        let _ = self.cpu.stack_trace.pop();
    }
}

#[cfg(test)]
#[test]
pub fn simple_ret_test() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(30), 5)?;
    core.handle_string_command(&String::from("ret"))?;
    assert_eq!(core.cpu.pc, 5);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn simple_ret_test_with_arg() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.pc = 1000;
    core.cpu.write_gen_reg(&RegisterType::XReg(10), 0x50)?;
    core.handle_string_command(&String::from("ret x10"))?;
    assert_eq!(core.cpu.pc, 0x50);
    Ok(())
}
