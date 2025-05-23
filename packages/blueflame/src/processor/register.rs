
pub type RegIndex = u32;
#[derive(Clone, Copy, Debug)]
pub enum RegisterType {
    // 64 bit int
    XReg(RegIndex),
    // 32 bit int (lower 32 of X)
    WReg(RegIndex),

    BReg(RegIndex),
    HReg(RegIndex),
    SReg(RegIndex),
    DReg(RegIndex),
    QReg(RegIndex),

    XZR,
    WZR,
    SP,
    LR,
}

pub struct Registers {
    // https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Registers-in-AArch64-state

    /// 31 general-purpose registers (called X0-X30 in code)
    /// X30 is used as the link register (LR)
    pub x: [i64; 31],
    /// 32 floating-point registers (called S0-S31 in code) - 128 bits each
    pub s: [f64; 32],
    // 4 stack pointer registers
    // https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Stack-Pointer-register?lang=en
    // sp_el0 is the classic "SP" register
    // The other 3 are stack pointers for each of the 3 exception levels
    // This is why the exception link/program status registers go from 1-3 rather than 0-2
    pub sp_el0: u64,

    pub pc: u64,

    pub flags: Flags,
    // == the exception registers are unused at the moment ==
    // _sp_el1: u64,
    // _sp_el2: u64,
    // _sp_el3: u64,
    // // 3 exception link registers
    // _elr_el1: i64,
    // _elr_el2: i64,
    // _elr_el3: i64,
    // // 3 saved program status registers (these are 32 bits)
    // _spsr_el1: i32,
    // _spsr_el2: i32,
    // _spsr_el3: i32,
    // // Floating point status control register
    // _fpscr: i32,
}

impl Registers {
    pub fn set_lr(&mut self, lr: u64) {
        self.x[30] = lr as i64;
    }
}

#[derive(Default, Debug)]
pub struct Flags {
    // Condition flags
    pub n: bool, // Negative
    pub z: bool, // Zero
    pub c: bool, // Carry
    pub v: bool, // Overflow
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_nzcv(nzcv: u8) -> Self {
        Flags {
            n: (nzcv & 0b1000) != 0,
            z: (nzcv & 0b0100) != 0,
            c: (nzcv & 0b0010) != 0,
            v: (nzcv & 0b0001) != 0,
        }
    }
}

impl Flags {
    pub fn does_condition_succeed(&self, cond: &str) -> bool {
        match cond.to_lowercase().as_str() {
            "eq" => self.check_eq(),
            "hs" => self.check_hs(),
            "lo" => self.check_lo(),
            "vc" => self.check_vc(),
            "le" => self.check_le(),
            "pl" => self.check_pl(),
            "mi" => self.check_mi(),
            "ne" => self.check_ne(),
            "hi" => self.check_hi(),
            "lt" => self.check_lt(),
            "ls" => self.check_ls(),
            "gt" => self.check_gt(),
            "ge" => self.check_ge(),
            "vs" => self.check_vs(),
            "cs" => self.check_cs(),
            "cc" => self.check_lo(), // carry clear
            // TODO --cleanup: change condition code to enum
            _ => panic!(
                "Unhandled condition code: {}",
                cond
            ),
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
mod tests {
    // TODO --cleanup: enable these tests
    // #[test]
    // pub fn eq_condional_passes() -> anyhow::Result<()> {
    //     use crate::processor::instruction_registry::RegisterType;
    //
    //     let mut cpu = crate::Processor::default();
    //     let mut mem = crate::Memory::new_empty_mem(0x10000);
    //     let mut proxies = crate::Proxies::default();
    //     let mut core = crate::Core {
    //         cpu: &mut cpu,
    //         mem: &mut mem,
    //         proxies: &mut proxies,
    //     };
    //     core.cpu.flags.z = true;
    //     core.handle_string_command(&String::from("add x9, xzr, #1"))?;
    //     core.handle_string_command(&String::from("add.eq x9, x9, #1"))?;
    //     assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(9))?, 2);
    //     Ok(())
    // }
    //
    // #[test]
    // pub fn eq_condional_fails() -> anyhow::Result<()> {
    //     use crate::processor::instruction_registry::RegisterType;
    //
    //     let mut cpu = crate::Processor::default();
    //     let mut mem = crate::Memory::new_empty_mem(0x10000);
    //     let mut proxies = crate::Proxies::default();
    //     let mut core = crate::Core {
    //         cpu: &mut cpu,
    //         mem: &mut mem,
    //         proxies: &mut proxies,
    //     };
    //     core.cpu.flags.z = false;
    //     core.handle_string_command(&String::from("add x9, xzr, #1"))?;
    //     core.handle_string_command(&String::from("add.eq x9, x9, #1"))?;
    //     assert_eq!(core.cpu.read_gen_reg(&RegisterType::XReg(9))?, 1);
    //     Ok(())
    // }
}
