use teleparse::{Span, ToSpan, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_error, cir_warning};
use crate::search::{self, QuotedItemResolver, ResolvedItem};
use crate::syn;
use crate::util;

/// Specification for an item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemSelectSpec {
    /// Amount of the item
    ///
    /// What this value means depends on the context.
    pub amount: i64,

    /// The item or category to select from
    pub item: ItemOrCategory,

    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemOrCategory {
    Item(Item),
    Category(cir::Category),
}

/// An item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    /// The item actor name
    pub actor: String,

    /// Metadata of the item
    ///
    /// The "star" option should always be None, and the actor
    /// is adjusted to be the actor with the given star num
    pub meta: Option<cir::ItemMeta>,
}

impl Item {
    /// Check if the item is a CookResult (based on actor name)
    pub fn is_cook_item(&self) -> bool {
        self.actor.starts_with("Item_Cook_")
    }
}

pub async fn parse_item_list_finite_optional<R: QuotedItemResolver>(
    list: &tp::Option<syn::ItemListFinite>,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Vec<ItemSpec> {
    match list.as_ref() {
        Some(list) => parse_item_list_finite(list, resolver, errors).await,
        None => Vec::new(),
    }
}

pub async fn parse_item_list_finite<R: QuotedItemResolver>(
    list: &syn::ItemListFinite,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Vec<ItemSpec> {
    let mut out_item_specs = Vec::new();
    match list {
        syn::ItemListFinite::Single(item) => {
            if let Some(parsed_item) = parse_item(item, resolver, errors).await {
                if let Some(m) = &parsed_item.meta
                    && m.position.is_some()
                {
                    errors.push(cir_warning!(&item, UnusedItemPosition));
                }
                out_item_specs.push(ItemSpec {
                    amount: 1,
                    item: parsed_item,
                });
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

                if let Some(parsed_item) = parse_item(&item.item, resolver, errors).await {
                    if let Some(m) = &parsed_item.meta
                        && m.position.is_some()
                    {
                        errors.push(cir_warning!(&item, UnusedItemPosition));
                    }
                    out_item_specs.push(ItemSpec {
                        amount,
                        item: parsed_item,
                    });
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
            if let Some(result) = parse_item_or_category(item, resolver, errors).await {
                out_item_specs.push(ItemSelectSpec {
                    amount: 1,
                    item: result,
                    span: item.span(),
                });
            };
        }
        syn::ItemListConstrained::List(items) => {
            for item in items.iter() {
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
                    span: item.span(),
                });
            }
        }
    }

    out_item_specs
}

pub async fn parse_item_or_category<R: QuotedItemResolver>(
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

    // fix the armor star actor based on meta
    let star_num = meta.as_mut().and_then(|m| m.star.take()).unwrap_or(0);
    let actor = util::get_armor_with_star(&actor, star_num).to_string();

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
                errors.push(cir_error!(word, InvalidItem(word.to_string())));
            }
            result
        }
        syn::ItemName::Quoted(quoted_word) => {
            let name = quoted_word.as_str().trim_matches('"');
            if name.is_empty() {
                errors.push(cir_error!(quoted_word, InvalidEmptyItem));
                return None;
            }
            let result = resolver.resolve_quoted(name).await;
            if result.is_none() {
                errors.push(cir_error!(quoted_word, InvalidItem(name.to_string())));
            }
            result
        }
        syn::ItemName::Angle(angled_word) => {
            let name = &angled_word.name;
            if name.is_empty() {
                errors.push(cir_error!(angled_word, InvalidEmptyItem));
                None
            } else {
                Some(ResolvedItem::new(name.to_string()))
            }
        }
    }
}
