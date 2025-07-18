use teleparse::{Span, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_error, cir_warning};
use crate::syn;

use super::MetaParser;

#[derive(Debug, Clone)]
pub enum GdtValue {
    Bool(bool),
    S32(i32),
    F32(f32),
    String32(String),
    String64(String),
    String256(String),
    Vec3f(f32, f32, f32),
    Vec2f(f32, f32),
}

impl Default for GdtValue {
    fn default() -> Self {
        GdtValue::Bool(false)
    }
}

impl std::hash::Hash for GdtValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            GdtValue::Bool(b) => b.hash(state),
            GdtValue::S32(i) => i.hash(state),
            GdtValue::F32(f) => f.to_bits().hash(state),
            GdtValue::String32(s) => s.hash(state),
            GdtValue::String64(s) => s.hash(state),
            GdtValue::String256(s) => s.hash(state),
            GdtValue::Vec3f(x, y, z) => {
                x.to_bits().hash(state);
                y.to_bits().hash(state);
                z.to_bits().hash(state);
            }
            GdtValue::Vec2f(x, y) => {
                x.to_bits().hash(state);
                y.to_bits().hash(state);
            }
        }
    }
}

impl PartialEq for GdtValue {
    fn eq(&self, other: &Self) -> bool {
        use GdtValue::*;
        match (self, other) {
            (Bool(a), Bool(b)) => a == b,
            (S32(a), S32(b)) => a == b,
            (F32(a), F32(b)) => a.to_bits() == b.to_bits(),
            (String32(a), String32(b)) => a == b,
            (String64(a), String64(b)) => a == b,
            (String256(a), String256(b)) => a == b,
            (Vec3f(ax, ay, az), Vec3f(bx, by, bz)) => {
                ax.to_bits() == bx.to_bits()
                    && ay.to_bits() == by.to_bits()
                    && az.to_bits() == bz.to_bits()
            }
            (Vec2f(ax, ay), Vec2f(bx, by)) => {
                ax.to_bits() == bx.to_bits() && ay.to_bits() == by.to_bits()
            }
            _ => false,
        }
    }
}
impl Eq for GdtValue {}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GdtMeta {
    pub value: GdtValue,
    pub array_idx: Option<usize>,
}

impl GdtMeta {
    pub fn new(value: GdtValue, idx: Option<usize>) -> Self {
        Self {
            value,
            array_idx: idx,
        }
    }
}

/// Parse the metadata for !set-gdt command
pub fn parse_gdt_meta(meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>) -> Option<GdtMeta> {
    let parser = GdtMetaParser::default();
    cir::parse_meta(meta, parser, errors)
}

/// Parse the metadata for !set-gdt-str command
pub fn parse_gdt_meta_str(
    meta: &syn::ItemMeta,
    errors: &mut Vec<ErrorReport>,
    quoted_value: &str,
) -> Option<GdtMeta> {
    let value = quoted_value.trim_matches('"');
    let parser = GdtMetaParser {
        string_value: Some(value),
        ..Default::default()
    };
    cir::parse_meta(meta, parser, errors)
}

#[derive(Debug, Clone, Default, PartialEq)]
struct GdtMetaParser<'s> {
    has_value: bool,
    value: GdtValue,
    array_idx: Option<usize>,
    vector_dim: u32,
    string_value: Option<&'s str>,
    x: f32,
    y: f32,
    z: f32,
}

