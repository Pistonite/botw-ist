use crate::processor::{self as self_, crate_};

use crate_::env::no_panic;

pub use blueflame_macros::reg;

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

/// Wrapper for naming a register
///
/// This has nothing to do with ARM representation, just
/// our own internal representation.
///
/// The format is
/// ```text
/// MS                LS
/// Type Width Index
/// T    WW    IIIII
/// ```
///
/// - Index: 0-31
/// - Width: 
///   - 0 is 32 bits, 1 is 64 bits, 2 is 128 bits (FP/SIMD only)
/// - Type:
///   - 0 = General Purpose (GP, or X/W registers)
///   - 1 = Vector (FP/SIMD, or S/D/B/H/Q registers)
///
/// X30 is the link register (LR)
///
/// If index is 31 and type is 0, then it's the special registers depending on Width:
/// - 0: WZR (zero register)
/// - 1: XZR (zero register)
/// - 2: SP
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RegName(u8);

impl std::fmt::Display for RegName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_fp() {
            return match self.width_field() {
                0 => write!(f, "s{}", self.idx()),
                1 => write!(f, "d{}", self.idx()),
                _ => write!(f, "q{}", self.idx()),
            };
        }
        if self.is_special() {
            return match self.width_field() {
                0 => write!(f, "wzr"),
                1 => write!(f, "xzr"),
                _ => write!(f, "sp"),
            };
        }
        match self.width_field() {
            0 | 2 => write!(f, "w{}", self.idx()),
            _ => write!(f, "x{}", self.idx()),
        }
    }
}

impl std::fmt::Debug for RegName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({:#08b})", self, self.0)
    }
}

impl RegName {
    #[inline(always)]
    pub const fn sp() -> Self {
        Self(0b01011111)
    }
    #[inline(always)]
    pub const fn xzr() -> Self {
        Self(0b00111111)
    }
    #[inline(always)]
    pub const fn wzr() -> Self {
        Self(0b00011111)
    }
    #[inline(always)]
    pub const fn w(idx: u8) -> Self {
        #[allow(clippy::identity_op)]
        Self(0b00000000 | (idx & 0b11111))
    }
    #[inline(always)]
    pub const fn x(idx: u8) -> Self {
        Self(0b00100000 | (idx & 0b11111))
    }
    #[inline(always)]
    pub const fn s(idx: u8) -> Self {
        Self(0b10000000 | (idx & 0b11111))
    }
    #[inline(always)]
    pub const fn d(idx: u8) -> Self {
        Self(0b10100000 | (idx & 0b11111))
    }
    #[inline(always)]
    pub const fn q(idx: u8) -> Self {
        Self(0b11000000 | (idx & 0b11111))
    }
    #[inline(always)]
    pub const fn is_gp(self) -> bool {
        self.0 & 0b10000000 == 0b00000000
    }
    #[inline(always)]
    pub const fn is_fp(self) -> bool {
        !self.is_gp()
    }
    #[inline(always)]
    pub const fn is_special(self) -> bool {
        self.idx() == 31 && self.is_gp()
    }
    /// If the qbit is 1:
    /// - If the register is GP, then it's SP
    /// - If the register is FP, then it's a Q register
    #[inline(always)]
    pub const fn qbit(self) -> bool {
        self.0 & 0b01000000 == 0b01000000
    }
    #[inline(always)]
    pub const fn is_64_bits(self) -> bool {
        self.0 & 0b00100000 == 0b00100000
    }
    /// Get the internal register index
    #[inline(always)]
    pub const fn idx(self) -> u8 {
        self.0 & 0b11111
    }

    /// Get the width field (0-3)
    #[inline(always)]
    pub const fn width_field(self) -> u8 {
        (self.0 >> 5) & 0b11
    }
}

#[derive(Debug, Default, Clone)]
pub struct Registers {
    // https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Registers-in-AArch64-state

    /// 31 general-purpose registers (called X0-X30 in code)
    /// X30 is used as the link register (LR)
    /// X31 is SP in our code, see below
    x: [u64; 32],
    /// 32 floating-point registers (called S0-S31 in code) - 128 bits each
    v: [u64; 32],
    /// upper 64 bits of FP registers. They are rarely used, so putting
    /// the lower 64 bits together in memory is probably better
    q: [u64; 32],

    // PC is not named
    pub pc: u64,

    // 4 stack pointer registers
    // https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Stack-Pointer-register?lang=en
    // sp_el0 is the classic "SP" register and is X31 in our representation
    // The other 3 are stack pointers for each of the 3 exception levels
    // This is why the exception link/program status registers go from 1-3 rather than 0-2

