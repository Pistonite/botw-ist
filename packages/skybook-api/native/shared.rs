use serde::Serialize;
use teleparse::ToSpan;

/// Generic error report type with an inner error and span
/// of where the error occurred.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct ErrorReport<T> {
    pub span: (usize, usize),
    pub is_warning: bool,
    pub error: T,
}
impl<E> ErrorReport<E> {
    pub fn error<T: ToSpan>(t: T, error: E) -> Self {
        let span = t.span();
        Self {
            span: (span.lo, span.hi),
            is_warning: false,
            error,
        }
    }
    pub fn warning<T: ToSpan>(t: T, error: E) -> Self {
        let span = t.span();
        Self {
            span: (span.lo, span.hi),
            is_warning: true,
            error,
        }
    }
}
