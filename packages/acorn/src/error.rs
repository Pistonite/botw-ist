use crate::{registers::Registers, PageTable};


pub struct Crash {
    /// The instruction location that is being
    /// executed when the crash happened
    pub pc: u64,

    pub registers: Registers,

    pub reason: CrashReason,

    /// If memory dump on crash (:enable memory-dump) is enabled, this will contain the memory
    /// snapshot at the time of the crash
    pub memory: Option<PageTable>,
}

pub enum CrashReason {
    /// Trying to execute an instruction that is bad/malformed
    BadInstruction(u32),
    /// Trying to execute an instruction that is recognized, but not
    /// supported on the (emulated) hardware
    Unsupported(u32),
    /// Trying to execute an instruction that is recognized but not implemented
    Unimplemented(u32),
    /// The program does not have permission to execute the instruction.
    ///
    /// Suppressed by :enable allow-privileged
    PrivilegeRequired(u32),
    /// Trying to access an unaligned address
    ///
    /// Suppressed by :disable memory-alignment, except when
    /// the access is across a page boundary, in which case the exception
    /// will still occur
    Unaligned(MemAccess),
    /// Trying to access an address without the proper permissions
    ///
    /// Suppressed by :disable memory-permissions, in which case
    /// all memory addresses have full RWX permissions
    PermissionDenied(MemAccess),
    /// Trying to read from an unallocated page (for example, a NULL pointer)
    ///
    /// Suppressed by :disable memory-faults, in which
    /// case 0s should be returned by memory reads
    PageFault(MemAccess),
    /// Error during arithmetic operations
    Arithmetic(ArithmeticError),
}

pub enum ArithmeticError {
    /// Integer division by zero
    ///
    /// Suppressed by :disable divide-by-zero
    DivideByZero,
    /// Integer Overflow
    ///
    /// Disabled by default. Enable with :enable integer-bounds
    Overflow,
    /// Integer Underflow
    ///
    /// Disabled by default. Enable with :enable integer-bounds
    Underflow,
}

/// Information for accessing memory
pub struct MemAccess {
    pub access_type: MemAccessType,
    pub addr: u64,
    pub bytes: u32,
}

pub enum MemAccessType {
    /// Reading data from the memory
    Read,
    /// Writing data to the memory
    Write,
    /// Reading instruction from memory
    Execute,
}
