use teleparse::{Span, ToSpan, tp};

use crate::error::{ErrorReport, cir_fail};

use crate::cir;
use crate::syn;

/// Parser for the item meta syntax
///
/// This trait exists to allow the meta syntax to be reused for different purposes
pub trait MetaParser {
    type Output;

    fn visit_start(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>);
    fn visit_entry(
        &mut self,
        span: Span,
        key: &tp::String<syn::ItemMetaKey>,
        value: &tp::Option<syn::ItemMetaValue>,
        errors: &mut Vec<ErrorReport>,
    );
    fn visit_end(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>);
    fn finish(self) -> Self::Output;
}

/// Parse the meta syntax using the provided parser.
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

/// Parse an optional value. If the value is not present (i.e. only the key is specified),
/// the value is assumed to be the boolean value `true`.
pub fn parse_optional_meta_value(
    value: Option<&syn::ItemMetaValue>,
) -> Result<cir::MetaValue, ErrorReport> {
    match value {
        Some(v) => Ok(parse_meta_value(&v.value)?),
        None => Ok(cir::MetaValue::Bool(true)),
    }
}

/// Parse a value from a literal
pub fn parse_meta_value(value: &syn::MetaValueLiteral) -> Result<cir::MetaValue, ErrorReport> {
    match value {
        syn::MetaValueLiteral::Word(x) => {
            let s = x.trim();
            match s {
                "true" => Ok(cir::MetaValue::Bool(true)),
                "false" => Ok(cir::MetaValue::Bool(false)),
                _ => Ok(cir::MetaValue::String(s.to_string())),
            }
        }
        syn::MetaValueLiteral::Category(x) => Ok(cir::MetaValue::String(x.trim().to_string())),
        syn::MetaValueLiteral::Number(x) => {
            let int_part = parse_syn_int_str(&x.int_part, x.span())?;
            let float_part = match &*x.float_part {
                Some(fp) => fp,
                None => return Ok(cir::MetaValue::Int(int_part)),
            };
            let decimal_part = match &*float_part.1 {
                Some(dp) => dp,
                // Integer followed by dot, like 3.
                None => return Ok(cir::MetaValue::Float(int_part as f64)),
            };
            let decimal_str: &str = decimal_part;
            let full_str = format!("{int_part}.{decimal_str}");
            let decimal_num = match decimal_part.strip_prefix("0x") {
                Some(_) => {
                    // float part can't be hex
                    cir_fail!(x, FloatFormat(full_str));
                }
                None => {
                    let Ok(num) = decimal_part.parse::<i64>() else {
                        // float part can't be non-numeric
                        cir_fail!(x, FloatFormat(full_str));
                    };
                    num
                }
            };
            // float part can't be negative
            if decimal_num < 0 {
                cir_fail!(x, FloatFormat(full_str));
            }
            let Ok(value) = full_str.parse::<f64>() else {
                cir_fail!(x, FloatFormat(full_str));
            };

            Ok(cir::MetaValue::Float(value))
        }
    }
}

pub fn parse_syn_int_str(number: &str, span: Span) -> Result<i64, ErrorReport> {
    match number.strip_prefix("0x") {
        Some(rest) => {
            let Ok(n) = i64::from_str_radix(rest, 16) else {
                cir_fail!(span, IntFormat(number.to_string()));
            };
            Ok(n)
        }
        None => {
            let Ok(n) = number.parse() else {
                cir_fail!(span, IntFormat(number.to_string()));
            };
            Ok(n)
        }
    }
}

pub fn parse_syn_int_str_i32(number: &str, span: Span) -> Result<i32, ErrorReport> {
    let number = parse_syn_int_str(number, span)?;
    if number > i32::MAX as i64 || number < i32::MIN as i64 {
        cir_fail!(span, IntRange(number.to_string()));
    }
    Ok(number as i32)
}
