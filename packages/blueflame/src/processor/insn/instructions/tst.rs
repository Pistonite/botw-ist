use crate::processor::Error;
use crate::Core;

use crate::processor::instruction_registry::RegisterType;

    fn parse_tst(args: &str) -> Result<Box<dyn ExecutableInstruction>> {
        let collected_args = Self::split_args(args, 2);
        let rn = RegisterType::from_str(&collected_args[0])?;

        if Self::is_imm(&collected_args[1]) {
            let imm_val = Self::get_imm_val(&collected_args[1])?;
            Ok(Box::new(TstImmInstruction { rn, imm_val }))
        } else {
            let rm = RegisterType::from_str(&collected_args[1])?;
            Ok(Box::new(TstInstruction { rn, rm }))
        }
    }


#[derive(Clone)]
pub struct TstInstruction {
    rn: RegisterType,
    rm: RegisterType,
}

impl ExecutableInstruction for TstInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.tst(self.rn, self.rm)
    }
}

#[derive(Clone)]
pub struct TstImmInstruction {
    rn: RegisterType,
    imm_val: i64,
}

impl ExecutableInstruction for TstImmInstruction {
    fn exec_on(&self, proc: &mut Core) -> Result<(), Error> {
        proc.tst_imm(self.rn, self.imm_val)
    }
}


impl Core<'_, '_, '_> {
    // Performs a bitwise and
    // Updates N and Z flags
    // Does not affect C and V flags; leaves them unchanged
    pub fn tst(&mut self, xn: RegisterType, xm: RegisterType) -> Result<(), Error> {
        let vn = self.cpu.read_gen_reg(&xn)?;
        let vm = self.cpu.read_gen_reg(&xm)?;
        let result = vn & vm;
        self.cpu.flags.n = result < 0;
        self.cpu.flags.z = result == 0;
        Ok(())
    }

    pub fn tst_imm(&mut self, xn: RegisterType, imm: i64) -> Result<(), Error> {
        let vn = self.cpu.read_gen_reg(&xn)?;
        let vm = imm;
        let result = vn & vm;
        self.cpu.flags.n = result < 0;
        self.cpu.flags.z = result == 0;
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn tst_zero_result() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    // Check that other flags are unaffected
    core.cpu.flags.v = true;
    core.cpu.flags.c = true;
    core.handle_string_command(&String::from("mov x1, #0"))?;
    core.handle_string_command(&String::from("mov x2, #111"))?;
    core.handle_string_command(&String::from("tst x1, x2"))?;
    assert!(core.cpu.flags.z);
    assert!(!core.cpu.flags.n);
    assert!(core.cpu.flags.c);
    assert!(core.cpu.flags.v);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn tst_negative_result() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    // Check that other flags are unaffected
    core.cpu.flags.v = true;
    core.cpu.flags.c = true;
    core.handle_string_command(&String::from("mov x1, #-1"))?;
    core.handle_string_command(&String::from("mov x2, #-700"))?;
    core.handle_string_command(&String::from("tst x1, x2"))?;
    assert!(!core.cpu.flags.z);
    assert!(core.cpu.flags.n);
    assert!(core.cpu.flags.c);
    assert!(core.cpu.flags.v);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn tst_positive_result() -> anyhow::Result<()> {
    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    // Check that other flags are unaffected
    core.cpu.flags.v = true;
    core.cpu.flags.c = true;
    core.handle_string_command(&String::from("mov x1, #1"))?;
    core.handle_string_command(&String::from("mov x2, #-701"))?;
    core.handle_string_command(&String::from("tst x1, x2"))?;
    assert!(!core.cpu.flags.z);
    assert!(!core.cpu.flags.n);
    assert!(core.cpu.flags.c);
    assert!(core.cpu.flags.v);
    Ok(())
}
