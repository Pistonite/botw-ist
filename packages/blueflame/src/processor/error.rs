use derive_more::derive::Constructor;

#[layered_crate::import]
use processor::{
    super::memory,
    super::env::DataId,
    self::{Cpu0, reg}
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("missing required data {0:?}")]
    MissingData(DataId),
    #[error("new cache at main+0x{new_start:08x} overlaps with existing cache at main+0x{existing_start:08x} (this is a bug)")]
    ExecuteCacheOverlap {
        new_start: u32,
        existing_start: u32,
    },
    #[error("hook at main+0x{0:08x} is too big (this is a bug)")]
    TooBigHook(u32),
    #[error("[proc-strict-replace-hook] unsupported jump to middle of replaced code at main+0x{main_offset:08x}")]
    StrictReplacement {
        main_offset: u32,
    },
    #[error("[limited-block-count] block count limit reached")]
    BlockCountLimitReached,
    #[error("[limited-block-iteration] block iteration limit reached")]
    BlockIterationLimitReached,
    #[error("[check-stack-frames] stack frames are corrupted")]
    StackFrameCorrupted,
    #[error("[check-return-address] return address 0x{0:016x} does not match expected 0x{1:016x}")]
    ReturnAddressMismatch(u64, u64),
    #[error("[instruction-abort] bad instruction 0x{0:08x}")]
    BadInstruction(u32),
    #[error("[check-stack-corruption] stack object at 0x{0:016x} with size 0x{1:x} is corrupted")]
    StackCorruption(u64, u32),



    #[error("Unrecognized conditional code: {0}")]
    UnhandledConditionCode(String),
    #[error("Instruction could not be read at address {0:#0x}")]
    InstructionCouldNotBeRead(u64),

    #[error("Memory error: {0}")]
    Memory(#[from] memory::Error),

    #[error("Unexpected: {0}")]
    Unexpected(String),
}

#[derive(Constructor, Clone)]
pub struct CrashReport {
    pub cpu: Cpu0,
    pub main_start: u64,
    pub error: Error
}

impl std::fmt::Display for CrashReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BlueFlame Core Crash (use {{:?}} to see details)")?;

        Ok(())
    }
}

impl std::error::Error for CrashReport {}
impl std::fmt::Debug for CrashReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== BLUEFLAME CRASH REPORT ===")?;
        writeln!(f, "Cause: {}", self.error)?;
        writeln!(f, "")?;
        writeln!(f, "Registers:")?;
        for i in 0..16 {
            let i2 = i + 16;
            let reg1 = reg!(x[i]);
            let reg2 = reg!(d[i]);
            let reg3 = if i2 < 31 { reg!(x[i2]) } else { reg!(sp) };
            let reg4 = reg!(d[i2]);
            let x: u64 = self.cpu.read(reg1);
            let v: u64 = self.cpu.read(reg2);
            let x2: u64 = self.cpu.read(reg3);
            let v2: u64 = self.cpu.read(reg4);
            let reg1 = format!("{:4}", reg1.to_string());
            let reg2 = format!("{:4}", reg2.to_string());
            let reg3 = format!("{:4}", reg3.to_string());
            let reg4 = format!("{:4}", reg4.to_string());
            // don't show Q regs right now, probably not important
            writeln!(f, "  {reg1}= 0x{x:016x}  {reg3}= 0x{x2:016x}  {reg2}= 0x{v:016x}  {reg4}= 0x{v2:016x}")?;
        }

        writeln!(f, "")?;
        writeln!(f, "Main Start: 0x{:016x}", self.main_start)?;
        writeln!(f, "PC: {}", format_address(self.cpu.pc, self.main_start))?;
        writeln!(f, "LR: {}", format_address(self.cpu.read::<u64>(reg!(lr)), self.main_start))?;
        writeln!(f, "Stack Trace: (top is most recent)")?;
        writeln!(f, "{}", self.cpu.stack_trace.format_with_main_start(self.main_start))?;

        Ok(())
    }
}

pub fn format_address(addr: u64, main_start: u64) -> String {
    if main_start <= addr {
        format!("0x{:016x} (main+0x{:08x})", addr, addr - main_start)
    } else {
        format!("0x{:016x}                ", addr)
    }
}
