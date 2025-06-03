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
        ErrorReport::error(&span, error)
    }
}

macro_rules! cir_error {
    ($span:expr, $error_type:ident ( $($args:expr),* $(,)? )) => {
        return Err($crate::error::ErrorReport::error(
            $span,
            $crate::error::Error::$error_type($($args),*),
        ))
    };
}
pub(crate) use cir_error;
macro_rules! cir_push_error {
    ($errors:ident, $span:expr, $error_type:ident) => {
        $errors.push($crate::error::ErrorReport::error(
            $span,
            $crate::error::Error::$error_type,
        ))
    };
    ($errors:ident, $span:expr, $error_type:ident ( $($args:expr),* $(,)? )) => {
        $errors.push($crate::error::ErrorReport::error(
            $span,
            $crate::error::Error::$error_type($($args),*),
        ))
    };
}
pub(crate) use cir_push_error;
macro_rules! cir_push_warning {
    ($errors:ident, $span:expr, $error_type:ident) => {
        $errors.push($crate::error::ErrorReport::warning(
            $span,
            $crate::error::Error::$error_type,
        ))
    };
    ($errors:ident, $span:expr, $error_type:ident ( $($args:expr),* $(,)? )) => {
        $errors.push($crate::error::ErrorReport::warning(
            $span,
            $crate::error::Error::$error_type($($args),*),
        ))
    };
}
pub(crate) use cir_push_warning;
