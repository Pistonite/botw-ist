use std::collections::BTreeSet;

use crate::cir;
use crate::search::SearchResult;

use super::ResolvedItem;

/// Search for an item by V4 identifier such as `royal_claymore`. Returns all matches ordered by
/// score (best match first)
pub fn search_item_by_ident_all(search_str: &str) -> Vec<ResolvedItem> {
    // empty input case - this has to be here
    // because supplement_search_strings will fabricate non-empty search strings
    // even if the input is empty
    if search_str
        .trim_matches(|c| c == ' ' || c == '_' || c == '-')
        .is_empty()
    {
        return vec![];
    }
    let search_str = search_str.to_ascii_lowercase();
    let search_str = match search_str.as_str() {
        "speedfood" => return vec![speed_food()],
        "endurafood" => return vec![endura_food()],
        other => other,
    };

    let Some((effect_id, rest_search_str)) = split_and_search_effect(search_str) else {
        return do_search_item_by_ident_all(search_str, all_item);
    };

    let mut results = do_search_item_by_ident_all(rest_search_str, is_cook_item);
    // fallback to search whole string if no cook items are found
    if results.is_empty() {
        return do_search_item_by_ident_all(search_str, all_item);
    }

    for result in &mut results {
        set_effect(result, effect_id);
    }

    results
}

/// Search for an item by V4 identifier such as `royal_claymore`. Returns the best match
pub fn search_item_by_ident(search_str: &str) -> Option<ResolvedItem> {
    // empty input case - this has to be here
    // because supplement_search_strings will fabricate non-empty search strings
    // even if the input is empty
    if search_str
        .trim_matches(|c| c == ' ' || c == '_' || c == '-')
        .is_empty()
    {
        return None;
    }
    let search_str = search_str.to_ascii_lowercase();
    let search_str = match search_str.as_str() {
        "speedfood" => return Some(speed_food()),
        "endurafood" => return Some(endura_food()),
        other => other,
    };

    let Some((effect_id, rest_search_str)) = split_and_search_effect(search_str) else {
        return do_search_item_by_ident(search_str, all_item);
    };

    if let Some(mut result) = do_search_item_by_ident(rest_search_str, is_cook_item) {
        set_effect(&mut result, effect_id);
        return Some(result);
    }

    // fallback to search whole string if the item is not found
    // by just searching the part after the effect
    do_search_item_by_ident(search_str, all_item)
}

/// Create an item for speed food. (for backward compability with V2 item)
fn speed_food() -> ResolvedItem {
    ResolvedItem {
        actor: "Item_Cook_A_03".to_string(),
        meta: Some(cir::ItemMeta {
            effect_id: Some(13),
            ..Default::default()
        }),
    }
}

/// Create an item for endura food. (for backward compability with V2 item)
fn endura_food() -> ResolvedItem {
    ResolvedItem {
        actor: "Item_Cook_A_01".to_string(),
        meta: Some(cir::ItemMeta {
            effect_id: Some(15),
            ..Default::default()
        }),
    }
}

/// If the first term in the search string could be an effect,
/// split it, returns the effect ID and the rest of the search string
///
/// Returns None if the first term matches multiple effects (i.e. ambiguous)
fn split_and_search_effect(search_str: &str) -> Option<(i32, &str)> {
    let i = search_str.find(|c| c == '_' || c == '-')?;
    let (maybe_effect, maybe_item) = (&search_str[..i], &search_str[i + 1..]);
    let mut found = None;
    for (effect_name, effect_id) in COOK_EFFECT_NAMES {
        if effect_name.contains(maybe_effect) {
            if found.is_some() {
                // found multiple
                return None;
            }
            found = Some(*effect_id);
        }
    }
    found.map(|effect_id| (effect_id, maybe_item.trim_matches(|c| c == '_' || c == '-')))
}

fn set_effect(result: &mut ResolvedItem, effect_id: i32) {
    result.meta = Some(cir::ItemMeta {
        effect_id: Some(effect_id),
        ..Default::default()
    });
}

fn do_search_item_by_ident(
    original_search_str: &str,
    filter: impl Fn(&str) -> bool,
) -> Option<ResolvedItem> {
    let mut all_results = BTreeSet::new();
    let all_search_strs = supplement_search_strings(original_search_str);
    for search_str in &all_search_strs {
        search_item_internal(original_search_str, search_str, &mut all_results);
    }

    let first_result = all_results
        .into_iter()
        .filter(|x| filter(&x.result.actor))
        .next()?;

    Some(ResolvedItem {
        actor: first_result.result.actor.to_string(),
        meta: None,
    })
}

fn do_search_item_by_ident_all(
    original_search_str: &str,
    filter: impl Fn(&str) -> bool,
) -> Vec<ResolvedItem> {
    let mut all_results = BTreeSet::new();
    let all_search_strs = supplement_search_strings(original_search_str);
    for search_str in &all_search_strs {
        search_item_internal(original_search_str, search_str, &mut all_results);
    }

    all_results
        .into_iter()
        .filter(|x| filter(&x.result.actor))
        .map(|r| ResolvedItem {
            actor: r.result.actor.to_string(),
            meta: None,
        })
        .collect()
}

fn all_item(_: &str) -> bool {
    true
}

fn is_cook_item(actor: &str) -> bool {
    actor.starts_with("Item_Cook")
}

/// Supplement the search string with variations such as plural forms
fn supplement_search_strings(search_str: &str) -> Vec<String> {
    let mut all_search_strs = vec![search_str.to_string()];
    // convert plural forms for english words to singular form
    let tries = [("ies", "y"), ("es", ""), ("s", "")];
    for (find, replace) in tries {
        let Some(rest) = search_str.strip_suffix(find) else {
            continue;
        };
        let new_search_str = format!("{}{}", rest, replace);
        all_search_strs.push(new_search_str);
    }
    all_search_strs
}

pub fn search_item_internal<'a>(
    original_search_str: &'a str,
    search_str: &str,
    out_results: &mut BTreeSet<SearchResult<'a, 'static>>,
) {
    // break name into _ or - separated search phrases
    let mut parts = search_str
        .split(|c| c == '_' || c == '-')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());
    let Some(first_part) = parts.next() else {
        return;
    };
    let mut filtered = crate::generated::ITEM_NAMES
        .iter()
        .filter_map(|n| {
            if n.extended_item_name.contains(first_part) {
                Some(n.to_result(original_search_str))
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();

    for part in parts {
        filtered.retain(|n| n.result.extended_item_name.contains(part));
        match filtered.len() {
            0 => return,
            1 => {
                out_results.insert(filtered.into_iter().next().unwrap());
                return;
            }
            _ => {}
        }
    }

    out_results.extend(filtered);
}

pub static COOK_EFFECT_NAMES: &[(&str, i32)] = &[
    ("hearty", 2),
    ("energizing", 14),
    ("enduring", 15),
    ("hasty", 13),
    ("fireproof", 16),
    ("spciy", 5),
    ("chilly", 4),
    ("electro", 6),
    ("mighty", 10),
    ("tough", 11),
    ("sneaky", 12),
];
