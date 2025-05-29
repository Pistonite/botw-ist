use enumset::{EnumSet, EnumSetType, enum_set};
use serde::Serialize;
use teleparse::ToSpan;

use crate::cir;
use crate::error::{Error, ErrorReport};
use crate::syn;

#[derive(Debug, EnumSetType, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
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

pub fn parse_category_in(
    category: &syn::Category,
    filter: impl Into<EnumSet<Category>>,
) -> Result<Category, ErrorReport> {
    let filter = filter.into();
    let c = parse_category(category);
    if !filter.contains(c) {
        return Err(Error::InvalidCategory(c).spanned(category));
    }
    Ok(c)
}

pub fn parse_category(category: &syn::Category) -> Category {
    match category {
        syn::Category::Weapon(_) => Category::Weapon,
        syn::Category::Bow(_) => Category::Bow,
        syn::Category::Shield(_) => Category::Shield,
        syn::Category::Armor(_) => Category::Armor,
        syn::Category::ArmorHead(_) => Category::ArmorHead,
        syn::Category::ArmorUpper(_) => Category::ArmorUpper,
        syn::Category::ArmorLower(_) => Category::ArmorLower,
        syn::Category::Material(_) => Category::Material,
        syn::Category::Food(_) => Category::Food,
        syn::Category::KeyItem(_) => Category::KeyItem,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategorySpec {
    /// Which category to select
    pub category: Category,
    /// The meaning depends on the context
    ///
    /// For example, this is the page to entangle
    pub amount: i64,
    /// Which row to select (1 indexed, 0 means unspecified)
    pub row: i8,
    /// Which column to select (1 indexed, 0 means unspecified)
    pub col: i8,
}
/// Parse any category and a times clause into a CategorySpec
pub fn parse_category_with_times(
    category: &syn::Category,
    times: Option<&syn::TimesClause>,
) -> Result<CategorySpec, ErrorReport> {
    let category = parse_category(category);
    let times = parse_times_clause(times)?;
    Ok(CategorySpec {
        category,
        amount: times,
        row: 0,
        col: 0,
    })
}

/// Parse a use category with a times clause.
///
/// Category must be Weapon, Bow, or Shield
pub fn parse_use_category_with_times(
    category: &syn::Category,
    times: Option<&syn::TimesClause>,
) -> Result<CategorySpec, ErrorReport> {
    let category = parse_category_in(
        category,
        Category::Weapon | Category::Bow | Category::Shield,
    )?;
    let times = parse_times_clause(times)?;
    Ok(CategorySpec {
        category,
        amount: times,
        row: 0,
        col: 0,
    })
}

pub fn parse_times_clause(times: Option<&syn::TimesClause>) -> Result<i64, ErrorReport> {
    match times {
        None => Ok(1),
        Some(times) => {
            let t = cir::parse_syn_int_str_i32(&times.times, &times.span())?;
            if t < 1 {
                return Err(Error::InvalidTimesClause(t).spanned(times));
            }
            Ok(t as i64)
        }
    }
}
