#[derive(Debug, Clone, PartialEq)]
// https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Registers-in-AArch64-state
#[allow(dead_code)]
pub struct Registers {
    // 31 general-purpose registers (called X0-X30 in code)
    // X30 is used as the link register (LR)
    pub x: [i64; 31],
    // 32 floating-point registers (called S0-S31 in code) - 128 bits each
    pub s: [f64; 32],
    // 4 stack pointer registers
    // https://developer.arm.com/documentation/dui0801/l/Overview-of-AArch64-state/Stack-Pointer-register?lang=en
    // sp_el0 is the classic "SP" register
    // The other 3 are stack pointers for each of the 3 exception levels
    // This is why the exception link/program status registers go from 1-3 rather than 0-2
    pub sp_el0: u64,
    pub sp_el1: u64,
    pub sp_el2: u64,
    pub sp_el3: u64,
    // 3 exception link registers
    pub elr_el1: i64,
    pub elr_el2: i64,
    pub elr_el3: i64,
    // 3 saved program status registers (these are 32 bits)
    pub spsr_el1: i32,
    pub spsr_el2: i32,
    pub spsr_el3: i32,
    // Floating point status control register
    pub fpscr: i32,
}
