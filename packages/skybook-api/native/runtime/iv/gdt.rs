//! GameData (GDT) inventory view types

mod __impl {
    use crate::runtime::iv;
    use serde::Serialize;

    /// Inventory data stored in GameData (GDT)
    ///
    /// This contains the list of items in GDT as well as other useful flags
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_Gdt {
        pub items: Vec<InvView_GdtItem>,
        /// Master Sword flags
        pub master_sword: InvView_GdtMasterSword,
        /// Other inventory flags
        pub info: InvView_GdtInvInfo,
    }

    /// Master Sword GDT flags
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_GdtMasterSword {
        /// The Open_MasterSword_FullPower flag
        pub is_true_form: bool,
        /// The MasterSword_Add_Power flag
        pub add_power: i32,
        /// The MasterSword_Add_BeamPower flag
        pub add_beam_power: i32,
        /// The MasterSwordRecoverTime flag
        pub recover_time: f32,
    }

    /// Other inventory GDT flags
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_GdtInvInfo {
        /// Number of weapon slots available per tab (WeaponPorchStockNum)
        pub num_weapon_slots: i32,
        /// Number of bow slots available per tab (BowPorchStockNum)
        pub num_bow_slots: i32,
        /// Number of shields slots available per tab (ShieldPorchStockNum)
        pub num_shield_slots: i32,

        // discovered flags
        pub sword_tab_discovered: bool,
        pub bow_tab_discovered: bool,
        pub shield_tab_discovered: bool,
        pub armor_tab_discovered: bool,
        pub material_tab_discovered: bool,
        pub food_tab_discovered: bool,
        pub key_item_tab_discovered: bool,
    }

    /// One item in GDT
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_GdtItem {
        /// Common item info
        pub common: iv::CommonItem,
        /// Index of the item in the GDT (0-419)
        pub idx: u32,
        /// Extra data that will be loaded from GDT based on the item type
        pub data: InvView_GdtItemData,
    }

    /// Extra flags to load for GDT items
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase", tag = "type")]
    #[allow(non_camel_case_types)]
    pub enum InvView_GdtItemData {
        /// No extra data for this item
        #[default]
        None,
        /// Sword info, loaded from PorchSword_FlagSp and PorchSword_ValueSp
        Sword {
            /// Index of the data in the PorchSword_* array
            idx: u32,
            /// modifier data loaded from GDT
            info: iv::WeaponModifier,
        },
        /// Bow info, loaded from PorchBow_FlagSp and PorchBow_ValueSp
        Bow {
            /// Index of the data in the PorchBow_* array
            idx: u32,
            /// modifier data loaded from GDT
            info: iv::WeaponModifier,
        },
        /// Shield info, loaded from PorchShield_FlagSp and PorchShield_ValueSp
        Shield {
            /// Index of the data in the PorchShield_* array
            idx: u32,
            /// modifier data loaded from GDT
            info: iv::WeaponModifier,
        },
        /// Food info, loaded from StaminaRecover, CookEffect0, and CookEffect1 flags
        Food {
            /// Index of the data in the various GDT array that stores food info
            idx: u32,
            /// Food info loaded from GDT
            info: iv::ItemData,
            /// The y value of CookEffect1
            unused_effect_1y: f32,
            /// The ingredient names, loaded from CookMaterialX flags, where X is 0-4
            ingredients: [String; 5],
        },
    }
}
pub use __impl::InvView_Gdt as Gdt;
pub use __impl::InvView_GdtInvInfo as GdtInvInfo;
pub use __impl::InvView_GdtItem as GdtItem;
pub use __impl::InvView_GdtItemData as GdtItemData;
pub use __impl::InvView_GdtMasterSword as GdtMasterSword;
