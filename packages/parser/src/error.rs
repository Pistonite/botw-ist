use teleparse::ToSpan;

use crate::syn;

pub type Error = skybook_api::parser::error::ParserError;
pub type ErrorReport = skybook_api::ErrorReport<Error>;
pub trait IntoErrorReport {
    fn into_error_report(self) -> ErrorReport;
}
impl IntoErrorReport for teleparse::syntax::Error<syn::TT> {
    fn into_error_report(self) -> ErrorReport {
        let span = self.span();
        let error = match self.data {
            teleparse::syntax::ErrorKind::Custom(message) => Error::Unexpected(message),
            teleparse::syntax::ErrorKind::UnexpectedCharacters => Error::SyntaxUnexpected,
            teleparse::syntax::ErrorKind::UnexpectedTokens => Error::SyntaxUnexpected,
            teleparse::syntax::ErrorKind::Expecting(_) => {
                // not really useful to display the expected set, since it can
                // be a large set of tokens sometimes
                Error::SyntaxUnexpected
            }
            teleparse::syntax::ErrorKind::UnexpectedEof => Error::SyntaxUnexpectedEof,
            teleparse::syntax::ErrorKind::UnexpectedNoAdvanceInLoop => {
                Error::Unexpected("no advance in parser loop".to_string())
            }
        };
        ErrorReport::error(span, error)
    }
}

/// Absort the error in the result into `errors`, turning it into an `Option`
pub fn absorb_error<T>(errors: &mut Vec<ErrorReport>, result: Result<T, ErrorReport>) -> Option<T> {
    match result {
        Ok(x) => Some(x),
        Err(e) => {
            errors.push(e);
            None
        }
    }
}

macro_rules! cir_fail {
    ($span:expr, $error_type:ident ( $($args:expr),* $(,)? )) => {
        return Err($crate::error::ErrorReport::error(
            $span,
            $crate::error::Error::$error_type($($args),*),
        ))
    };
}
pub(crate) use cir_fail;
macro_rules! cir_error {
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
pub(crate) use cir_error;
macro_rules! cir_warning {
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
pub(crate) use cir_warning;
