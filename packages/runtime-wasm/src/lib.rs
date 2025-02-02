use serde::{Deserialize, Serialize};
use teleparse::Root;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;
use skybook_parser::search;

#[wasm_bindgen]
pub fn parse_script(input: String) -> String {
    return format!("hello {}", input);
    // match CommandInit::parse(&input) {
    //     Ok(Some(cmd)) => format!("{:?}", cmd),
    //     Ok(None) => "no command found".to_string(),
    //     Err(e) => format!("error: {:?}", e),
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi)]
pub struct ItemSearchResult {
    actor: String,
    cook_effect: i32,
}

/// resolveItemIdent implementation
#[wasm_bindgen]
pub fn resolve_item_ident(query: String) -> Vec<ItemSearchResult> {
    search::search_item_by_ident_all(&query)
        .into_iter()
        .map(|r| ItemSearchResult {
            actor: r.actor,
            cook_effect: r.meta.and_then(|m| m.effect_id).unwrap_or(0),
        })
        .collect()
}

