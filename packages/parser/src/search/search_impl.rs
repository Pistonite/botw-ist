use std::collections::BTreeSet;

use crate::search::SearchResult;
use crate::cir;

use super::ResolvedItem;

pub fn search_item_by_ident(search_str: &str) -> Option<ResolvedItem> {
    // v2 food with effect
    match search_str.to_ascii_lowercase().as_str() {
        "speedfood" => {
            Some(ResolvedItem {
                actor: "Item_Cook_A_03".to_string(),
                meta: Some(cir::ItemMeta {
                    effect_id: Some(13),
                    ..Default::default()
                }),
            })
        },
        "endurafood" => {
            Some(ResolvedItem {
                actor: "Item_Cook_A_01".to_string(),
                meta: Some(cir::ItemMeta {
                    effect_id: Some(15),
                    ..Default::default()
                }),
            })
        },
        other => {
            let (maybe_effect, maybe_item) = match other.find(|c| c == '_' || c == '-') {
                Some(i) => (&other[..i], &other[i+1..]),
                None => return do_search_item_by_ident(other),
            };
            // check if first part can be effect
            let Some(effect_id) = search_effect(maybe_effect) else {
                return do_search_item_by_ident(other);
            };
            let maybe_item = maybe_item.trim_matches(|c| c == '_' || c == '-');
            if let Some(result) = do_search_item_by_ident(maybe_item) {
                // check if the result is a cook item
                if result.actor.starts_with("Item_Cook") {
                    return Some(ResolvedItem {
                        actor: result.actor,
                        meta: Some(cir::ItemMeta {
                            effect_id: Some(effect_id),
                            ..Default::default()
                        }),
                    });
                }
            }

            do_search_item_by_ident(other)
        }

    }
}

fn search_effect(maybe_effect: &str) -> Option<i32> {
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
    found
}

pub fn do_search_item_by_ident(search_str: &str) -> Option<ResolvedItem> {
    let mut all_results = BTreeSet::new();
    // convert plural forms for english words 
    // to singular form
    let tries = [
        ("ies", "y"),
        ("es", ""),
        ("s", ""),
    ];

    let mut all_search_strs = vec![search_str.to_string()];
    for (find, replace) in tries {
        let Some(rest) = search_str.strip_suffix(replace) else {
            continue;
        };
        let new_search_str = format!("{}{}", rest, find);
        all_search_strs.push(new_search_str);
    }
    for search_str in &all_search_strs {
        search_item_internal(search_str, &mut all_results);
        // println!("all_results: {:?}", all_results);
    }

    let first_result = all_results.into_iter().next()?;

    Some(ResolvedItem {
        actor: first_result.result.actor.to_string(),
        meta: None,
    })
}

pub fn search_item_internal<'a>(search_str: &'a str, out_results: &mut BTreeSet<SearchResult<'a, 'static>>) {
    // break name into _ or - separated search phrases
    let mut parts = search_str.split(|c| c == '_' || c == '-');
    let Some(first_part) = parts.next() else {
        return;
    };
    let mut filtered = crate::generated::ITEM_NAMES.iter()
        .filter_map(|n| {
            if n.search_str.contains(first_part) {
                Some(n.to_result(search_str))
            } else {
                None
            }
        }).collect::<BTreeSet<_>>();
    
    for part in parts {
        if part.is_empty() {
            continue;
        }
        filtered.retain(|n| {
            n.result.search_str.contains(part)
        });
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
