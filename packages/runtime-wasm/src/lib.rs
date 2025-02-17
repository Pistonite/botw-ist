use std::ptr::NonNull;

use js_sys::Function;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;
use skybook_parser::{search, ParseOutput};

mod js_item_resolve;
use js_item_resolve::JsQuotedItemResolver;
//
//
// #[wasm_bindgen]
// pub fn init_runtime(
//     resolve_quoted_item: Function
// ) {
//     // create the runtime
//     let runtime = RuntimeWasm::new(JsQuotedItemResolver::new(resolve_quoted_item));
//     // set the runtime
//     let runtime_ref = unsafe { &mut *RUNTIME.get() };
//     runtime_ref.write(runtime);
// }
//

/// Parse the script
///
/// The returned pointer must be freed with `free_parse_output` when no longer needed.
#[wasm_bindgen]
pub async fn parse_script(
    script: String, 
    resolve_quoted_item: Function
) -> NonNull<ParseOutput> {
    let resolver = JsQuotedItemResolver::new(resolve_quoted_item);
    let parse_output = skybook_parser::parse(&resolver, &script).await;
    Box::leak(Box::new(parse_output)).into()
}

/// Parse the semantics of the script in the given range
///
/// The returned vector is triplets of (start, length, semantic token)
#[wasm_bindgen]
pub fn parse_script_semantic(script: String, start: usize, end: usize) -> Vec<u32> {
    let semantic_tokens = skybook_parser::parse_semantic(&script, start, end);
    let mut output = Vec::with_capacity(semantic_tokens.len() * 3);
    for (span, semantic) in semantic_tokens {
        output.push(span.lo as u32);
        output.push((span.hi - span.lo) as u32);
        output.push(semantic as u32);
    }
    output
}

/// Get the errors from the parse output. Does not take ownership of the parse output. (i.e.
/// does not free the parse output)
#[wasm_bindgen]
pub fn get_parser_errors(
    ptr: NonNull<ParseOutput>
) -> Vec<skybook_parser::ErrorReport> {
    let parse_output = unsafe { &*ptr.as_ptr() };
    parse_output.errors.clone()
}

/// Free the parse output
#[wasm_bindgen]
pub fn free_parse_output(
    ptr: NonNull<ParseOutput>
) {
    let _ = unsafe { Box::from_raw(ptr.as_ptr()) };
}

// #[wasm_bindgen]
// pub async fn on_script_change(script: String) {
//     let runtime = get_runtime_mut();
//     let script: Arc<str> = Arc::from(script);
//     runtime.execute_script(&script).await;
// }

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

