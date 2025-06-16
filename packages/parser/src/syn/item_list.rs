//! Syntax for specifying a list of items
use teleparse::{derive_syntax, tp};

use crate::syn;

/// Specifying an unconstrained, finite list of items
///
/// This is usually used for adding items
#[derive_syntax]
#[derive(Debug)]
pub enum ItemListFinite {
    /// Single item, e.g. `apple`
    Single(syn::Item),
    /// multiple items with amounts, e.g. `5 apples 3 royal_claymore`
    List(tp::Nev<syn::NumberedItem>),
}

/// Specifying items from a contrained list
///
/// This is usually used for selecting items from a list (for example,
/// for removal)
#[derive_syntax]
#[derive(Debug)]
pub enum ItemListConstrained {
    /// Single item, e.g. `apple :from slot 3`
    Single(syn::ItemOrCategory),
    /// multiple items with amounts, e.g. `5 apples 3 royal_claymore :from slot 3`
    List(tp::Nev<syn::NumberedOrAllItemOrCategory>),
}
