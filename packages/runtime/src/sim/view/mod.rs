mod pouch;
pub use pouch::*;
mod gdt;
pub use gdt::*;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("failed to read state from memory")]
    Memory(blueflame::memory::Error),
    #[error("coherence check failed when reading state")]
    Coherence(String),
}

impl From<Error> for crate::error::RuntimeViewError {
    fn from(value: Error) -> Self {
        match value {
            Error::Memory(_) => Self::Memory,
            Error::Coherence(_) => Self::Coherence,
        }
    }
}

macro_rules! try_mem {
    ($op:expr, $error:ident, $format:literal) => {
        match $op {
            Ok(x) => x,
            Err($error) => {
                log::error!($format);
                return Err(Error::Memory($error));
            }
        }
    };
}
pub(crate) use try_mem;

macro_rules! coherence_error {
    ($($args:tt)*) => {{
        let msg = format!($($args)*);
        log::error!("{msg}");
        return Err(Error::Coherence(msg));
    }}
}
pub(crate) use coherence_error;
