use teleparse::ToSpan;

use crate::cir;


pub struct ErrorReport {
    pub span: (usize, usize),
    pub is_warning: bool,
    pub error: Error,
}

impl ErrorReport {
    pub fn spanned<T: ToSpan>(t: &T, error: Error) -> Self {
        let span = t.span();
        Self {
            span: (span.lo, span.hi),
            is_warning: false,
            error,
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("failed to resolve item: {0}")]
    InvalidItem(String),
    #[error("invalid integer format: {0}")]
    IntFormat(String),
    #[error("invalid number format: {0}")]
    FloatFormat(String),
    #[error("unused meta key: {0}")]
    UnusedMetaKey(String),
    #[error("key `{0}` has invalid value: {0}")]
    InvalidMetaValue(String, cir::MetaValue),
    #[error("invalid weapon modifier: {0}")]
    InvalidWeaponModifier(String),
    #[error("invalid cook effect: {0}")]
    InvalidCookEffect(String),
    #[error("item has too many ingredients (max 5)")]
    TooManyIngredients,
}

impl Error {
    pub fn spanned<T: ToSpan>(self, t: &T) -> ErrorReport {
        ErrorReport::spanned(t, self)
    }

    pub fn spanned_warning<T: ToSpan>(self, t: &T) -> ErrorReport {
        let mut report = ErrorReport::spanned(t, self);
        report.is_warning = true;
        report
    }
}
