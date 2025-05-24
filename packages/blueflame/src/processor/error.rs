use super::Cpu0;


#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("new cache at main+0x{new_start:08x} overlaps with existing cache at main+0x{existing_start:08x} (this is a bug)")]
    ExecuteCacheOverlap {
        new_start: u32,
        existing_start: u32,
    },
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



    #[error("Unhandled extra-op: {0}")]
    UnhandledExtraOp(String),
    #[error("Unrecognized conditional code: {0}")]
    UnhandledConditionCode(String),
    #[error("Instruction could not be read at address {0:#0x}")]
    InstructionCouldNotBeRead(u64),
    // TODO --cleanup: RegisterType
    // #[error("Cannot read {0} value from register {1:?}")]
    // InvalidRegisterRead(&'static str, RegisterType),
    // #[error("Cannot write {0} value to register {1:?}")]
    // InvalidRegisterWrite(&'static str, RegisterType),

    #[error("Memory error: {0}")]
    Mem(crate::memory::Error),
    #[error("Instruction emitted an error: {0}")]
    InstructionError(String),
    #[error("Unexpected: {0}")]
    Unexpected(String),
}

impl From<crate::memory::Error> for Error {
    fn from(err: crate::memory::Error) -> Self {
        Error::Mem(err)
    }
}

pub struct CrashReport {
    pub cpu: Cpu0,
    pub error: Error
}
