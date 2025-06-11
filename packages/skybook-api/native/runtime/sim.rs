use serde::{Deserialize, Serialize};

/// Parameters for initializing a custom image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInitParams {
    /// DLC version to simulate
    ///
    /// 0 means no DLC, 1-3 means DLC version 1.0 (Day 1),
    /// 2.0 (Master Trials), or 3.0 (Champion's Ballad)
    #[serde(default)]
    pub dlc: u32,

    /// Program start address
    ///
    /// The string should look like 0x000000XXXXX00000, where X is a hex digit
    ///
    /// Unspecified (empty string) means the script can run with any program start address
    #[serde(default)]
    pub program_start: String,

    /// Stack start address
    ///
    /// The string should look like 0x000000XXXXX00000, where X is a hex digit
    ///
    /// Unspecified (empty string) means using the internal default
    #[serde(default)]
    pub stack_start: String,

    /// Size of the stack
    ///
    /// Unspecified, or 0, means using the internal default
    #[serde(default)]
    pub stack_size: u32,

    /// Size of the free region of the heap
    ///
    /// Unspecified, or 0, means using the internal default
    #[serde(default)]
    pub heap_free_size: u32,

    /// Physical address of the PauseMenuDataMgr. Used to calculate heap start.
    /// Should be a hex string prefixed with 0x
    ///
    /// Unspecified (empty string) means using the internal default
    #[serde(default)]
    pub pmdm_addr: String,
}
