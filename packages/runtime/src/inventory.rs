/// Information for an item slot in the inventory
///
/// This info is extracted from memory and is used
/// to display the item slot. These data are derived
/// from the PouchItem class from decompilation project.
/// Data that can be looked up from the item's parameters
/// should not be included
#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct ItemSlotInfo {
    /// Name of the actor, from PouchItem::mName
    ///
    /// This is what will be used to look up extra data for the item
    pub actor_name: String,

    /// PouchItem::mType
    ///
    /// Note this is raw memory value and may not be a valid enum value
    pub item_type: i32,

    /// PouchItem::mItemUse
    ///
    /// Note this is raw memory value and may not be a valid enum value
    pub item_use: i32,

    /// PouchItem::mValue
    ///
    /// This is stack size or durability * 100
    pub value: i32,

    /// PouchItem::mEquipped
    pub is_equipped: bool,

    /// PouchItem::mInInventory
    pub is_in_inventory: bool,

    /// This is either the weapon modifier value,
    /// or the HP recovery value for food (in quarter-hearts)
    pub mod_effect_value: i32,

    /// For food with a timed effect, this is the duration in seconds.
    /// For stamina, this is the raw value
    pub mod_effect_duration: i32,

    /// For weapon modifier, this is the flag bitset. For food,
    /// this is the sell price
    pub mod_sell_price: i32,

    /// Effect ID for the food
    ///
    /// Note this is raw memory value and may not be a valid enum value
    pub mod_effect_id: f32,

    /// The level of the effect, *usually* 1-3. However this
    /// is the raw memory value and may not be valid
    pub mod_effect_level: f32,

    /// PouchItem::mIngredients. Length should always be 5
    pub ingredient_actor_names: [String; 5],

    /// The item's position in the item list.
    ///
    /// If the item is in the unallocated pool, this is its position
    /// in the unallocated pool (stack). 0 is the top of the stack/beginning
    /// of the list
    pub list_pos: u16,

    /// If the item is currently in the unallocated pool
    pub unallocated: bool,

    /// The item's position in the pool
    ///
    /// This basically serves as a unique pointer to the item
    pub pool_pos: u16,

    /// If the item is in "broken" slot, i.e. will be transferred on reload
    pub is_in_broken_slot: bool,

    /// Number of items held if the item is being held by the player
    pub holding_count: u8,

    /// Enable the prompt entangled state for this slot
    pub prompt_entangled: bool,
}
