use std::sync::Arc;

use js_sys::Function;
use serde::{Deserialize, Serialize};
use skybook_parser::{search, ParseOutput};
use skybook_runtime::{iv, RunOutput};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

mod js_item_resolve;
use js_item_resolve::JsQuotedItemResolver;

/// Initialize the WASM module
#[wasm_bindgen]
pub fn module_init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

//////////// Item Resolver //////////

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

////////// Parser //////////

/// Parse the script
///
/// ## Pointer Ownership
/// Returns ownership of the ParseOutput pointer.
#[wasm_bindgen]
pub async fn parse_script(script: String, resolve_quoted_item: Function) -> *const ParseOutput {
    let resolver = JsQuotedItemResolver::new(resolve_quoted_item);
    let parse_output = skybook_parser::parse(&resolver, &script).await;
    Arc::into_raw(Arc::new(parse_output))
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
///
/// ## Pointer Ownership
/// Borrows the ParseOutput pointer.
#[wasm_bindgen]
pub fn get_parser_errors(
    parse_output_ref: *const ParseOutput, // borrowed
) -> Vec<skybook_parser::ErrorReport> {
    if parse_output_ref.is_null() {
        return Vec::new();
    }
    let parse_output = unsafe { &*parse_output_ref };
    parse_output.errors.clone()
}

/// Free the parse output
#[wasm_bindgen]
pub fn free_parse_output(parse_output: *const ParseOutput, // takes ownership
) {
    if parse_output.is_null() {
        return;
    }
    let _ = unsafe { Arc::from_raw(parse_output) };
}

/// Add ref for the parse output
#[wasm_bindgen]
pub fn add_ref_parse_output(parse_output_ref: *const ParseOutput) -> *const ParseOutput {
    if parse_output_ref.is_null() {
        return std::ptr::null();
    }
    let x = unsafe { Arc::from_raw(parse_output_ref) };
    let x2 = Arc::clone(&x);
    let _ = Arc::into_raw(x);
    Arc::into_raw(x2)
}

/// Get index of the step from byte position in script
///
/// 0 is returned if steps are empty
///
/// ## Pointer Ownership
/// Borrows the ParseOutput pointer.
#[wasm_bindgen]
pub fn get_step_from_pos(parse_output_ref: *const ParseOutput, pos: usize) -> usize {
    if parse_output_ref.is_null() {
        return 0;
    }
    let parse_output = unsafe { &*parse_output_ref };
    parse_output.step_idx_from_pos(pos).unwrap_or_default()
}

////////// Runtime //////////

/// Run simulation using the ParseOutput
///
/// ## Pointer Ownership
/// Takes ownership of the ParseOutput pointer. Returns
/// ownership of the RunOutput pointer.
#[wasm_bindgen]
pub async fn run_parsed(parse_output: *const ParseOutput) -> *const RunOutput {
    let parse_output = unsafe { Arc::from_raw(parse_output) };
    let run_output = skybook_runtime::run_parsed(&parse_output).await;
    Arc::into_raw(Arc::new(run_output))
}

/// Free the run output
#[wasm_bindgen]
pub fn free_run_output(run_output: *const RunOutput, // takes ownership
) {
    if run_output.is_null() {
        return;
    }
    let _ = unsafe { Arc::from_raw(run_output) };
}

/// Add ref for the run output
#[wasm_bindgen]
pub fn add_ref_run_output(run_output_ref: *const RunOutput) -> *const RunOutput {
    if run_output_ref.is_null() {
        return std::ptr::null();
    }
    let x = unsafe { Arc::from_raw(run_output_ref) };
    let x2 = Arc::clone(&x);
    let _ = Arc::into_raw(x);
    Arc::into_raw(x2)
}

/// Get the Pouch inventory view for the given byte position in the script
///
/// ## Pointer Ownership
/// Borrows both the RunOutput and ParseOutput pointers.
#[wasm_bindgen]
pub fn get_pouch_list(
    run_output_ref: *const RunOutput,
    parse_output_ref: *const ParseOutput,
    byte_pos: usize,
) -> iv::PouchList {
    if parse_output_ref.is_null() || run_output_ref.is_null() {
        return Default::default();
    }
    let parse_output = unsafe { &*parse_output_ref };
    let step = parse_output.step_idx_from_pos(byte_pos).unwrap_or_default();
    let run_output = unsafe { &*run_output_ref };
    run_output.get_pouch_list(step)
}

/// Get the GDT inventory view for the given byte position in the script
///
/// ## Pointer Ownership
/// Borrows both the RunOutput and ParseOutput pointers.
#[wasm_bindgen]
pub fn get_gdt_inventory(
    run_output_ref: *const RunOutput,
    parse_output_ref: *const ParseOutput,
    byte_pos: usize,
) -> iv::Gdt {
    if parse_output_ref.is_null() || run_output_ref.is_null() {
        return Default::default();
    }
    let parse_output = unsafe { &*parse_output_ref };
    let step = parse_output.step_idx_from_pos(byte_pos).unwrap_or_default();
    let run_output = unsafe { &*run_output_ref };
    run_output.get_gdt_inventory(step)
}
