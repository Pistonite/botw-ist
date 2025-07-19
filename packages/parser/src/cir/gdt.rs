use teleparse::{Span, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_error, cir_warning};
use crate::syn;

use super::MetaParser;

/// Specifier for setting a GDT value
///
/// Vectors can also be set per-component, with `None`
/// meaning to use the current value for that component
#[derive(Debug, Clone)]
pub enum GdtValueSpec {
    Bool(bool),
    S32(i32),
    F32(f32),
    String32(String),
    String64(String),
    String256(String),
    Vec3f(Option<f32>, Option<f32>, Option<f32>),
    Vec2f(Option<f32>, Option<f32>),
}

impl Default for GdtValueSpec {
    fn default() -> Self {
        Self::Bool(false)
    }
}

impl std::hash::Hash for GdtValueSpec {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Bool(b) => b.hash(state),
            Self::S32(i) => i.hash(state),
            Self::F32(f) => f.to_bits().hash(state),
            Self::String32(s) => s.hash(state),
            Self::String64(s) => s.hash(state),
            Self::String256(s) => s.hash(state),
            Self::Vec3f(x, y, z) => {
                x.map(f32::to_bits).hash(state);
                y.map(f32::to_bits).hash(state);
                z.map(f32::to_bits).hash(state);
            }
            Self::Vec2f(x, y) => {
                x.map(f32::to_bits).hash(state);
                y.map(f32::to_bits).hash(state);
            }
        }
    }
}

impl PartialEq for GdtValueSpec {
    fn eq(&self, other: &Self) -> bool {
        use GdtValueSpec::*;
        match (self, other) {
            (Bool(a), Bool(b)) => a == b,
            (S32(a), S32(b)) => a == b,
            (F32(a), F32(b)) => a.to_bits() == b.to_bits(),
            (String32(a), String32(b)) => a == b,
            (String64(a), String64(b)) => a == b,
            (String256(a), String256(b)) => a == b,
            (Vec3f(ax, ay, az), Vec3f(bx, by, bz)) => {
                ax.map(f32::to_bits) == bx.map(f32::to_bits)
                    && ay.map(f32::to_bits) == by.map(f32::to_bits)
                    && az.map(f32::to_bits) == bz.map(f32::to_bits)
            }
            (Vec2f(ax, ay), Vec2f(bx, by)) => {
                ax.map(f32::to_bits) == bx.map(f32::to_bits)
                    && ay.map(f32::to_bits) == by.map(f32::to_bits)
            }
            _ => false,
        }
    }
}
impl Eq for GdtValueSpec {}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GdtMeta {
    pub value: GdtValueSpec,
    pub array_idx: Option<usize>,
}

impl GdtMeta {
    pub fn new(value: GdtValueSpec, idx: Option<usize>) -> Self {
        Self {
            value,
            array_idx: idx,
        }
    }
}

/// Parse the metadata for !set-gdt command
///
/// The format is as follows:
/// - if `i` is present, then assume the flag name is array flag, and i is the index
/// - the type keys are mutually exclusive
/// - if one of `s32, f32, bool, str32, str64, str256` type keys is specified,
///   the value is the GDT value to set
/// - if `vec2f` or `vec3f` is set (value must be empty), then additional properties
///   `x`, `y` and `z` are used to set the value
///
/// `string32, string64, string256, vector2f, vector3f` are aliases
pub fn parse_gdt_meta(meta: &syn::Meta, errors: &mut Vec<ErrorReport>) -> Option<GdtMeta> {
    let parser = GdtMetaParser::default();
    cir::parse_meta(meta, parser, errors)
}

// /// Parse the metadata for !set-gdt-str command
// pub fn parse_gdt_meta_str(
//     meta: &syn::ItemMeta,
//     errors: &mut Vec<ErrorReport>,
//     quoted_value: &str,
// ) -> Option<GdtMeta> {
//     let value = quoted_value.trim_matches('"');
//     let parser = GdtMetaParser {
//         string_value: Some(value),
//         ..Default::default()
//     };
//     cir::parse_meta(meta, parser, errors)
// }

