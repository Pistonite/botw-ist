use std::fmt::Debug;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("boot crash: {0}")]
    Boot(#[from] crate::boot::Error),

    #[error("CPU error")]
    Cpu(crate::processor::Error),

    #[error("memory error: {0}")]
    Mem(crate::memory::Error),
}

impl From<crate::memory::Error> for Error {
    fn from(err: crate::memory::Error) -> Self {
        Error::Mem(err)
    }
}

impl From<crate::processor::Error> for Error {
    fn from(err: crate::processor::Error) -> Self {
        if let crate::processor::Error::Mem(m) = err {
            Error::Mem(m)
        } else {
            Error::Cpu(err)
        }
    }
}

#[derive(thiserror::Error)]
pub struct ExecutionError {
    error: Error,
    ida_address: u64,
    stack_trace: Vec<(u64, u64)>,
}
impl ExecutionError {
    pub fn new(error: Error, ida_address: u64, stack_trace: Vec<(u64, u64)>) -> Self {
        ExecutionError {
            error,
            ida_address,
            stack_trace,
        }
    }
}
impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0} @{1:#0x} (IDA)\n\tstack trace:\n{2}",
            self.error,
            self.ida_address,
            self.stack_trace
                .iter()
                .rev()
                .map(|a| format!("\t\t{1:#0x} (called from {0:#0x})", a.0, a.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
impl std::fmt::Debug for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
