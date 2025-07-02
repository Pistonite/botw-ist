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

/// Syntax for an item or category prefixed with a numeric amount
#[derive_syntax]
#[derive(Debug)]
pub struct NumberedItemOrCategory {
    pub num: syn::Number,
    pub item: ItemOrCategory,
}

/// Syntax for an item prefixed with an amount or "all"
#[derive_syntax]
#[derive(Debug)]
pub enum NumberedOrAllItemOrCategory {
    Numbered(NumberedItemOrCategory),
    All(AllItemOrCategory),
}

/// Syntax for an item prefixed with "all"
#[derive_syntax]
#[derive(Debug)]
pub struct AllItemOrCategory {
    #[teleparse(semantic(Amount))]
    pub all: syn::KwAll,
    pub item: ItemOrCategory,
}

/// Syntax for an item or a category
#[derive_syntax]
#[derive(Debug)]
pub enum ItemOrCategory {
    Item(Item),
    Category(syn::Category),
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
    pub meta: tp::Option<ItemMeta>,
}

/// Syntax for the name of an item, like `item`, `"item"`, or `<item>`
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

/// Syntax for the metadata specifier for an item, e.g. `[key1:value1, key2=value2, key3]`
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMeta {
    pub open: syn::SymLBracket,
    pub entries: tp::Punct<ItemMetaKeyValue, syn::SymComma>,
    pub close: syn::SymRBracket,
}

/// A key-value pair in an item's metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMetaKeyValue {
    /// The key of the key-value pair
    #[teleparse(semantic(Variable))]
    pub key: tp::String<ItemMetaKey>,
    pub value: tp::Option<ItemMetaValue>,
}

/// Valid strings for the key in an item's metadata specifier
///
/// This is needed because some keywords can be used as keys
#[derive_syntax]
#[derive(Debug)]
pub enum ItemMetaKey {
    Time(syn::KwTime),
    Slot(syn::KwSlot),
    Equip(syn::KwEquip),
    Other(syn::ItemWord),
}

/// A word that can be used as item name or property name
#[derive_syntax]
#[derive(Debug)]
pub enum ItemWord {
    Word(syn::Word),
    // annotation words are kebab case and can be used as well
    // this is to avoid possible conflict that an annotation is very generic
    KwWeaponSlots(syn::KwWeaponSlots),
    KwShieldSlots(syn::KwShieldSlots),
    KwBowSlots(syn::KwBowSlots),
}

/// Value after the key in an item's metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMetaValue {
    /// The seperator between the key and value
    pub sep: syn::ColonOrEqual,
    /// The value of the key-value pair
    pub value: syn::MetaValueLiteral,
}
