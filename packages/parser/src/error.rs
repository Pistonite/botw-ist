use serde::Serialize;
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

#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub enum Error {
    #[error("failed to resolve item: {0}")]
    InvalidItem(String),
    #[error("item name cannot be empty")]
    InvalidEmptyItem,
    #[error("invalid integer format: {0}")]
    IntFormat(String),
    #[error("invalid number format: {0}")]
    FloatFormat(String),
    #[error("unused meta key: {0}")]
    UnusedMetaKey(String),
    #[error("key `{0}` has invalid value: {1}")]
    InvalidMetaValue(String, cir::MetaValue),
    #[error("invalid weapon modifier: {0}")]
    InvalidWeaponModifier(String),
    #[error("invalid cook effect: {0}")]
    InvalidCookEffect(String),
    #[error("item has too many ingredients (max 5)")]
    TooManyIngredients,
    #[error("armor star number must be between 0 and 4, got: {0}")]
    InvalidArmorStarNum(i32),
    #[error("`{0}` is not a valid item slot specifier (must be at least 1)")]
    InvalidSlotClause(i64),
    #[error("`{0}` is not a valid number for times (must be at least 1)")]
    InvalidTimesClause(i64),
    #[error("`{0}` is not a valid trial name")]
    InvalidTrial(String),
    #[error("category `{0:?}` is not allowed in this context")]
    InvalidCategory(cir::Category),
    #[error("`{0}` is not a valid row in the inventory, valid values are [1, 2, 3, 4]")]
    InvalidInventoryRow(i64),
    #[error("`{0}` is not a valid column in the inventory, valid values are [1, 2, 3, 4, 5]")]
    InvalidInventoryCol(i64),
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