#[derive(Debug, Clone, Default, PartialEq)]
struct GdtMetaParser {
    value: Option<GdtValueSpec>,
    array_idx: Option<usize>,
    // temporary state so x,y,z can be specified before the type
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

impl MetaParser for GdtMetaParser {
    type Output = Option<GdtMeta>;

    fn visit_entry(
        &mut self,
        key: &tp::String<syn::MetaKey>,
        value: Option<&syn::MetaValue>,
        v_span: Span,
        errors: &mut Vec<ErrorReport>,
    ) {
        super::cir_match_meta_key_value! { (key, key_str, value, v_span, errors):
            "i" | "idx" | "index" => required {
                int(x) => {
                    if x < 0 {
                        errors.push(cir_error!(v_span, GdtInvalidIndex(x as i32)));
                        return;
                    }
                    self.array_idx = Some(x as usize);
                },
            },
            "bool" => optional {
                bool(x) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    self.value = Some(GdtValueSpec::Bool(x))
                }
            },
            "s32" | "i32" => required {
                int(x) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    self.value = Some(GdtValueSpec::S32(x as i32))
                }
            },
            "f32" => required {
                float(x) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    self.value = Some(GdtValueSpec::F32(x as f32))
                }
            },
            "string32" | "str32" => required {
                string(x) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    if x.len() >= 32 {
                        errors.push(cir_error!(v_span, InvalidStringLength(32)));
                        return;
                    }
                    self.value = Some(GdtValueSpec::String32(x))
                }
            },
            "string64" | "str64" => required {
                string(x) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    if x.len() >= 64 {
                        errors.push(cir_error!(v_span, InvalidStringLength(64)));
                        return;
                    }
                    self.value = Some(GdtValueSpec::String64(x))
                }
            },
            "string256" | "str256" => required {
                string(x) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    if x.len() >= 256 {
                        errors.push(cir_error!(v_span, InvalidStringLength(256)));
                        return;
                    }
                    self.value = Some(GdtValueSpec::String256(x))
                }
            },
            "vector2f" | "vec2f" => optional {
                bool(true) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    self.value = Some(GdtValueSpec::Vec2f(None, None))
                },
            },
            "vector3f" | "vec3f" => optional {
                bool(true) => {
                    if self.value.is_some() {
                        errors.push(cir_error!(key, GdtTypeConflict));
                        return;
                    }
                    self.value = Some(GdtValueSpec::Vec3f(None, None, None))
                },
            },
            "x" => required {
                int(v) => self.x = Some(v as f32),
                float(v) => self.x = Some(v as f32),
            },
            "y" => required {
                int(v) => self.y = Some(v as f32),
                float(v) => self.y = Some(v as f32),
            },
            "z" => required {
                int(v) => self.z = Some(v as f32),
                float(v) => self.z = Some(v as f32),
            },
        }
    }

    fn visit_end(self, meta: &syn::Meta, errors: &mut Vec<ErrorReport>) -> Self::Output {
        let Some(mut value) = self.value else {
            errors.push(cir_error!(meta, GdtTypeNotSet));
            return None;
        };
        if matches!(value, GdtValueSpec::Vec2f(_, _)) {
            if self.x.is_none() && self.y.is_none() {
                errors.push(cir_warning!(meta, GdtMissingVecComp));
            } else {
                value = GdtValueSpec::Vec2f(self.x, self.y);
            }
        }
        if matches!(value, GdtValueSpec::Vec3f(_, _, _)) {
            if self.x.is_none() && self.y.is_none() && self.z.is_none() {
                errors.push(cir_warning!(meta, GdtMissingVecComp));
            } else {
                value = GdtValueSpec::Vec3f(self.x, self.y, self.z);
            }
        }
        Some(GdtMeta {
            value,
            array_idx: self.array_idx,
        })
    }
}
