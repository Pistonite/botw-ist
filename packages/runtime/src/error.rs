use teleparse::Span;

pub use skybook_api::runtime::error::RuntimeError as Error;
pub use skybook_api::runtime::error::{MaybeAborted, RuntimeInitError, RuntimeViewError};
pub type ErrorReport = skybook_api::ErrorReport<Error>;

#[derive(Debug, Clone)]
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

    pub fn map<U, F>(self, f: F) -> Report<U>
    where
        F: FnOnce(T) -> U,
    {
        Report {
            value: f(self.value),
            errors: self.errors,
        }
    }

    /// Create a new report with one error
    pub fn error(value: T, error: ErrorReport) -> Self {
        Self {
            value,
            errors: vec![error],
        }
    }

    pub fn spanned(value: T, span: &Span, error: Error) -> Self {
        Self {
            value,
            errors: vec![ErrorReport::error(span, error)],
        }
    }

    pub fn with_errors(value: T, errors: Vec<ErrorReport>) -> Self {
        Self { value, errors }
    }

    pub fn push(&mut self, error: ErrorReport) {
        self.errors.push(error);
    }
}

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
