//! Syntax for specifying a list of items

use teleparse::{derive_syntax, tp};

use super::item::{Item, ItemOrCategoryWithSlot, NumberedItem, NumberedOrInfiniteItem, NumberedOrAllItemOrCategory};
use super::token::SlotClause;

/// Specifying an unconstrained, finite list of items 
///
/// This is usually used for adding items
#[derive_syntax]
#[derive(Debug)]
pub enum ItemListFinite {
    /// Single item, e.g. `apple`
    Single(Item),
    /// multiple items with amounts, e.g. `5 apples 3 royal_claymore`
    List(tp::Nev<NumberedItem>),
}

/// Specifying an unconstrained list of items that allows 
/// an infinite amount of items
#[derive_syntax]
#[derive(Debug)]
pub enum ItemListInfinite {
    /// Single item, e.g. `apple`
    Single(Item),
    /// multiple items with amounts, e.g. `5 apples infinite royal_claymore`
    List(tp::Nev<NumberedOrInfiniteItem>),
}

/// Specifying items from a contrained list
///
/// This is usually used for selecting items from a list (for example,
/// for removal)
#[derive_syntax]
#[derive(Debug)]
pub enum ItemListConstrained {
    /// Single item, e.g. `apple :from slot 3`
    Single(ItemOrCategoryWithSlot),
    /// multiple items with amounts, e.g. `5 apples 3 royal_claymore :from slot 3`
    List(ItemListWithSlot),
}


/// Syntax for specifying a list of items with a slot
#[derive_syntax]
#[derive(Debug)]
pub struct ItemListWithSlot {
    pub items: tp::Nev<NumberedOrAllItemOrCategory>,
    pub slot: tp::Option<SlotClause>,
}
