use serde::{Deserialize, Serialize};
use teleparse::{tp, Span, ToSpan};

use crate::error::{Error, ErrorReport};
use crate::syn;

mod category;
pub use category::*;

mod command;
pub use command::*;

mod entangle;
pub use entangle::*;

mod item_meta;
pub use item_meta::*;

mod item_spec;
pub use item_spec::*;

mod trial;
pub use trial::*;

/// Parser for the item meta syntax
///
/// This trait exists to allow the meta syntax to be reused for different purposes
pub trait MetaParser {
    type Output;

    fn visit_start(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>);
    fn visit_entry(
        &mut self,
        span: Span,
        key: &tp::String<syn::Word>,
        value: &tp::Option<syn::ItemMetaValue>,
        errors: &mut Vec<ErrorReport>,
    );
    fn visit_end(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>);
    fn finish(self) -> Self::Output;
}

pub fn parse_meta<T: MetaParser>(
    meta: &syn::ItemMeta,
    mut parser: T,
    errors: &mut Vec<ErrorReport>,
) -> T::Output {
    parser.visit_start(meta, errors);
    let span = meta.span();
    for entry in &meta.entries {
        parser.visit_entry(span, &entry.key, &entry.value, errors);
    }
    parser.visit_end(meta, errors);
    parser.finish()
}

/// Value in the metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(untagged)]
pub enum MetaValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl std::fmt::Display for MetaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{}", b),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "{}", s),
        }
    }
}

impl MetaValue {
    /// Parse an optional value. If the value is not present (i.e. only the key is specified),
    /// the value is assumed to be the boolean value `true`.
    pub fn parse_option(value: Option<&syn::ItemMetaValue>) -> Result<Self, ErrorReport> {
        match value {
            Some(v) => Ok(Self::parse(&v.value)?),
            None => Ok(Self::Bool(true)),
        }
    }

    /// Parse a value from a literal
    pub fn parse(value: &syn::MetaValueLiteral) -> Result<Self, ErrorReport> {
        match value {
            syn::MetaValueLiteral::Word(x) => {
                let s = x.trim();
                match s {
                    "true" => Ok(Self::Bool(true)),
                    "false" => Ok(Self::Bool(false)),
                    _ => Ok(Self::String(s.to_string())),
                }
            }
            syn::MetaValueLiteral::Number(x) => {
                let int_part = parse_syn_int_str(&x.int_part, &x.span())?;
                let float_part = match &*x.float_part {
                    Some(fp) => fp,
                    None => return Ok(Self::Int(int_part)),
                };
                let decimal_part = match &*float_part.1 {
                    Some(dp) => dp,
                    // Integer followed by dot, like 3.
                    None => return Ok(Self::Float(int_part as f64)),
                };
                let decimal_str: &str = decimal_part;
                let decimal_num = match decimal_part.strip_prefix("0x") {
                    Some(_) => {
                        // float part can't be hex
                        return Err(
                            Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                        );
                    }

                    None => decimal_part.parse::<i64>().map_err(|_| {
                        Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                    })?,
                };
                // float part can't be negative
                if decimal_num < 0 {
                    return Err(
                        Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                    );
                }
                let full_str = format!("{}.{}", int_part, decimal_str);
                let value = full_str.parse::<f64>().map_err(|_| {
                    Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                })?;

                Ok(Self::Float(value))
            }
        }
    }
}

pub fn parse_syn_int_str(number: &str, span: &Span) -> Result<i64, ErrorReport> {
    match number.strip_prefix("0x") {
        Some(rest) => i64::from_str_radix(rest, 16)
            .map_err(|_| Error::IntFormat(number.to_string()).spanned(span)),
        None => number
            .parse()
            .map_err(|_| Error::IntFormat(number.to_string()).spanned(span)),
    }
}
