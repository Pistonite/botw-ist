use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;

    fn parse_cbz(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let split = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&split[0])?;
        let label_offset = Self::get_label_val(&split[1])?;
        Ok(Box::new(CbzInstruction { rn, label_offset }))
    }

#[derive(Clone)]
pub struct CbzInstruction {
    rn: RegisterType,
    label_offset: u64,
}

impl ExecutableInstruction for CbzInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.cbz(self.rn, self.label_offset)
    }
}

impl Core<'_, '_, '_> {
    /// Processes the ARM64 command `cbnz xn, label`
    ///
    /// Branches to a PC relative label if xn is zero
    pub fn cbz(&mut self, xn: RegisterType, label_offset: u64) -> Result<(), Error> {
        let xn_val = self.cpu.read_gen_reg(&xn)?;

        if xn_val == 0 {
            let new_pc = self.cpu.pc.wrapping_add_signed((label_offset - 4) as i64);
            self.cpu.pc = new_pc;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn cbz_withzero_dobranch() -> anyhow::Result<()> {
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
    core.cpu.write_gen_reg(&RegisterType::XReg(10), 0)?;
    core.handle_string_command(&String::from("cbz x10, 50"))?;
    assert_eq!(core.cpu.pc, 0x1050);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn cbz_withnonzero_donotbranch() -> anyhow::Result<()> {
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
    core.cpu.write_gen_reg(&RegisterType::XReg(10), 2)?;
    core.handle_string_command(&String::from("cbz x10, 50"))?;
    assert_eq!(core.cpu.pc, 0x1004);
    Ok(())
}
