use std::collections::BTreeSet;

use super::SearchResult;

// generated from data/item-search-terms.yaml by the build script
include!("item_name.gen.rs");

/// Filter all items by the word
pub fn filter_items<'a>(
    filter_word: &str,
    original_search_str: &'a str,
) -> BTreeSet<SearchResult<'a, 'static>> {
    ITEM_NAMES
        .iter()
        .filter_map(|n| {
            if n.extended_item_name.contains(filter_word) {
                Some(n.to_result(original_search_str))
            } else {
                None
            }
        })
        .collect()
}

/// A searchable item entry
#[derive(Debug, PartialEq, Eq)]
pub struct SearchName {
    /// The string used to search for this item
    pub extended_item_name: &'static str,
    /// The actor name for the item
    pub actor: &'static str,
    /// If the item is a material
    pub is_material: bool,
    /// Length of the id from search_str
    pub id_len: u8,
}

impl SearchName {
    pub const fn new(
        search_str: &'static str,
        actor: &'static str,
        is_material: bool,
        id_len: u8,
    ) -> Self {
        Self {
            extended_item_name: search_str,
            actor,
            is_material,
            id_len,
        }
    }

    pub fn is_arrow(&self) -> bool {
        self.extended_item_name.contains("arrow")
    }

    pub fn get_type_for_compare(&self) -> u8 {
        if self.is_arrow() {
            1
        } else if self.is_material {
            2
        } else {
            3
        }
    }

    pub fn get_priority(&self) -> u8 {
        match self.extended_item_name {
            "treebranch"
            | "torch"
            | "soupladle"
            | "lizaltriboomerang"
            | "woodcuttersaxe"
            | "lightscaletrident"
            | "bokospear"
            | "normalarrow"
            | "potlid"
            | "hylianshroom"
            | "energeticrhinobeetle"
            | "korokseed" => 1,
            _ => 2,
        }
    }

    pub fn id(&self) -> &str {
        &self.extended_item_name[..self.id_len as usize]
    }

    pub fn to_result<'a, 'b>(&'b self, search_input: &'a str) -> SearchResult<'a, 'b> {
        SearchResult {
            search_input,
            result: self,
        }
    }
}
