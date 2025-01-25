#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {

    #[error("boot crash: {0}")]
    Boot(#[from] crate::boot::Error),

    #[error("execution error")]
    Cpu, // TODO: CPU errors

    #[error("memory error: {0}")]
    Mem(#[from] crate::memory::Error),
}
