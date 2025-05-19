use crate::processor::Flags;

use super::Error;

impl Flags {
    pub fn does_condition_succeed(&self, cond: &str) -> Result<bool, Error> {
        match cond.to_lowercase().as_str() {
            "eq" => Ok(self.check_eq()),
            "hs" => Ok(self.check_hs()),
            "lo" => Ok(self.check_lo()),
            "vc" => Ok(self.check_vc()),
            "le" => Ok(self.check_le()),
            "pl" => Ok(self.check_pl()),
            "mi" => Ok(self.check_mi()),
            "ne" => Ok(self.check_ne()),
            "hi" => Ok(self.check_hi()),
            "lt" => Ok(self.check_lt()),
            "ls" => Ok(self.check_ls()),
            "gt" => Ok(self.check_gt()),
            "ge" => Ok(self.check_ge()),
            "vs" => Ok(self.check_vs()),
            "cs" => Ok(self.check_cs()),
            "cc" => Ok(self.check_lo()), // carry clear
            _ => Err(Error::UnhandledConditionCode(cond.to_lowercase())),
        }
    }

    fn check_eq(&self) -> bool {
        self.z
    }

    fn check_hs(&self) -> bool {
        self.c
    }

    fn check_lo(&self) -> bool {
        !self.c
    }

    fn check_vc(&self) -> bool {
        !self.v
    }

    fn check_le(&self) -> bool {
        self.z || self.n != self.v
    }

    fn check_pl(&self) -> bool {
        !self.n
    }

    fn check_mi(&self) -> bool {
        self.n
    }

    fn check_ne(&self) -> bool {
        !self.z
    }

    fn check_hi(&self) -> bool {
        self.c && !self.z
    }

    fn check_lt(&self) -> bool {
        self.n != self.v
    }

    fn check_ls(&self) -> bool {
        !self.c || self.z
    }

    fn check_gt(&self) -> bool {
        !self.z && self.n == self.v
    }

    fn check_ge(&self) -> bool {
        self.n == self.v
    }

    fn check_vs(&self) -> bool {
        self.v
    }

    fn check_cs(&self) -> bool {
        self.c
    }
}

#[cfg(test)]
#[test]
pub fn eq_condional_passes() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = true;
    core.handle_string_command(&String::from("add x9, xzr, #1"))?;
    core.handle_string_command(&String::from("add.eq x9, x9, #1"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(9))?, 2);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn eq_condional_fails() -> anyhow::Result<()> {
    use crate::processor::instruction_registry::RegisterType;

    let mut cpu = crate::Processor::default();
    let mut mem = crate::Memory::new_empty_mem(0x10000);
    let mut proxies = crate::Proxies::default();
    let mut core = crate::Core {
        cpu: &mut cpu,
        mem: &mut mem,
        proxies: &mut proxies,
    };
    core.cpu.flags.z = false;
    core.handle_string_command(&String::from("add x9, xzr, #1"))?;
    core.handle_string_command(&String::from("add.eq x9, x9, #1"))?;
    assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(9))?, 1);
    Ok(())
}
