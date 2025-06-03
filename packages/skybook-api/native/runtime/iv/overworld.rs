//! Item data for overworld actors

mod __impl {
    use crate::runtime::iv;
    use serde::Serialize;

    /// View of the items in the overworld (technically not inventory, but convienient to think of
    /// it this way)
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_Overworld {
        pub items: Vec<InvView_OverworldItem>,
    }

    /// Item info for something in the overworld
    #[derive(Debug, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "kebab-case", tag = "type")]
    #[allow(non_camel_case_types)]
    pub enum InvView_OverworldItem {
        /// Equipment on the player (weapons and armors - actor name and value)
        ///
        /// Modifier is 0,0 if no modifier
        Equipped {
            actor: String,
            value: i32,
            modifier: iv::WeaponModifier,
        },
        /// Holding in your hand (holding one)
        Held { actor: String },
        /// Equipment on the ground (actor name and value)
        ///
        /// Modifier is 0,0 if no modifier
        GroundEquipment {
            actor: String,
            value: i32,
            modifier: iv::WeaponModifier,
        },
        /// Other items dropped on the ground
        GroundItem { actor: String },
    }
}
pub use __impl::InvView_Overworld as Overworld;
pub use __impl::InvView_OverworldItem as OverworldItem;
