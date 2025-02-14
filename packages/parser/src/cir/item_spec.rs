use teleparse::ToSpan;

use crate::cir;
use crate::error::{Error, ErrorReport};
use crate::search::{self, QuotedItemResolver, ResolvedItem};
use crate::syn;
use crate::util;

/// Specification for an item
#[derive(Debug, Clone)]
pub struct ItemSpec {
    /// Amount of the item
    ///
    /// What this value means depends on the context.
    /// For example when adding item, this item will be added
    /// `amount` times.
    pub amount: i64,

    /// The item
    pub item: Item,
}

/// Specification for selecting an item
///
/// This is more detailed than [`ItemSpec`], allowing
/// selecting by category and selecting by slot number
///
/// The meaning of the spec depends on the context
#[derive(Debug, Clone)]
pub struct ItemSelectSpec {
    /// Amount of the item
    ///
    /// What this value means depends on the context.
    pub amount: i64,

    /// The item or category to select from
    pub item: ItemOrCategory,

    /// The slot number to select from, 0 means not specified
    pub slot: i64,
}

#[derive(Debug, Clone)]
pub enum ItemOrCategory {
    Item(Item),
    Category(cir::Category),
}

/// An item
#[derive(Debug, Clone)]
pub struct Item {
    /// The item actor name
    pub actor: String,

    /// Metadata of the item
    ///
    /// The "star" option should always be None, and the actor
    /// is adjusted to be the actor with the given star num
    pub meta: Option<cir::ItemMeta>,
}

pub async fn parse_item_list_finite<R: QuotedItemResolver>(
    list: &syn::ItemListFinite,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Vec<ItemSpec> {
    let mut out_item_specs = Vec::new();
    match list {
        syn::ItemListFinite::Single(item) => {
            if let Some(item) = parse_item(item, resolver, errors).await {
                out_item_specs.push(ItemSpec { amount: 1, item });
            }
        }
        syn::ItemListFinite::List(items) => {
            for item in items.iter() {
                let amount = match cir::parse_syn_int_str(&item.num, &item.num.span()) {
                    Ok(amount) => amount,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                if let Some(item) = parse_item(&item.item, resolver, errors).await {
                    out_item_specs.push(ItemSpec { amount, item });
                }
            }
        }
    }

    out_item_specs
}

pub async fn parse_item_list_constrained<R: QuotedItemResolver>(
    list: &syn::ItemListConstrained,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Vec<ItemSelectSpec> {
    let mut out_item_specs = Vec::new();

    match list {
        syn::ItemListConstrained::Single(item) => {
            if let Some(result) = parse_item_or_category_with_slot(item, resolver, errors).await {
                out_item_specs.push(result);
            };
        }
        syn::ItemListConstrained::List(items) => {
            let slot = match parse_slot_clause(items.slot.as_ref()) {
                Ok(slot) => slot,
                Err(e) => {
                    errors.push(e);
                    return out_item_specs;
                }
            };

            for item in items.items.iter() {
                let (amount, item) = match item {
                    syn::NumberedOrAllItemOrCategory::Numbered(item) => {
                        let amount = match cir::parse_syn_int_str(&item.num, &item.num.span()) {
                            Ok(amount) => amount,
                            Err(e) => {
                                errors.push(e);
                                continue;
                            }
                        };
                        (amount, &item.item)
                    }
                    syn::NumberedOrAllItemOrCategory::All(item) => (-1, &item.item),
                };
                let Some(result) = parse_item_or_category(item, resolver, errors).await else {
                    continue;
                };
                out_item_specs.push(ItemSelectSpec {
                    amount,
                    item: result,
                    slot,
                });
            }
        }
    }

    out_item_specs
}

pub async fn parse_item_or_category_with_slot<R: QuotedItemResolver>(
    item: &syn::ItemOrCategoryWithSlot,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<ItemSelectSpec> {
    let result = parse_item_or_category(&item.item, resolver, errors).await?;
    match parse_slot_clause(item.slot.as_ref()) {
        Ok(slot) => Some(ItemSelectSpec {
            amount: 1,
            item: result,
            slot,
        }),
        Err(e) => {
            errors.push(e);
            None
        }
    }
}

async fn parse_item_or_category<R: QuotedItemResolver>(
    item: &syn::ItemOrCategory,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<ItemOrCategory> {
    match item {
        syn::ItemOrCategory::Item(item) => {
            let item = parse_item(item, resolver, errors).await?;
            Some(ItemOrCategory::Item(item))
        }
        syn::ItemOrCategory::Category(category) => {
            let category = cir::parse_category(category);
            Some(ItemOrCategory::Category(category))
        }
    }
}

/// Parse an item syntax node. Use the provided resolver to resolve quoted items.
async fn parse_item<R: QuotedItemResolver>(
    item: &syn::Item,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<Item> {
    let resolved_item = parse_item_name(&item.name, resolver, errors).await?;
    let actor = resolved_item.actor;
    // merge the resolved meta and input meta
    let input_meta = item.meta.as_ref();
    let mut meta = match (resolved_item.meta, input_meta) {
        (None, None) => None,
        (None, Some(x)) => Some(cir::ItemMeta::parse_syn(x, errors)),
        (Some(x), None) => Some(x),
        (Some(mut resolved), Some(input)) => {
            // the input meta overrides from resolved
            resolved.parse(input, errors);
            Some(resolved)
        }
    };

    let actor = if let Some(star_num) = meta.as_mut().and_then(|m| m.star.take()) {
        util::get_armor_with_star(&actor, star_num).to_string()
    } else {
        actor
    };

    Some(Item { actor, meta })
}

/// Parse an item name syntax node. Use the provided resolver to resolve quoted items.
async fn parse_item_name<R: QuotedItemResolver>(
    item_name: &syn::ItemName,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<ResolvedItem> {
    match item_name {
        syn::ItemName::Word(word) => {
            let result = search::search_item_by_ident(word);
            if result.is_none() {
                errors.push(Error::InvalidItem(word.to_string()).spanned(word));
            }
            result
        }
        syn::ItemName::Quoted(quoted_word) => {
            let name = quoted_word.as_str().trim_matches('"');
            if name.is_empty() {
                errors.push(Error::InvalidEmptyItem.spanned(quoted_word));
                return None;
            }
            let result = resolver.resolve_quoted(name).await;
            if result.is_none() {
                errors.push(Error::InvalidItem(name.to_string()).spanned(quoted_word));
            }
            result
        }
        syn::ItemName::Angle(angled_word) => {
            let name = &angled_word.name;
            if name.is_empty() {
                errors.push(Error::InvalidEmptyItem.spanned(angled_word));
                None
            } else {
                Some(ResolvedItem::new(name.to_string()))
            }
        }
    }
}

/// Parse a SlotClause syntax node
fn parse_slot_clause(slot: Option<&syn::SlotClause>) -> Result<i64, ErrorReport> {
    match slot {
        None => Ok(0),
        Some(slot) => {
            let slot_num = cir::parse_syn_int_str(&slot.idx, &slot.idx.span())?;
            if slot_num < 1 {
                return Err(Error::InvalidSlotClause(slot_num as i32).spanned(slot));
            }
            Ok(slot_num)
        }
    }
}
