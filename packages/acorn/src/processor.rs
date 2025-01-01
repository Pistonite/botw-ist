use crate::registers::Registers;


pub struct Processor {
    pub registers: Registers,
    pub pc: u64,
    pub flags: Flags,
}

/// Condition flags
pub struct Flags {
    /// Negative
    pub n: bool,
    /// Zero
    pub z: bool,
    /// Carry
    pub c: bool,
    /// Overflow
    pub v: bool,
}
