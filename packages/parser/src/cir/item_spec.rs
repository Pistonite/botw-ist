use crate::error::{Error, ErrorReport};
use crate::search::{QuotedItemResolver, ResolvedItem};
use crate::syn;

use super::ItemMeta;


/// Specification for adding an item
pub struct ItemAddSpec {
    /// Amount of the item to add.
    ///
    /// What this value means depends on the context.
    /// For example when adding item, this item will be added
    /// `amount` times.
    pub amount: i32,

    /// The item to add.
    pub actor: String,

    /// Metadata of the item to add
    pub meta: Option<ItemMeta>,
}

/// Specification for selecting an item (slot)
pub struct ItemSelectSpec {
    /// What this value means depends on the context.
    /// For example when removing item, this item will be removed
    /// `amount` times.
    pub amount: i32,

    /// The item to select
    pub actor: String,

    /// Metadata of the item to add
    pub meta: Option<ItemMeta>,
}

pub fn parse_item_add_list_finite(list: &syn::ItemListFinite) -> Vec<ItemAddSpec> {
}
async fn parse_item_in_add_list(amount: i32, item: &syn::Item, resolver: impl QuotedItemResolver, 
    errors: &mut Vec<ErrorReport>) -> ItemAddSpec {
}

async fn parse_item_name(item_name: &syn::ItemName, resolver: impl QuotedItemResolver, 
    errors: &mut Vec<ErrorReport>) -> ResolvedItem {
    match item_name {
        syn::ItemName::Word(word) => {
        },
        syn::ItemName::Quoted(quoted_word) => {
        },
        syn::ItemName::Angle(angled_word) => {
            let name = &angled_word.name;
            if name.is_empty() {
                errors.push(Error::
            }
            ResolvedItem::new(angled_word.name.as_str().to_string())
        }
    }
}
