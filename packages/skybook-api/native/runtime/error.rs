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

impl<T> MaybeAborted<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(x) => x,
            Self::Aborted => panic!("unwrap called on MaybeAborted::Aborted"),
        }
    }
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
    #[error("the game version is not supported")]
    UnsupportedVersion,
    #[error("the image file is invalid")]
    BadImage,
    #[error("stack-start param is invalid")]
    InvalidStackStart,
    #[error("pmdm-addr param is invalid")]
    InvalidPmdmAddr,
    #[error(
        "the custom image provided has program-start = {0}, which does not match the one requested by the environment = {0}"
    )]
    ProgramStartMismatch(String, String),
    #[error("we don't support heap this big right now")]
    HeapTooBig,
    #[error("failed to initialize the process")]
    InitializeProcess,
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
    #[error("you are already on this screen so transitioning has no effect")]
    UselessScreenTransition,
    #[error("you cannot do this on this screen")]
    NotRightScreen,
    #[error("cannot auto switch screen because screen was switched manually")]
    CannotAutoSwitchScreen,
    #[error(
        "the item in the inventory in this position is `{0}`, which does not match the input item `{1}`"
    )]
    ItemMismatch(String, String),
    #[error(
        "the item in the inventory in this position is `{0}`, which does not match the input category `{1:?}`"
    )]
    ItemMismatchCategory(String, crate::parser::cir::Category),
    #[error("cannot find this item in inventory")]
    CannotFindItem,
    #[error("cannot find this item in inventory (need `{0}` more)")]
    CannotFindItemNeedMore(usize),
    #[error("the item `{0}` is not sellable")]
    ItemNotSellable(String),
    #[error("cannot find this item on the ground")]
    CannotFindGroundItem,
    #[error("cannot find this item on the ground (need `{0}` more)")]
    CannotFindGroundItemNeedMore(usize),
    #[error("this requires `{0}` items, but only `{1}` items found")]
    NotEnoughForAllBut(usize, usize),
    #[error("the `all but` syntax did not achieve the desired result")]
    InaccurateAllBut,
    #[error("cannot hold more items")]
    CannotHoldMore,
    #[error("cannot do this while holding items in the overworld")]
    CannotDoWhileHolding,
    #[error("not holding any items")]
    NotHolding,
    // #[error("only materials can be held unless prompt entanglement is in effect")]
    // CannotHoldNonMaterial,
    #[error("cannot specify item position here")]
    PositionSpecNotAllowed,
    #[error("this meta property is ignored while matching")]
    UselessItemMatchProp,
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
