/// Pointer interop type
///
/// This uses a u128 internal storage for a u64 value
/// to force generated bindings to convert the value to bigint
/// instead of number when sending to JS.
#[cfg(any(feature = "wasm", feature = "__ts-binding"))]
#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[repr(transparent)]
pub struct Pointer(u128);

#[cfg(not(any(feature = "wasm", feature = "__ts-binding")))]
#[derive(Debug, Default, Clone, serde::Serialize)]
#[repr(transparent)]
pub struct Pointer(u64);

#[cfg(any(feature = "wasm", feature = "__ts-binding"))]
impl From<u64> for Pointer {
    fn from(value: u64) -> Self {
        Self(value as u128)
    }
}

#[cfg(not(any(feature = "wasm", feature = "__ts-binding")))]
impl From<u64> for Pointer {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Pointer {
    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }
}

impl From<Pointer> for u64 {
    fn from(value: Pointer) -> Self {
        value.0 as u64
    }
}
