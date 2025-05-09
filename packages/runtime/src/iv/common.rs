//! Common structs for the inventory view

mod __impl {
    use serde::Serialize;

    /// Cook or weapon data in pouch
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_ItemData {
        /// This is either the weapon modifier value,
        /// or the HP recovery value for food (in quarter-hearts)
        ///
        /// This is the x value of StaminaRecover flag in GDT
        pub effect_value: i32,

        /// For food with a timed effect, this is the duration in seconds.
        /// For stamina, this is the raw value
        ///
        /// This is the y value of StaminaRecover flag in GDT
        pub effect_duration: i32,

        /// For weapon modifier, this is the flag bitset. For food,
        /// this is the sell price
        ///
        /// This is the x value of CookEffect1 flag in GDT
        pub sell_price: i32,

        /// Effect ID for the food
        ///
        /// Note this is raw memory value and may not be a valid enum value
        ///
        /// This is the x value of CookEffect0 flag in GDT
        pub effect_id: f32,

        /// The level of the effect, *usually* 1-3. However this
        /// is the raw memory value and may not be valid
        ///
        /// This is the y value of CookEffect0 flag in GDT
        pub effect_level: f32,
    }

    /// Weapon modifier info, which is a bitflag for modifier type and a modifier value
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_WeaponModifier {
        /// The weapon modifier type bit flag
        pub flag: i32,
        /// The value of the modifier
        pub value: i32,
    }

    /// Common (display) info for an item
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_CommonItem {
        /// Name of the item actor
        ///
        /// This is stored in PouchItem::mName, or the
        /// PorchItem flag
        pub actor_name: String,

        /// Raw value of the item, could be count or durability
        ///
        /// This is stored in PouchItem::mValue, or the
        /// PorchItem_Value1 flag
        pub value: i32,

        /// Equip flag
        ///
        /// This is PouchItem::mEquipped or the PorchItem_EquipFlag flag
        pub is_equipped: bool,
    }
}
pub use __impl::InvView_CommonItem as CommonItem;
pub use __impl::InvView_ItemData as ItemData;
pub use __impl::InvView_WeaponModifier as WeaponModifier;
