use std::ops::Deref;

use teleparse::{Span, ToSpan, tp};

use crate::cir;
use crate::error::{ErrorReport, absorb_error, cir_error, cir_warning};
use crate::search::{self, QuotedItemResolver, ResolvedItem};
use crate::syn;
use crate::util;

/// Specification for an item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemSpec {
    /// Amount of the item
    pub amount: usize,
    /// The item name
    pub name: String,
    /// The item metadata
    pub meta: Option<cir::ItemMeta>,
}

/// Specification for selecting an item
///
/// This is more detailed than [`ItemSpec`], allowing
/// selecting by category and selecting by slot number
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemSelectSpec {
    /// Amount of the item to target
    pub amount: AmountSpec,

    /// The item or category to target
    pub name: ItemNameSpec,

    /// The optional metadata to be matched, including position
    pub meta: Option<cir::ItemMeta>,

    pub span: Span,
}

/// Specifier for an amount when selecting items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmountSpec {
    All,
    AllBut(usize),
    Num(usize),
}

impl AmountSpec {
    pub fn is_zero(self) -> bool {
        matches!(self, Self::Num(0))
    }
    pub fn sub(&mut self, n: usize) {
        if let AmountSpec::Num(self_n) = self {
            *self_n = self_n.saturating_sub(n)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemNameSpec {
    Actor(String),
    Category(cir::Category),
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
    let list = match list {
        syn::ItemListFinite::Single(item) => {
            let Some((name, meta)) = parse_item(item, resolver, errors).await else {
                return vec![];
            };
            if meta.as_ref().is_some_and(|m| m.position.is_some()) {
                errors.push(cir_warning!(&item, UnusedItemPosition));
            }
            return vec![ItemSpec {
                amount: 1,
                name,
                meta,
            }];
        }
        syn::ItemListFinite::List(items) => items,
    };
    let mut out_item_specs = Vec::new();
    for item in list.iter() {
        let Some(amount) = parse_item_amount(&item.num, errors) else {
            continue;
        };

        let Some((name, meta)) = parse_item(&item.item, resolver, errors).await else {
            continue;
        };
        if meta.as_ref().is_some_and(|m| m.position.is_some()) {
            errors.push(cir_warning!(&item, UnusedItemPosition));
        }
        out_item_specs.push(ItemSpec { amount, name, meta });
    }

    out_item_specs
}

/// Parse a Contrained Item List
pub async fn parse_item_list_constrained<R: QuotedItemResolver>(
    list: &syn::ItemListConstrained,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Vec<ItemSelectSpec> {
    let list = match list {
        syn::ItemListConstrained::Single(item) => {
            if let Some((name, meta)) = parse_item_or_category(item, resolver, errors).await {
                return vec![ItemSelectSpec {
                    amount: AmountSpec::Num(1),
                    name,
                    meta,
                    span: item.span(),
                }];
            };
            return vec![];
        }
        syn::ItemListConstrained::List(items) => items,
    };

    let mut out_item_specs = Vec::new();

    for item in list.iter() {
        let (amount, item) = match item {
            syn::NumberedOrAllItemOrCategory::Numbered(item) => {
                let Some(amount) = parse_item_amount(&item.num, errors) else {
                    continue;
                };
                (AmountSpec::Num(amount), &item.item)
            }
            syn::NumberedOrAllItemOrCategory::All(item) => {
                let amount = match item.but_clause.deref() {
                    Some(but) => {
                        let Some(amount) = parse_item_amount(&but.num, errors) else {
                            continue;
                        };
                        AmountSpec::AllBut(amount)
                    }
                    None => AmountSpec::All,
                };
                (amount, &item.item)
            }
        };
        let item_span = item.span();
        let Some((name, meta)) = parse_item_or_category(item, resolver, errors).await else {
            continue;
        };
        out_item_specs.push(ItemSelectSpec {
            amount,
            name,
            meta,
            span: item_span,
        });
    }

    out_item_specs
}

pub async fn parse_item_or_category<R: QuotedItemResolver>(
    item: &syn::ItemOrCategory,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<(ItemNameSpec, Option<cir::ItemMeta>)> {
    match item {
        syn::ItemOrCategory::Item(item) => {
            let (item, meta) = parse_item(item, resolver, errors).await?;
            Some((ItemNameSpec::Actor(item), meta))
        }
        syn::ItemOrCategory::Category(category) => {
            let meta = category
                .meta
                .as_ref()
                .map(|m| cir::ItemMeta::parse_syn(m, errors));
            let category = cir::parse_category(&category.name);
            Some((ItemNameSpec::Category(category), meta))
        }
    }
}

/// Parse an item syntax node. Use the provided resolver to resolve quoted items.
async fn parse_item<R: QuotedItemResolver>(
    item: &syn::Item,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<(String, Option<cir::ItemMeta>)> {
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

    Some((actor, meta))
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

fn parse_item_amount(num: &syn::Number, errors: &mut Vec<ErrorReport>) -> Option<usize> {
    let amount = absorb_error(errors, cir::parse_syn_int_str(num, num.span()))?;
    if amount < 0 {
        errors.push(cir_error!(num, InvalidItemAmount));
        return None;
    }

    Some(amount as usize)
}
