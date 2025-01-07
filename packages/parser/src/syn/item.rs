//! Base syntax for specifying an item

use teleparse::{derive_syntax, tp};

use super::token::{
    AngledWord, ColonOrEqual, 
    NumOrAll, NumOrInfinite, Number, 
    QuotedWord, SymComma, SymLBracket, 
    SymRBracket, Word, MetaValueLiteral, KwAll, SlotClause};

use super::category::Category;

/// Syntax for an item prefixed with a numeric amount
#[derive_syntax]
#[derive(Debug)]
pub struct NumberedItem {
    #[teleparse(semantic(Amount))]
    pub num: Number,
    pub item: Item,
}

/// Syntax for an item or category prefixed with a numeric amount
#[derive_syntax]
#[derive(Debug)]
pub struct NumberedItemOrCategory {
    #[teleparse(semantic(Amount))]
    pub num: Number,
    pub items: ItemOrCategory,
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
    pub all: KwAll,
    pub items: ItemOrCategory,
}

/// Syntax for an item or a category
#[derive_syntax]
#[derive(Debug)]
pub enum ItemOrCategory {
    Item(Item),
    Category(Category),
}

/// Syntax for specifying a single item with a slot
#[derive_syntax]
#[derive(Debug)]
pub struct ItemOrCategoryWithSlot {
    pub item: ItemOrCategory,
    pub slot: tp::Option<SlotClause>,
}

// /// Syntax for an item with a slot or a category
// #[derive_syntax]
// #[derive(Debug)]
// pub enum ItemWithSlotOrCategory {
//     Item(ItemWithSlot),
//     Category(Category),
// }

/// Syntax for an item prefixed with an amount or "infinite"
#[derive_syntax]
#[derive(Debug)]
pub struct NumberedOrInfiniteItem {
    #[teleparse(semantic(Amount))]
    pub num: NumOrInfinite,
    pub item: Item,
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
    #[teleparse(semantic(Name))]
    pub name: ItemName,
    pub meta: tp::Option<ItemMeta>,
}

/// Syntax for the name of an item, like `item`, `"item"`, or `<item>`
#[derive_syntax]
#[derive(Debug)]
pub enum ItemName {
    /// Using `-` or `_` separated word to search item by English name
    Word(Word),
    /// Use quoted value to search by name in any language
    Quoted(QuotedWord),
    /// Use angle brackets to use the literal as the actor name
    /// e.g. `<Weapon_Sword_070>`
    Angle(AngledWord),
}

/// Syntax for the metadata specifier for an item, e.g. `[key1:value1, key2=value2, key3]`
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMeta {
    pub open: SymLBracket,
    pub entries: tp::Punct<ItemMetaKeyValue, SymComma>,
    pub close: SymRBracket,
}

/// A key-value pair in an item's metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMetaKeyValue {
    /// The key of the key-value pair
    #[teleparse(semantic(Variable))]
    pub key: tp::String<Word>,
    pub value: tp::Option<ItemMetaValue>
}

/// Value after the key in an item's metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMetaValue {
    /// The seperator between the key and value
    pub sep: ColonOrEqual,
    /// The value of the key-value pair
    pub value: MetaValueLiteral,
}
