use teleparse::{tp, Span, ToSpan};

use crate::error::Error;
use crate::syn;
use crate::{error::ErrorReport};

mod item_meta;
pub use item_meta::*;


pub trait MetaParser {
    type Output;

    async fn visit_start(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>);
    async fn visit_entry(&mut self, span: Span, key: &tp::String<syn::Word>, value: &tp::Option<syn::ItemMetaValue>, errors: &mut Vec<ErrorReport>);
    async fn visit_end(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>);
    async fn finish(self) -> Self::Output;
}

pub async fn parse_meta<T: MetaParser>(meta: &syn::ItemMeta, mut parser: T, errors: &mut Vec<ErrorReport>) -> T::Output {
    parser.visit_start(meta, errors).await;
    let span = meta.span();
    for entry in &meta.entries {
        parser.visit_entry(span, &entry.key, &entry.value, errors).await;
    }
    parser.visit_end(meta, errors).await;
    parser.finish().await
}

#[derive(Debug, Clone)]
pub enum MetaValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl std::fmt::Display for MetaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaValue::Bool(b) => write!(f, "{}", b),
            MetaValue::Int(i) => write!(f, "{}", i),
            MetaValue::Float(fl) => write!(f, "{}", fl),
            MetaValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl MetaValue {
    pub fn parse_option(value: Option<&syn::ItemMetaValue>) -> Result<Self, ErrorReport> {
        match value {
            Some(v) => Ok(Self::parse(&v.value)?),
            None => Ok(Self::Bool(true)),
        }
    }
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
                let int_part: &str = &*x.int_part;
                let int_part = match int_part.strip_prefix("0x") {
                    Some(rest) => i64::from_str_radix(rest, 16)
                        .map_err(|_| {
                            Error::IntFormat(x.int_part.to_string()).spanned(x)
                        })?,

                    None => int_part.parse()
                        .map_err(|_| {
                            Error::IntFormat(x.int_part.to_string()).spanned(x)
                        })?
                };
                let float_part = match &*x.float_part {
                    Some(fp) => fp,
                    None => return Ok(Self::Int(int_part)),
                };
                let decimal_part = match &*float_part.1 {
                    Some(dp) => dp,
                    // Integer followed by dot, like 3.
                    None => return Ok(Self::Float(int_part as f64)),
                };
                let decimal_str: &str = &*decimal_part;
                let decimal_num = match decimal_part.strip_prefix("0x") {
                    Some(_) =>  {
                        // float part can't be hex
                        return Err(
                            Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                        )
                    }

                    None => decimal_part.parse::<i64>()
                        .map_err(|_| {
                            Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                        })?
                };
                // float part can't be negative
                if decimal_num < 0 {
                    return Err(
                        Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                    )
                }
                let full_str = format!("{}.{}", int_part, decimal_str);
                let value = full_str.parse::<f64>()
                    .map_err(|_| {
                        Error::FloatFormat(format!("{}.{}", int_part, decimal_str)).spanned(x)
                    })?;
                return Ok(Self::Float(value));
            }
        }
        
    }
}

