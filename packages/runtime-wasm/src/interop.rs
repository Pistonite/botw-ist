
mod __impl {
    use serde::Serialize;

    /// Interop type for @pistonite/pure/result
    #[derive(Debug, Clone, Serialize)]
    #[derive(tsify::Tsify)]
    #[tsify(into_wasm_abi)]
    pub enum ResultInterop<T, E> {
        #[serde(rename = "val")]
        Ok(T),
        #[serde(rename = "err")]
        Err(E),
    }
}

pub use __impl::ResultInterop as Result;
