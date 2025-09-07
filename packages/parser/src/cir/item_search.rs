use std::collections::BTreeSet;

use crate::cir;
use crate::data::{self, SearchResult};

/// Search for an item by V4 identifier such as `royal_claymore`, and return the best match
/// if there is one
pub fn search_item_by_ident(search_str: &str) -> Option<cir::ResolvedItem> {
    search_item_by_ident_all(search_str).into_iter().next()
}

/// Search for an item by V4 identifier such as `royal_claymore`. Returns all matches ordered by
/// score (best match first)
#[cfg_attr(
    feature = "cached",
    cached::proc_macro::cached(size = 512, key = "String", convert = "{ search_str.to_string() }")
)]
pub fn search_item_by_ident_all(search_str: &str) -> Vec<cir::ResolvedItem> {
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

    let Some((effect_id, effect_name, rest_search_str)) = split_and_search_effect(search_str)
    else {
        return do_search_item_by_ident_all(search_str, all_item);
    };

    let mut results = do_search_item_by_ident_all(rest_search_str, is_cook_item);
    if results.is_empty() {
        // fallback to search whole string if no cook items are found
        results = do_search_item_by_ident_all(search_str, all_item);

        if results.is_empty() {
            // V3 compatibility: some exlixirs are able to be found just by
            // searching the effect name, so we do a guess here.
            // if the search string is exactly a prefix of the effect,
            // then we coerce the result to a potion
            if rest_search_str.trim().is_empty() && effect_name.starts_with(search_str) {
                let item = cir::ResolvedItem {
                    actor: "Item_Cook_C_17".to_string(),
                    meta: None,
                };
                results = vec![item];
            }
        }
    }
    for result in &mut results {
        if is_cook_item(&result.actor) {
            set_effect(result, effect_id);
        }
    }
    results
}

/// Create an item for speed food. (for backward compability with V2 item)
fn speed_food() -> cir::ResolvedItem {
    cir::ResolvedItem {
        actor: "Item_Cook_A_03".to_string(),
        meta: Some(cir::ItemMeta {
            effect_id: Some(13),
            ..Default::default()
        }),
    }
}

/// Create an item for endura food. (for backward compability with V2 item)
fn endura_food() -> cir::ResolvedItem {
    cir::ResolvedItem {
        actor: "Item_Cook_A_01".to_string(),
        meta: Some(cir::ItemMeta {
            effect_id: Some(15),
            ..Default::default()
        }),
    }
}

/// If the first term in the search string could be an effect,
/// split it, returns the effect ID, effect name and the rest of the search string
///
/// Returns None if the first term matches multiple effects (i.e. ambiguous)
fn split_and_search_effect(search_str: &str) -> Option<(i32, &'static str, &str)> {
    let (maybe_effect, maybe_item) = match search_str.find(['_', '-']) {
        Some(i) => (&search_str[..i], &search_str[i + 1..]),
        None => (search_str, ""),
    };
    let mut found = None;
    for (effect_name, effect_id) in COOK_EFFECT_NAMES {
        if effect_name.contains(maybe_effect) {
            if found.is_some() {
                // found multiple
                return None;
            }
            found = Some((*effect_id, *effect_name));
        }
    }
    let (effect_id, effect_name) = found?;
    Some((
        effect_id,
        effect_name,
        maybe_item.trim_matches(|c| c == '_' || c == '-'),
    ))
}

fn set_effect(result: &mut cir::ResolvedItem, effect_id: i32) {
    result.meta = Some(cir::ItemMeta {
        effect_id: Some(effect_id),
        ..Default::default()
    });
}

fn do_search_item_by_ident_all(
    original_search_str: &str,
    filter: impl Fn(&str) -> bool,
) -> Vec<cir::ResolvedItem> {
    let mut all_results = BTreeSet::new();
    let all_search_strs = supplement_search_strings(original_search_str);
    for search_str in &all_search_strs {
        search_item_internal(original_search_str, search_str, &mut all_results);
    }

    all_results
        .into_iter()
        .filter(|x| filter(x.result.actor))
        .map(|r| cir::ResolvedItem {
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
        let new_search_str = format!("{rest}{replace}");
        all_search_strs.push(new_search_str);
    }
    all_search_strs
}

fn search_item_internal<'a>(
    original_search_str: &'a str,
    search_str: &str,
    out_results: &mut BTreeSet<SearchResult<'a, 'static>>,
) {
    // break name into _ or - separated search phrases
    let mut parts = search_str
        .split(['_', '-'])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());
    let Some(first_part) = parts.next() else {
        return;
    };
    let mut filtered = data::filter_items(first_part, original_search_str);

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

#[doc(hidden)]
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
