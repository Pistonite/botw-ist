use teleparse::{Span, ToSpan, tp};

use crate::error::{ErrorReport, cir_error, cir_fail};

use crate::cir;
use crate::syn;

/// Parser for the metadata syntax
///
/// This trait exists to allow the meta syntax to be reused for different purposes.
///
/// When parsing, `visit_start` will be called first with access to the entire
/// meta object. Then, each key/value pair will be visited with `visit_entry`.
/// Finally, `visit_end` consumes the parser and returns the output
pub trait MetaParser {
    type Output;

    fn visit_start(&mut self, _meta: &syn::Meta, _errors: &mut Vec<ErrorReport>) {
        // empty default
    }
    fn visit_entry(
        &mut self,
        key: &tp::String<syn::MetaKey>,
        value: Option<&syn::MetaValue>,
        value_span: Span, // key span if value doesn't exist
        errors: &mut Vec<ErrorReport>,
    );
    fn visit_end(self, meta: &syn::Meta, errors: &mut Vec<ErrorReport>) -> Self::Output;
}

/// Parse the meta syntax using the provided parser.
pub fn parse_meta<T: MetaParser>(
    meta: &syn::Meta,
    mut parser: T,
    errors: &mut Vec<ErrorReport>,
) -> T::Output {
    parser.visit_start(meta, errors);
    for entry in &meta.entries {
        let value = entry.value.as_ref().map(|x| &x.value);
        let value_span = value
            .as_ref()
            .map(|x| x.span())
            .unwrap_or_else(|| entry.key.span());
        parser.visit_entry(&entry.key, value, value_span, errors);
    }
    parser.visit_end(meta, errors)
}

/// Reduces boilerplate for parsing meta
///
/// Basically, the body of visit_entry should just be one invocation of this macro
#[rustfmt::skip]
macro_rules! cir_match_meta_key_value {
    (
        ( 
            $a_key:ident, $l_key_str:ident,
            $a_value:ident, $a_value_span:ident,
            $a_errors:ident
        ):
        $(
            $( $i_key_lit:literal)|* => $i_parse_fn:ident {
                $(
                    $variant:ident ( $i_local:ident ) => $i_block:stmt
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        let $l_key_str = $a_key.to_ascii_lowercase();
        match $l_key_str.as_str() {
        $(
            $( $i_key_lit )|* => match $crate::cir::meta::__parse_fn::$i_parse_fn(
                &$l_key_str, $a_value, $a_value_span
            ) {
            $(
                $crate::cir::meta::__match_type::$variant!($i_local) => 
                { $i_block },
            )*
                #[allow(unreachable_patterns)]
                Ok(mv) => $a_errors.push($crate::error::cir_error!(
                    $a_value_span, InvalidMetaValue($l_key_str, mv)
                )),
                Err(e) => $a_errors.push(e),
            }
        ),*
        _ => $a_errors.push($crate::error::cir_warning!($a_key, UnusedMetaKey($l_key_str)))
        }
    };
}
pub(crate) use cir_match_meta_key_value;

pub(crate) mod __match_type {
    macro_rules! bool {
        ($x:ident) => {
            Ok($crate::cir::MetaValue::Bool($x))
        };
    }
    pub(crate) use bool;
    macro_rules! int {
        ($x:ident) => {
            Ok($crate::cir::MetaValue::Int($x))
        };
    }
    pub(crate) use int;
    macro_rules! float {
        ($x:ident) => {
            Ok($crate::cir::MetaValue::Float($x))
        };
    }
    pub(crate) use float;
    macro_rules! words {
        ($x:ident) => {
            Ok($crate::cir::MetaValue::Words($x))
        };
    }
    pub(crate) use words;
    // macro_rules! quoted {
    //     ($x:ident) => {
    //         Ok($crate::cir::MetaValue::Quoted($x))
    //     }
    // }
    // pub(crate) use quoted;
    macro_rules! angled {
        ($x:ident) => {
            Ok($crate::cir::MetaValue::Angled($x))
        };
    }
    pub(crate) use angled;
    macro_rules! string {
        ($x:ident) => {
            Ok($crate::cir::MetaValue::Words($x))
                | Ok($crate::cir::MetaValue::Quoted($x))
                | Ok($crate::cir::MetaValue::Angled($x))
        };
    }
    pub(crate) use string;
}

pub(crate) mod __parse_fn {
    use super::*;
    pub fn optional(
        _key_str: &str,
        value: Option<&syn::MetaValue>,
        _value_span: Span,
    ) -> Result<cir::MetaValue, ErrorReport> {
        match value {
            Some(v) => Ok(parse_meta_value_internal(v)?),
            None => Ok(cir::MetaValue::Bool(true)),
        }
    }

    pub fn required(
        key_str: &str,
        value: Option<&syn::MetaValue>,
        value_span: Span,
    ) -> Result<cir::MetaValue, ErrorReport> {
        match value {
            Some(v) => Ok(parse_meta_value_internal(v)?),
            None => Err(cir_error!(
                value_span,
                RequiredMetaValue(key_str.to_string())
            )),
        }
    }
}

/// Parse a required meta value
pub fn parse_meta_value_internal(value: &syn::MetaValue) -> Result<cir::MetaValue, ErrorReport> {
    match value {
        syn::MetaValue::Word(x) => {
            let s = x.trim();
            match s {
                "true" => Ok(cir::MetaValue::Bool(true)),
                "false" => Ok(cir::MetaValue::Bool(false)),
                _ => Ok(cir::MetaValue::Words(s.to_string())),
            }
        }
        syn::MetaValue::Quoted(x) => Ok(cir::MetaValue::Quoted(x.trim_matches('"').to_string())),
        syn::MetaValue::Angled(x) => Ok(cir::MetaValue::Angled(x.name.to_string())),
        syn::MetaValue::Category(x) => Ok(cir::MetaValue::Words(x.to_string())),
        syn::MetaValue::Number(x) => {
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
