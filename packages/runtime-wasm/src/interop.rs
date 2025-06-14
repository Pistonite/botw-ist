mod __impl {
    use serde::Serialize;

    /// Interop type for @pistonite/pure/result
    #[derive(Debug, Clone, Serialize, tsify::Tsify)]
    #[tsify(into_wasm_abi)]
    pub enum ResultInterop<T, E> {
        #[serde(rename = "val")]
        Ok(T),
        #[serde(rename = "err")]
        Err(E),
    }

    impl<T, E> From<Result<T, E>> for ResultInterop<T, E> {
        fn from(value: Result<T, E>) -> Self {
            match value {
                Ok(x) => Self::Ok(x),
                Err(x) => Self::Err(x),
            }
        }
    }
}

pub use __impl::ResultInterop as Result;
