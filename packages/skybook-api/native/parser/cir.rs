use enumset::{EnumSet, EnumSetType, enum_set};
use serde::{Deserialize, Serialize};

/// Category in parser CIR
#[derive(Debug, EnumSetType, Serialize, Hash)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub enum Category {
    //////////////////////////////////
    // DO NOT update the enum names
    // The translation files needs to be updated accordingly!!!
    //////////////////////////////////
    Weapon,
    Bow,
    Shield,
    Armor,
    ArmorHead,
    ArmorUpper,
    ArmorLower,
    Material,
    Food,
    KeyItem,
}

impl Category {
    /// Check if this category is an armor category
    pub const fn is_armor(&self) -> bool {
        matches!(
            self,
            Category::Armor | Category::ArmorHead | Category::ArmorUpper | Category::ArmorLower
        )
    }

    /// Return the armor category if this category is armor (or a subcategory of armor),
    /// otherwise return the category itself
    pub const fn coerce_armor(&self) -> Self {
        match self {
            Category::ArmorHead => Category::Armor,
            Category::ArmorUpper => Category::Armor,
            Category::ArmorLower => Category::Armor,
            other => *other,
        }
    }

    /// Return categories except for ArmorHead, ArmorUpper, and ArmorLower
    pub const fn non_sub_categories() -> EnumSet<Self> {
        enum_set!(
            Category::Weapon
                | Category::Bow
                | Category::Shield
                | Category::Armor
                | Category::Material
                | Category::Food
                | Category::KeyItem
        )
    }
}

/// MetaValue in parser CIR
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
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
            Self::Bool(b) => write!(f, "{b}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::String(s) => write!(f, "{s}"),
        }
    }
}
