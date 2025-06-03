pub use skybook_api::runtime::error::{MaybeAborted, RuntimeInitError};
pub use skybook_api::runtime::error::RuntimeError as Error;
pub type ErrorReport = skybook_api::ErrorReport<Error>;


pub struct Report<T> {
    pub value: T,
    pub errors: Vec<ErrorReport>,
}
impl<T> Report<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            errors: Vec::new(),
        }
    }

    pub fn with_errors(value: T, errors: Vec<ErrorReport>) -> Self {
        Self { value, errors }
    }

    pub fn push(&mut self, error: ErrorReport) {
        self.errors.push(error);
    }

}

// mod 

// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//     #[error("executor error: {0}")]
//     Executor(#[from] crate::exec::Error),
// }
macro_rules! sim_error {
    ($span:expr, $error_type:ident) => {
        $crate::error::ErrorReport::error(
            $span,
            $crate::error::Error::$error_type,
        )
    };
    ($span:expr, $error_type:ident ( $($args:expr),* $(,)? )) => {
        $crate::error::ErrorReport::error(
            $span,
            $crate::error::Error::$error_type($($args),*),
        )
    };
}
pub(crate) use sim_error;
macro_rules! sim_warning {
    ($span:expr, $error_type:ident) => {
        $crate::error::ErrorReport::warning(
            $span,
            $crate::error::Error::$error_type,
        )
    };
    ($span:expr, $error_type:ident ( $($args:expr),* $(,)? )) => {
        $crate::error::ErrorReport::warning(
            $span,
            $crate::error::Error::$error_type($($args),*),
        )
    };
}
pub(crate) use sim_warning;
