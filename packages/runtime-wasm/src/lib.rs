use std::{cell::OnceCell, sync::Arc};

use blueflame::env::GameVer;
use js_sys::{Function, Uint8Array};
use serde::{Deserialize, Serialize};
use skybook_parser::{ParseOutput, search};
use skybook_runtime::{
    CustomImageInitParams, ResultInterop, RunHandle, RunOutput, Runtime, RuntimeInitError,
    RuntimeInitOutput, erc, error::MaybeAborted, exec::Spawner, iv,
};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

mod interop;

mod js_item_resolve;
use js_item_resolve::JsQuotedItemResolver;

thread_local! {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
}

#[wasm_bindgen]
extern "C" {
    /// Crash function on the global scope on the JS side
    pub fn __global_crash_handler();
}

/// Initialize the WASM module
#[wasm_bindgen]
pub async fn module_init(wasm_module_path: String, wasm_bindgen_js_path: String) {
    let _ = console_log::init_with_level(log::Level::Debug);
    log::info!("initializing wasm module");
    std::panic::set_hook(Box::new(move |info| {
        console_error_panic_hook::hook(info);
        __global_crash_handler();
    }));

    let spawner = match Spawner::try_new(&wasm_module_path, &wasm_bindgen_js_path).await {
        Ok(spawner) => spawner,
        Err(e) => {
            panic!("failed to initialize spawner: {}", e);
        }
    };

    RUNTIME.with(|runtime| {
        let _ = runtime.set(Runtime::new(spawner));
    });

    log::info!("wasm module initialized successfully");
}

#[derive(Debug, Clone, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInitOutput {
    /// "1.5" or "1.6"
    pub game_version: String,
}

/// Initialize the simulator runtime
#[wasm_bindgen]
pub fn init_runtime(
    // the params should be one Option, but wasm-bindgen doesn't support tuples
    custom_image: Option<Uint8Array>,
    custom_image_params: Option<CustomImageInitParams>,
) -> ResultInterop<RuntimeInitOutput, RuntimeInitError> {
    let custom_image = match (custom_image, custom_image_params) {
        (Some(data), Some(params)) => {
            let data = data.to_vec();
            Some((data, params))
        }
        _ => None,
    };

    RUNTIME.with(|runtime| {
        let runtime = runtime.get().unwrap();
        if let Err(e) = runtime.init(custom_image) {
            return ResultInterop::Err(e);
        }
        let game_version = match runtime.game_version() {
            GameVer::X150 => "1.5",
            GameVer::X160 => "1.6",
        };
        ResultInterop::Ok(RuntimeInitOutput {
            game_version: game_version.to_string(),
        })
    })
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
pub fn get_parser_errors(parse_output_ref: *const ParseOutput) -> Vec<skybook_parser::ErrorReport> {
    if parse_output_ref.is_null() {
        return Vec::new();
    }
    let parse_output = unsafe { &*parse_output_ref };
    parse_output.errors.clone()
}

/// Get the number of steps in the parse output (The actual number of steps/commands,
/// not number of steps displayed)
///
/// ## Pointer Ownership
/// Borrows the ParseOutput pointer.
#[wasm_bindgen]
pub fn get_step_count(parse_output_ref: *const ParseOutput) -> usize {
    if parse_output_ref.is_null() {
        return 0;
    }
    let parse_output = unsafe { &*parse_output_ref };
    parse_output.steps.len()
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

/// Make a run handle that you can pass back into run_parsed
/// to be able to abort the run
#[wasm_bindgen]
pub fn make_task_handle() -> *const RunHandle {
    let handle = Arc::new(RunHandle::new());
    RunHandle::into_raw(handle)
}

/// Abort the task using the handle. Frees the handle
#[wasm_bindgen]
pub fn abort_task(ptr: *const RunHandle) {
    let handle = RunHandle::from_raw(ptr);
    handle.abort();
}

/// Run simulation using the ParseOutput
///
/// ## Pointer Ownership
/// Takes ownership of the ParseOutput pointer. Returns
/// ownership of the RunOutput pointer.
#[wasm_bindgen]
pub async fn run_parsed(
    parse_output: *const ParseOutput,
    handle: *const RunHandle,
) -> MaybeAborted<usize> {
    let parse_output = unsafe { Arc::from_raw(parse_output) };
    let handle = RunHandle::from_raw(handle);
    match skybook_runtime::run_parsed(parse_output, handle).await {
        MaybeAborted::Ok(run_output) => {
            let run_output_ptr = Arc::into_raw(Arc::new(run_output));
            MaybeAborted::Ok(run_output_ptr as usize)
        }
        MaybeAborted::Aborted => MaybeAborted::Aborted,
    }
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

/// Get the overworld items for the given byte position in the script
///
/// ## Pointer Ownership
/// Borrows both the RunOutput and ParseOutput pointers.
#[wasm_bindgen]
pub fn get_overworld_items(
    run_output_ref: *const RunOutput,
    parse_output_ref: *const ParseOutput,
    byte_pos: usize,
) -> iv::Overworld {
    if parse_output_ref.is_null() || run_output_ref.is_null() {
        return Default::default();
    }
    let parse_output = unsafe { &*parse_output_ref };
    let step = parse_output.step_idx_from_pos(byte_pos).unwrap_or_default();
    let run_output = unsafe { &*run_output_ref };
    run_output.get_overworld_items(step)
}

////////// Ref Counting //////////
#[wasm_bindgen]
pub fn free_parse_output(ptr: *const ParseOutput) {
    erc::free(ptr);
}
#[wasm_bindgen]
pub fn add_ref_parse_output(ptr: *const ParseOutput) -> *const ParseOutput {
    erc::add_ref(ptr)
}
#[wasm_bindgen]
pub fn free_task_handle(ptr: *const RunHandle) {
    erc::free(ptr);
}
#[wasm_bindgen]
pub fn add_ref_task_handle(ptr: *const RunHandle) -> *const RunHandle {
    erc::add_ref(ptr)
}
#[wasm_bindgen]
pub fn free_run_output(ptr: *const RunOutput) {
    erc::free(ptr);
}
#[wasm_bindgen]
pub fn add_ref_run_output(ptr: *const RunOutput) -> *const RunOutput {
    erc::add_ref(ptr)
}
