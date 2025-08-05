//! Syntax for specifying a list of items
use teleparse::{derive_syntax, tp};

use crate::syn;

/// Specifying an unconstrained, finite list of items
///
/// This is usually used for adding items
#[derive_syntax]
#[derive(Debug)]
pub struct ItemListFinite(pub tp::Nev<ItemListFiniteEntry>);
pub type ItemListFiniteEntry = (syn::MaybeNumberedItem, tp::Option<syn::SymComma>);

/// Specifying items from a contrained list
///
/// This is usually used for selecting items from a list (for example,
/// for removal)
#[derive_syntax]
#[derive(Debug)]
pub struct ItemListConstrained(pub tp::Nev<ItemListConstrainedEntry>);
pub type ItemListConstrainedEntry = (
    syn::MaybeNumberedOrAllItemOrCategory,
    tp::Option<syn::SymComma>,
);
