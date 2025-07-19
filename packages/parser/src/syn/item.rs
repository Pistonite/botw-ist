//! Base syntax for specifying an item

use teleparse::{derive_syntax, tp};

use crate::syn;

/// Syntax for an item prefixed with a numeric amount
#[derive_syntax]
#[derive(Debug)]
pub struct NumberedItem {
    pub num: syn::Number,
    pub item: Item,
}
/// Syntax for an item prefixed with an amount or "all"
#[derive_syntax]
#[derive(Debug)]
pub enum NumberedOrAllItemOrCategory {
    Numbered(NumberedItemOrCategory),
    All(AllItemOrCategory),
}

/// Syntax for an item or category prefixed with a numeric amount
#[derive_syntax]
#[derive(Debug)]
pub struct NumberedItemOrCategory {
    pub num: syn::Number,
    pub item: ItemOrCategory,
}

/// Syntax for an item prefixed with "all" or "all but"
#[derive_syntax]
#[derive(Debug)]
pub struct AllItemOrCategory {
    pub all: syn::KwAll,
    pub but_clause: tp::Option<ButClause>,
    pub item: ItemOrCategory,
}

#[derive_syntax]
#[derive(Debug)]
pub struct ButClause {
    pub but: syn::KwBut,
    pub num: syn::Number,
}

/// Syntax for an item or a category
#[derive_syntax]
#[derive(Debug)]
pub enum ItemOrCategory {
    Item(Item),
    Category(syn::Category),
}

/// Syntax for an item or a category name only, without metadata
#[derive_syntax]
#[derive(Debug)]
pub enum ItemOrCategoryName {
    Item(ItemName),
    Category(syn::CategoryName),
}

/// Syntax for an item
///
/// # Example
/// - `item`
/// - `item[meta]`
/// - `"item"`
/// - `<item>`
#[derive_syntax]
#[derive(Debug)]
pub struct Item {
    pub name: ItemName,
    pub meta: tp::Option<syn::Meta>,
}

/// Syntax for the name of an item, like `item`, `"item"`, or `<item>`
///
/// This is also used as save names
#[derive_syntax]
#[derive(Debug)]
pub enum ItemName {
    /// Using `-` or `_` separated word to search item by English name
    Word(tp::String<syn::ItemWord>),
    /// Use quoted value to search by name in any language
    Quoted(tp::String<syn::QuotedWord>),
    /// Use angle brackets to use the literal as the actor name
    /// e.g. `<Weapon_Sword_070>`
    Angle(syn::AngledWord),
}

/// A word that can be used as item name or property name
#[derive_syntax]
#[derive(Debug)]
pub enum ItemWord {
    Word(syn::Word),
    // annotation words are kebab case and can be used as well
    // this is to avoid possible conflict that an annotation is very generic
    KwSmug(syn::KwSmug),
    KwPauseDuring(syn::KwPauseDuring),
    KwSameDialog(syn::KwSameDialog),
    KwAccuratelySimulate(syn::KwAccuratelySimulate),
    KwTargeting(syn::KwTargeting),
    KwOverworld(syn::KwOverworld),
    KwNonBreaking(syn::KwNonBreaking),
    KwBreaking(syn::KwBreaking),
    KwDpad(syn::KwDpad),
    KwPerUse(syn::KwPerUse),
    KwDiscovered(syn::KwDiscovered),
}
