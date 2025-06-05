use serde::Serialize;

/// Wrapper for output of a task which may be aborted by calling `abort` on the handle.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(tag = "type", content = "value")]
pub enum MaybeAborted<T> {
    Ok(T),
    Aborted,
}

/// Error type for calling `Runtime::init`
#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(tag = "type", content = "data")]
pub enum RuntimeInitError {
    #[error("executor error")]
    Executor,
    #[error("invalid DLC version: {0}. Valid versions are 0, 1, 2 or 3")]
    BadDlcVersion(u32),
    #[error("invalid custom image (1.6 is not supported right now)")]
    BadImage,
    #[error("program-start param is invalid")]
    InvalidProgramStart,
    #[error("stack-start param is invalid")]
    InvalidStackStart,
    #[error("pmdm-addr param is invalid")]
    InvalidPmdmAddr,
    #[error(
        "the custom image provided has program-start = {0}, which does not match the one requested by the environment = {0}"
    )]
    ProgramStartMismatch(String, String),
}

/// Error type for the runtime
#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(tag = "type", content = "data")]
pub enum RuntimeError {
    //////////////////////////////////
    // DO NOT update the enum names
    // The translation files needs to be updated accordingly!!!
    //////////////////////////////////
    #[error("the runtime has not been initialized yet, you need to call `Runtime::init`")]
    Uninitialized,
    #[error("game has crashed in this step")]
    Crash,
    #[error(
        "game has crashed in a previous step and you need to `reload` or `new-game` to continue"
    )]
    PreviousCrash,
    #[error("unexpected executor error")]
    Executor,
    #[error(
        "this command or syntax is not implemented yet, please track the development on GitHub"
    )]
    Unimplemented,
    //////////////////////////////////
    // Add new errors below
    // The translation files needs to be updated accordingly!!!
    //////////////////////////////////
}

/// Error type for viewing results from the runtime
#[derive(Debug, Clone, thiserror::Error, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(tag = "type", content = "data")]
pub enum RuntimeViewError {
    //////////////////////////////////
    // DO NOT update the enum names
    // The translation files needs to be updated accordingly!!!
    //////////////////////////////////
    #[error("game has crashed at or before this step")]
    Crash,
    #[error("failed to read state from memory")]
    Memory,
    #[error("coherence check failed when reading state")]
    Coherence,
    //////////////////////////////////
    // Add new errors below
    // The translation files needs to be updated accordingly!!!
    //////////////////////////////////
}