impl MetaParser for GdtMetaParser<'_> {
    type Output = Option<GdtMeta>;

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
            "i" | "idx" | "index" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) if x >= 0 => {
                    self.array_idx = Some(x as usize);
                    self.has_value = true;
                    self.vector_dim = 0;
                }
                Ok(mv) => {
                    errors.push(cir_error!(value, InvalidMetaValue(key_str, mv)));
                }
                Err(e) => {
                    errors.push(e);
                }
            },
            "bool" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Bool(x)) => {
                    self.value = GdtValue::Bool(x);
                    self.has_value = true;
                    self.vector_dim = 0;
                }
                Ok(mv) => errors.push(cir_error!(value, InvalidMetaValue(key_str, mv))),
                Err(e) => errors.push(e),
            },
            "s32" | "i32" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Int(x)) => {
                    self.value = GdtValue::S32(x as i32);
                    self.has_value = true;
                    self.vector_dim = 0;
                }
                Ok(mv) => errors.push(cir_error!(value, InvalidMetaValue(key_str, mv))),
                Err(e) => errors.push(e),
            },
            "f32" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Float(x)) => {
                    self.value = GdtValue::F32(x as f32);
                    self.has_value = true;
                    self.vector_dim = 0;
                }
                Ok(mv) => errors.push(cir_error!(value, InvalidMetaValue(key_str, mv))),
                Err(e) => errors.push(e),
            },
            "string" | "str" | "string32" | "str32" => {
                match cir::parse_optional_meta_value(value.as_ref()) {
                    Ok(cir::MetaValue::Bool(x)) if x => {
                        let string_value = self.string_value.unwrap_or("");
                        if string_value.len() >= 32 {
                            errors.push(cir_error!(value, InvalidStringLength(32)));
                        } else {
                            self.value = GdtValue::String32(string_value.to_string());
                            self.has_value = true;
                            self.vector_dim = 0;
                        }
                    }
                    _ => errors.push(cir_error!(value, UnexpectedMetaKeyWithValue(key_str))),
                }
            }
            "string64" | "str64" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Bool(x)) if x => {
                    let string_value = self.string_value.unwrap_or("");
                    if string_value.len() >= 64 {
                        errors.push(cir_error!(value, InvalidStringLength(64)));
                    } else {
                        self.value = GdtValue::String64(string_value.to_string());
                        self.has_value = true;
                        self.vector_dim = 0;
                    }
                }
                _ => errors.push(cir_error!(value, UnexpectedMetaKeyWithValue(key_str))),
            },
            "string256" | "str256" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Bool(x)) if x => {
                    let string_value = self.string_value.unwrap_or("");
                    if string_value.len() >= 256 {
                        errors.push(cir_error!(value, InvalidStringLength(256)));
                    } else {
                        self.value = GdtValue::String64(string_value.to_string());
                        self.has_value = true;
                        self.vector_dim = 0;
                    }
                }
                _ => errors.push(cir_error!(value, UnexpectedMetaKeyWithValue(key_str))),
            },
            "vector2f" | "vec2f" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Bool(x)) if x => self.vector_dim = 2,
                _ => errors.push(cir_error!(value, UnexpectedMetaKeyWithValue(key_str))),
            },
            "vector3f" | "vec3f" => match cir::parse_optional_meta_value(value.as_ref()) {
                Ok(cir::MetaValue::Bool(x)) if x => self.vector_dim = 3,
                _ => errors.push(cir_error!(value, UnexpectedMetaKeyWithValue(key_str))),
            },
            "x" => {
                if self.vector_dim == 0 {
                    errors.push(cir_warning!(span, UnusedMetaKey(key_str)));
                } else {
                    match cir::parse_optional_meta_value(value.as_ref()) {
                        Ok(cir::MetaValue::Float(x)) => self.x = x as f32,
                        Ok(cir::MetaValue::Int(x)) => self.x = x as f32,
                        Ok(mv) => errors.push(cir_error!(value, InvalidMetaValue(key_str, mv))),
                        Err(e) => errors.push(e),
                    }
                }
            }
            "y" => {
                if self.vector_dim == 0 {
                    errors.push(cir_warning!(span, UnusedMetaKey(key_str)));
                } else {
                    match cir::parse_optional_meta_value(value.as_ref()) {
                        Ok(cir::MetaValue::Float(y)) => self.y = y as f32,
                        Ok(cir::MetaValue::Int(y)) => self.y = y as f32,
                        Ok(mv) => errors.push(cir_error!(value, InvalidMetaValue(key_str, mv))),
                        Err(e) => errors.push(e),
                    }
                }
            }
            "z" => {
                if self.vector_dim < 3 {
                    errors.push(cir_warning!(span, UnusedMetaKey(key_str)));
                } else {
                    match cir::parse_optional_meta_value(value.as_ref()) {
                        Ok(cir::MetaValue::Float(z)) => self.z = z as f32,
                        Ok(cir::MetaValue::Int(z)) => self.z = z as f32,
                        Ok(mv) => errors.push(cir_error!(value, InvalidMetaValue(key_str, mv))),
                        Err(e) => errors.push(e),
                    }
                }
            }
            _ => errors.push(cir_warning!(span, UnusedMetaKey(key_str))),
        }
    }

    fn visit_end(&mut self, meta: &syn::ItemMeta, errors: &mut Vec<ErrorReport>) {
        if !self.has_value {
            if self.string_value.is_some() {
                errors.push(cir_error!(meta, GdtStrTypeNotSet));
            } else {
                errors.push(cir_error!(meta, GdtTypeNotSet));
            }
        }
        match self.vector_dim {
            2 => self.value = GdtValue::Vec2f(self.x, self.y),
            3 => self.value = GdtValue::Vec3f(self.x, self.y, self.z),
            _ => {}
        }
    }

    fn finish(self) -> Self::Output {
        if self.has_value {
            Some(GdtMeta {
                value: self.value,
                array_idx: self.array_idx,
            })
        } else {
            None
        }
    }
}
