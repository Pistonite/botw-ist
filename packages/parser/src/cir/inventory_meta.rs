use teleparse::{Span, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_error};
use crate::syn;

use super::MetaParser;

/// Metadata for the `:slots` annotation
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlotMeta {
    pub weapon: Option<i32>,
    pub bow: Option<i32>,
    pub shield: Option<i32>,
}

/// Parse the meta for `:slots` annotation
pub fn parse_slots_meta(meta: &syn::Meta, errors: &mut Vec<ErrorReport>) -> SlotMeta {
    cir::parse_meta(meta, SlotMeta::default(), errors)
}

impl MetaParser for SlotMeta {
    type Output = Self;

    fn visit_entry(
        &mut self,
        key: &tp::String<syn::MetaKey>,
        value: Option<&syn::MetaValue>,
        v_span: Span,
        errors: &mut Vec<ErrorReport>,
    ) {
        super::cir_match_meta_key_value! { (key, key_str, value, v_span, errors):
            "weapon" | "weapons" => required {
                int(x) => {
                    let x = x as i32;
                    if x < 8 || x > 20 {
                        errors.push(cir_error!(v_span, InvalidEquipmentSlotNum(cir::Category::Weapon, x)));
                        return;
                    }
                    self.weapon = Some(x);
                }
            },
            "bow" | "bows" => required {
                int(x) => {
                    let x = x as i32;
                    if x < 5 || x > 14 {
                        errors.push(cir_error!(v_span, InvalidEquipmentSlotNum(cir::Category::Bow, x)));
                        return;
                    }
                    self.bow = Some(x);
                }
            },
            "shield" | "shields" => required {
                int(x) => {
                    let x = x as i32;
                    if x < 4 || x > 20 {
                        errors.push(cir_error!(v_span, InvalidEquipmentSlotNum(cir::Category::Shield, x)));
                        return;
                    }
                    self.shield = Some(x);
                }
            },
        }
    }

    fn visit_end(self, _meta: &syn::Meta, _errors: &mut Vec<ErrorReport>) -> Self::Output {
        self
    }
}

/// Metadata for the `:discovered` annotation
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscoverMeta {
    pub categories: [Option<bool>; 7],
}

/// Parse the meta for `:slots` annotation
pub fn parse_discover_meta(meta: &syn::Meta, errors: &mut Vec<ErrorReport>) -> DiscoverMeta {
    cir::parse_meta(meta, DiscoverMeta::default(), errors)
}

impl MetaParser for DiscoverMeta {
    type Output = Self;

    fn visit_entry(
        &mut self,
        key: &tp::String<syn::MetaKey>,
        value: Option<&syn::MetaValue>,
        v_span: Span, // key span if value doesn't exist
        errors: &mut Vec<ErrorReport>,
    ) {
        super::cir_match_meta_key_value! { (key, key_str, value, v_span, errors):
            "weapon" | "weapons" => optional {
                bool(x) => self.categories[0] = Some(x),
            },
            "bow" | "bows" | "arrow" | "arrows" => optional {
                bool(x) => self.categories[1] = Some(x),
            },
            "shield" | "shields" => optional {
                bool(x) => self.categories[2] = Some(x),
            },
            "armor" | "armors" => optional {
                bool(x) => self.categories[3] = Some(x),
            },
            "material" | "materials" => optional {
                bool(x) => self.categories[4] = Some(x),
            },
            "food" | "foods" => optional {
                bool(x) => self.categories[5] = Some(x),
            },
            "key-item" | "key-items" => optional {
                bool(x) => self.categories[6] = Some(x),
            },
        }
    }

    fn visit_end(self, _meta: &syn::Meta, _errors: &mut Vec<ErrorReport>) -> Self::Output {
        self
    }
}
