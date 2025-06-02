
mod __impl {
    use serde::Serialize;

    /// Interop type for @pistonite/pure/result
    #[derive(Debug, Clone, Serialize)]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    pub enum ResultInterop<T, E> {
        #[serde(rename = "val")]
        Ok(T),
        #[serde(rename = "err")]
        Err(E),
    }
}

pub use __impl::ResultInterop as Result;
