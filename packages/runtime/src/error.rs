use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(tag = "type", content = "value")]
pub enum MaybeAborted<T> {
    Ok(T),
    Aborted,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("executor error: {0}")]
    Executor(#[from] crate::exec::Error),
}
