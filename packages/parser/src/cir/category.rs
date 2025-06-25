use enumset::EnumSet;
use teleparse::{Root, ToSpan};

use crate::cir;
use crate::error::{ErrorReport, cir_fail};
use crate::syn;

pub use skybook_api::parser::cir::Category;

pub fn parse_category_in(
    category: &syn::Category,
    filter: impl Into<EnumSet<Category>>,
) -> Result<Category, ErrorReport> {
    let filter = filter.into();
    let c = parse_category(category);
    if !filter.contains(c) {
        cir_fail!(category, InvalidCategory(c));
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

pub fn parse_category_from_str(category: &str) -> Option<Category> {
    let category = syn::Category::parse(category).ok()??;
    Some(parse_category(&category))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    let Some(times) = times else {
        return Ok(1);
    };
    let t = cir::parse_syn_int_str_i32(&times.times, &times.span())?;
    if t < 1 {
        cir_fail!(times, InvalidTimesClause(t));
    }
    Ok(t as i64)
}
