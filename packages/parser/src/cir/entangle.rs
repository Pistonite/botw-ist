use teleparse::{Span, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_push_error, cir_push_warning};
use crate::syn;

use super::MetaParser;

pub fn parse_entangle_meta(
    category: &syn::Category,
    meta: Option<&syn::ItemMeta>,
    errors: &mut Vec<ErrorReport>,
) -> cir::CategorySpec {
    let parsed_category = cir::parse_category(category);
    if parsed_category.coerce_armor() != parsed_category {
        cir_push_warning!(errors, category, InvalidCategory(parsed_category));
    }

    let spec = cir::CategorySpec {
        category: parsed_category,
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
        key: &tp::String<syn::ItemMetaKey>,
        value: &tp::Option<syn::ItemMetaValue>,
        errors: &mut Vec<ErrorReport>,
    ) {
        let key_str = key.to_ascii_lowercase();
        match key_str.trim() {
            "page" | "tab" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.inner.amount = x;
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "row" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    if x < 1 || x > 4 {
                        cir_push_error!(errors, value, InvalidInventoryRow(x as i32));
                        return;
                    }
                    self.inner.row = x as i8;
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "col" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    if x < 1 || x > 5 {
                        cir_push_error!(errors, value, InvalidInventoryCol(x as i32));
                        return;
                    }
                    self.inner.row = x as i8;
                }
                Ok(mv) => {
                    cir_push_error!(errors, value, InvalidMetaValue(key_str, mv));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            _ => {
                cir_push_warning!(errors, &span, UnusedMetaKey(key_str));
            }
        }
    }

    fn visit_end(&mut self, _meta: &syn::ItemMeta, _errors: &mut Vec<ErrorReport>) {}

    fn finish(self) -> Self::Output {
        self.inner
    }
}
