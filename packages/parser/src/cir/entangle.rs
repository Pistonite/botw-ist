use teleparse::{tp, Span};

use crate::cir;
use crate::error::{Error, ErrorReport};
use crate::syn;

use super::MetaParser;

pub fn parse_entangle_meta(
    category: &syn::Category,
    meta: Option<&syn::ItemMeta>,
    errors: &mut Vec<ErrorReport>,
) -> cir::CategorySpec {
    let category = cir::parse_category(category);
    let spec = cir::CategorySpec {
        category,
        amount: 1,
        row: 0,
        col: 0,
    };

    match meta {
        Some(meta) => cir::parse_meta(meta, EntangleMeta { inner: spec }, errors),
        None => spec,
    }
}

struct EntangleMeta {
    inner: cir::CategorySpec,
}

impl MetaParser for EntangleMeta {
    type Output = cir::CategorySpec;

    fn visit_start(&mut self, _meta: &syn::ItemMeta, _errors: &mut Vec<ErrorReport>) {}

    fn visit_entry(
        &mut self,
        span: Span,
        key: &tp::String<syn::Word>,
        value: &tp::Option<syn::ItemMetaValue>,
        errors: &mut Vec<ErrorReport>,
    ) {
        let key_str = key.to_ascii_lowercase();
        match key_str.trim() {
            "page" | "tab" => match cir::MetaValue::parse_option(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.inner.amount = x;
                }
                Ok(mv) => {
                    errors.push(Error::InvalidMetaValue(key_str, mv).spanned(value));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "row" => match cir::MetaValue::parse_option(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    if x < 1 || x > 4 {
                        errors.push(Error::InvalidInventoryRow(x as i32).spanned(value));
                        return;
                    }
                    self.inner.row = x as i8;
                }
                Ok(mv) => {
                    errors.push(Error::InvalidMetaValue(key_str, mv).spanned(value));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "col" => match cir::MetaValue::parse_option(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    if x < 1 || x > 5 {
                        errors.push(Error::InvalidInventoryCol(x as i32).spanned(value));
                        return;
                    }
                    self.inner.row = x as i8;
                }
                Ok(mv) => {
                    errors.push(Error::InvalidMetaValue(key_str, mv).spanned(value));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            _ => {
                errors.push(Error::UnusedMetaKey(key_str).spanned_warning(&span));
            }
        }
    }

    fn visit_end(&mut self, _meta: &syn::ItemMeta, _errors: &mut Vec<ErrorReport>) {}

    fn finish(self) -> Self::Output {
        self.inner
    }
}
