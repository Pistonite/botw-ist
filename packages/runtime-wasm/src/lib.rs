#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{cell::OnceCell, sync::Arc};

use blueflame::env::GameVer;
use js_sys::{Function, Promise, Uint8Array};
use serde::{Deserialize, Serialize};
use skybook_parser::{ParseOutput, search};
use skybook_runtime::exec::Spawner;
use skybook_runtime::sim::{self, RuntimeInitParams};
use skybook_runtime::{MaybeAborted, RuntimeInitError, RuntimeViewError};
use skybook_runtime::{erc, iv};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

mod interop;

mod js_item_resolve;
use js_item_resolve::JsQuotedItemResolver;
use wasm_bindgen_futures::JsFuture;

thread_local! {
    static RUNTIME: OnceCell<Arc<sim::Runtime>> = const { OnceCell::new() };
}

#[wasm_bindgen]
extern "C" {
    /// Crash function on the global scope on the JS side
    pub fn __global_crash_handler();

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn log_error_in_js(js_error: JsValue);
}

/// Initialize the WASM module
#[wasm_bindgen]
pub async fn module_init(wasm_module_path: String, wasm_bindgen_js_path: String) {
    let _ = console_log::init_with_level(log::Level::Info);
    log::info!("initializing wasm module");
    std::panic::set_hook(Box::new(move |info| {
        console_error_panic_hook::hook(info);
        __global_crash_handler();
    }));

    let spawner = match Spawner::try_new(&wasm_module_path, &wasm_bindgen_js_path).await {
        Ok(spawner) => spawner,
        Err(e) => {
            panic!("failed to initialize spawner: {e}");
        }
    };

    RUNTIME.with(|runtime| {
        let _ = runtime.set(Arc::new(sim::Runtime::new(spawner)));
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
    params: Option<RuntimeInitParams>,
) -> interop::Result<RuntimeInitOutput, RuntimeInitError> {
    RUNTIME.with(|runtime| {
        let runtime = runtime
            .get()
            .expect("init_runtime called before module_init");
        let threads = 4;
        let result = match custom_image {
            Some(data) => {
                log::info!("initializing runtime in WASM using custom image");
                runtime.init(&data.to_vec(), threads, params.as_ref())
            }
            None => {
                log::info!("initializing runtime in WASM using default image");
                runtime.init(
                    include_bytes!("../../runtime-tests/data/program-mini.bfi"),
                    threads,
                    params.as_ref(),
                )
            }
        };
        let env = match result {
            Err(e) => {
                return interop::Result::Err(e);
            }
            Ok(x) => x,
        };
        let game_version = match env.game_ver {
            GameVer::X150 => "1.5",
            GameVer::X160 => "1.6",
        };
        interop::Result::Ok(RuntimeInitOutput {
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

/// Get the errors from the parse output.
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

/// Get the starting index for each step
///
/// ## Pointer Ownership
/// Borrows the ParseOutput pointer.
#[wasm_bindgen]
pub fn get_step_byte_positions(parse_output_ref: *const ParseOutput) -> Vec<u32> {
    if parse_output_ref.is_null() {
        return vec![];
    }
    let parse_output = unsafe { &*parse_output_ref };
    parse_output.steps.iter().map(|x| x.pos() as u32).collect()
}

////////// Runtime //////////

/// Make a run handle that you can pass back into run_parsed
/// to be able to abort the run
#[wasm_bindgen]
pub fn make_task_handle() -> *const sim::RunHandle {
    let handle = Arc::new(sim::RunHandle::new());
    sim::RunHandle::into_raw(handle)
}

/// Abort the task using the handle. Frees the handle
#[wasm_bindgen]
pub fn abort_task(ptr: *const sim::RunHandle) {
    let handle = sim::RunHandle::from_raw(ptr);
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
    handle: *const sim::RunHandle,
    notify_fn: Function, // (up_to_byte_pos, Arc<RunOutput> as usize) -> Promise<void>
) -> MaybeAborted<usize> {
    let parse_output = unsafe { Arc::from_raw(parse_output) };
    let handle = sim::RunHandle::from_raw(handle);
    let run = sim::Run::new(handle);
    let runtime = RUNTIME.with(|runtime| {
        // unwrap: the worker guarantees it calls init_runtime before this
        runtime
            .get()
            .expect("run_parsed called before module_init")
            .clone()
    });
    let output = run
        .run_parsed_with_notify(parse_output, &runtime, |up_to_byte_pos, output| {
            // this pointer is leaked to the JS side to be externally managed
            let output_ptr = Arc::into_raw(Arc::new(output.clone())) as usize;
            let result = notify_fn.call2(
                &JsValue::undefined(),
                &up_to_byte_pos.into(),
                &output_ptr.into(),
            );
            async {
                let promise = match result {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("error calling notify_fn in run_parsed");
                        log_error_in_js(e);
                        return;
                    }
                };
                // await the future if needed
                if let Ok(x) = promise.dyn_into::<Promise>()
                    && let Err(e) = JsFuture::from(x).await
                {
                    log::error!("error calling notify_fn in run_parsed");
                    log_error_in_js(e);
                }
            }
        })
        .await;

    match output {
        MaybeAborted::Ok(run_output) => {
            let run_output_ptr = Arc::into_raw(Arc::new(run_output));
            MaybeAborted::Ok(run_output_ptr as usize)
        }
        MaybeAborted::Aborted => MaybeAborted::Aborted,
    }
}

/// Get the errors from the run output.
///
/// ## Pointer Ownership
/// Borrows the RunOutput pointer.
#[wasm_bindgen]
pub fn get_run_errors(run_output_ref: *const sim::RunOutput) -> Vec<skybook_runtime::ErrorReport> {
    if run_output_ref.is_null() {
        return Vec::new();
    }
    let run_output = unsafe { &*run_output_ref };
    run_output.errors.clone()
}

macro_rules! deref_with_step {
    ($run:ident, $parse:ident, $pos:ident) => {{
        if $parse.is_null() || $run.is_null() {
            return Default::default();
        }
        let parse_output = unsafe { &*$parse };
        let step = parse_output.step_idx_from_pos($pos).unwrap_or_default();
        let run_output = unsafe { &*$run };
        // safety: the pass in pointers are leaked from Box,
        // so the reference will always be valid in the function
        (run_output, step)
    }};
}

/// Get the Pouch inventory view for the given byte position in the script
///
/// ## Pointer Ownership
/// Borrows both the RunOutput and ParseOutput pointers.
#[wasm_bindgen]
pub fn get_pouch_list(
    run_output_ref: *const sim::RunOutput,
    parse_output_ref: *const ParseOutput,
    byte_pos: usize,
) -> interop::Result<iv::PouchList, RuntimeViewError> {
    let (run_output, step) = deref_with_step!(run_output_ref, parse_output_ref, byte_pos);
    run_output.get_pouch_list(step).into()
}

/// Get the GDT inventory view for the given byte position in the script
///
/// ## Pointer Ownership
/// Borrows both the RunOutput and ParseOutput pointers.
#[wasm_bindgen]
pub fn get_gdt_inventory(
    run_output_ref: *const sim::RunOutput,
    parse_output_ref: *const ParseOutput,
    byte_pos: usize,
) -> interop::Result<iv::Gdt, RuntimeViewError> {
    let (run_output, step) = deref_with_step!(run_output_ref, parse_output_ref, byte_pos);
    run_output.get_gdt_inventory(step).into()
}

/// Get the overworld items for the given byte position in the script
///
/// ## Pointer Ownership
/// Borrows both the RunOutput and ParseOutput pointers.
#[wasm_bindgen]
pub fn get_overworld_items(
    run_output_ref: *const sim::RunOutput,
    parse_output_ref: *const ParseOutput,
    byte_pos: usize,
) -> interop::Result<iv::Overworld, RuntimeViewError> {
    let (run_output, step) = deref_with_step!(run_output_ref, parse_output_ref, byte_pos);
    run_output.get_overworld_items(step).into()
}

/// Get the crash info at the given byte position, empty if no crash
///
/// ## Pointer Ownership
/// Borrows both the RunOutput and ParseOutput pointers.
#[wasm_bindgen]
pub fn get_crash_info(
    run_output_ref: *const sim::RunOutput,
    parse_output_ref: *const ParseOutput,
    byte_pos: usize,
) -> String {
    let (run_output, step) = deref_with_step!(run_output_ref, parse_output_ref, byte_pos);
    run_output
        .get_crash_report(step)
        .map(|x| format!("{x:?}"))
        .unwrap_or_default()
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
pub fn free_task_handle(ptr: *const sim::RunHandle) {
    erc::free(ptr);
}
#[wasm_bindgen]
pub fn add_ref_task_handle(ptr: *const sim::RunHandle) -> *const sim::RunHandle {
    erc::add_ref(ptr)
}
#[wasm_bindgen]
pub fn free_run_output(ptr: *const sim::RunOutput) {
    erc::free(ptr);
}
#[wasm_bindgen]
pub fn add_ref_run_output(ptr: *const sim::RunOutput) -> *const sim::RunOutput {
    erc::add_ref(ptr)
}
