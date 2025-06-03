use serde::Serialize;
use super::cir;

/// Error type for the parser
#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(tag = "type", content = "data")]
pub enum ParserError {
    //////////////////////////////////
    // DO NOT update the enum names
    // The translation files needs to be updated accordingly!!!
    //////////////////////////////////
    #[error("unexpected internal error: {0}")]
    Unexpected(String),
    #[error("unexpected syntax")]
    SyntaxUnexpected,
    #[error("unexpected syntax, expecting: {0}")]
    SyntaxUnexpectedExpecting(String),
    #[error("unexpected end of input")]
    SyntaxUnexpectedEof,
    #[error("failed to resolve item: {0}")]
    InvalidItem(String),
    #[error("item name cannot be empty")]
    InvalidEmptyItem,
    #[error("invalid integer format: {0}")]
    IntFormat(String),
    #[error("integer `{0}` is out of range")]
    IntRange(String),
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
    InvalidSlotClause(i32),
    #[error("`{0}` is not a valid number for times (must be at least 1)")]
    InvalidTimesClause(i32),
    #[error("`{0}` is not a valid trial name")]
    InvalidTrial(String),
    #[error("category `{0:?}` is not allowed in this context")]
    InvalidCategory(cir::Category),
    #[error("`{0}` is not a valid row in the inventory, valid values are [1, 2, 3, 4]")]
    InvalidInventoryRow(i32),
    #[error("`{0}` is not a valid column in the inventory, valid values are [1, 2, 3, 4, 5]")]
    InvalidInventoryCol(i32),
    #[error("The `{0}` key should not have a value when used in this context")]
    UnexpectedMetaKeyWithValue(String),
    #[error("The maximum length allowed for the string is {0} in this context")]
    InvalidStringLength(u32),
    #[error(
        "GDT meta must include one of the following properties: bool, s32, f32, vec2f or vec3f"
    )]
    GdtTypeNotSet,
    #[error(
        "GDT string meta must include one of the following properties: str32, str64, or str256"
    )]
    GdtStrTypeNotSet,
    #[error("`{1}` is not a valid number of slots for category `{0:?}`")]
    InvalidEquipmentSlotNum(cir::Category, i32),
    //////////////////////////////////
    // Add new errors below
    // The translation files needs to be updated accordingly!!!
    //////////////////////////////////
}