    /// NZCV register
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

pub trait RegValue {
    fn from_32(x: u32) -> Self;
    fn from_64(x: u64) -> Self;
    fn from_128(lo: u64, hi: u64) -> Self;
    fn into_64(self) -> u64;
    fn into_128(self) -> (u64, u64);
}

#[rustfmt::skip]
impl RegValue for bool {
    #[inline(always)] fn from_32(x: u32) -> Self { x & 1 != 0 }
    #[inline(always)] fn from_64(x: u64) -> Self { x & 1 != 0 }
    #[inline(always)] fn from_128(x: u64, _: u64) -> Self { x & 1 != 0 }
    #[inline(always)] fn into_64(self) -> u64 { if self { 1 } else { 0 } }
    #[inline(always)] fn into_128(self) -> (u64, u64) { (if self { 1 } else { 0 }, 0) }
}

macro_rules! impl_reg_value {
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.int-truncation
    // Read: Truncate/ZeroExtend
    // Write: Truncate/ZeroExtend
    ($type:ty, unsigned) => {
        impl RegValue for $type {
            #[inline(always)] fn from_32(x: u32) -> Self { x as Self }
            #[inline(always)] fn from_64(x: u64) -> Self { x as Self }
            #[inline(always)] fn from_128(x: u64, _: u64) -> Self { x as Self }
            #[inline(always)] fn into_64(self) -> u64 { self as u64 }
            #[inline(always)] fn into_128(self) -> (u64, u64) { (self as u64, 0) }
        }
    };
    // Read: SignExtend (for example if you read a i64 from a W register, it can be negative)
    // Write: UpperZeroed
    ($type:ty, signed, $unsigned_type:ty) => {
        impl RegValue for $type {
            #[inline(always)] fn from_32(x: u32) -> Self { x as i32 as Self }
            #[inline(always)] fn from_64(x: u64) -> Self { x as i64 as Self }
            #[inline(always)] fn from_128(x: u64, _: u64) -> Self { Self::from_64(x) }
            #[inline(always)] fn into_64(self) -> u64 { self as $unsigned_type as u64 }
            #[inline(always)] fn into_128(self) -> (u64, u64) { (self.into_64() , 0) }
        }
    };
}
impl_reg_value!(u8, unsigned);
impl_reg_value!(u16, unsigned);
impl_reg_value!(u32, unsigned);
impl_reg_value!(u64, unsigned);
impl_reg_value!(i8, signed, u8);
impl_reg_value!(i16, signed, u16);
impl_reg_value!(i32, signed, u32);
impl_reg_value!(i64, signed, u64);

#[rustfmt::skip]
impl RegValue for f32 {
    #[inline(always)] fn from_32(x: u32) -> Self { f32::from_bits(x) }
    // this is really invalid operation, so we can do whatever
    // current implementation is only use the lower 32 bits
    #[inline(always)] fn from_64(x: u64) -> Self { f32::from_bits(x as u32) }
    #[inline(always)] fn from_128(x: u64, _: u64) -> Self { f32::from_bits(x as u32) }
    // zero upper bits
    #[inline(always)] fn into_64(self) -> u64 { self.to_bits() as u64 }
    #[inline(always)] fn into_128(self) -> (u64, u64) { (self.to_bits() as u64, 0) }
}
#[rustfmt::skip]
impl RegValue for f64 {
    // this DOES make sense - reading as f32 then casting to f64
    #[inline(always)] fn from_32(x: u32) -> Self { f32::from_bits(x) as Self }
    #[inline(always)] fn from_64(x: u64) -> Self { f64::from_bits(x) }
    // this is really invalid operation, so we can do whatever
    // current implementation is only use the lower 64 bits
    #[inline(always)] fn from_128(x: u64, _: u64) -> Self { f64::from_bits(x) }
    #[inline(always)] fn into_64(self) -> u64 { self.to_bits() }
    // zero upper bits
    #[inline(always)] fn into_128(self) -> (u64, u64) { (self.to_bits() , 0) }
}

// no current implementation for 128 bit values yet

impl Registers {
    /// Read a register
    ///
    /// For performance, there is no error checking here in release mode,
    /// it's undefined behavior if the register is not valid
    #[no_panic]
    pub fn read<R: RegValue>(&self, name: RegName) -> R {
        let i = name.idx();
        match (name.is_fp(), name.qbit()) {
            (true, true) => {
                RegValue::from_128(self.v[i as usize], self.q[i as usize])
            }
            (true, false) => {
                match name.is_64_bits() {
                    true => RegValue::from_64(self.v[i as usize]),
                    false => RegValue::from_32(self.v[i as usize] as u32),
                }
            }
            (false, qbit) => {
                if i == 31 {
                    if !qbit {
                        // zero
                        return RegValue::from_64(0)
                    }
                    debug_assert!(name == RegName::sp(), "invalid register");
                }
                // named GP or SP
                match name.is_64_bits() {
                    true => RegValue::from_64(self.x[i as usize]),
                    false => RegValue::from_32(self.x[i as usize] as u32),
                }
            }
        }
    }
    /// Write a register
    #[no_panic]
    pub fn write<R: RegValue>(&mut self, name: RegName, value: R) {
        let i = name.idx();
        match (name.is_fp(), name.qbit()) {
            (true, true) => {
                let (lo, hi) = value.into_128();
                log::debug!("reg write: {name:?} <-[{i}] q={lo}, v={hi}");
                self.q[i as usize] = hi;
                self.v[i as usize] = lo;
            }
            (true, false) => {
                let mut x = value.into_64();
                if !name.is_64_bits() {
                    x = x as u32 as u64; // truncate
                }
                log::debug!("reg write: {name:?} <-[{i}] v={x}");
                self.v[i as usize] = x;
            }
            (false, qbit) => {
                if i == 31 {
                    if !qbit {
                        // writing to zero
                        return;
                    }
                    debug_assert!(name == RegName::sp(), "invalid register");
                }
                // named GP or SP
                let mut x = value.into_64();
                if !name.is_64_bits() {
                    x = x as u32 as u64; // truncate
                }
                log::debug!("reg write: {name:?} <-[{i}] x={x}");
                self.x[i as usize] = x
            }
        }
    }

    pub const fn inc_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4);
    }

    /// Check if the condition specified by `cond` is true in the nzcv register
    pub fn check_condition(&self, cond: &str) -> bool {
        self.flags.does_condition_succeed(cond)
    }
}

#[derive(Default, Debug, Clone, Copy)]
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
                "Unhandled condition code: {cond}",
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
