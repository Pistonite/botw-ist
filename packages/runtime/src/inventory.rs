use std::collections::BTreeMap;

/// Pointer interop type
///
/// This uses a u128 internal storage for a u64 value
/// to force generated bindings to convert the value to bigint
/// instead of number when sending to JS.
#[cfg(any(feature = "wasm", feature = "__ts-binding"))]
#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
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


/// List view of the inventory.
///
/// In this view, the inventory is represented as a vector
/// of items. Unallocated items are not included in the view.
///
/// Only valid if all internal PouchItem pointer references
/// are valid in PMDM, and mListHeads references (PouchItem**)
/// are either nullptr, or points to element in mTabs
#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct InventoryListView {
    pub info: InventoryInfo,
    pub items: Vec<ItemSlotInfo>,
}

/// Inventory data stored in GameData (GDT)
///
/// This contains the list of items in GDT as well as other useful flags
#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct GameDataInventory {
    pub items: Vec<ItemSlotInfo>,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct InventoryGraphView {
    pub info: InventoryInfo,
    pub list1: OffsetListInfo,
    pub list2: OffsetListInfo,
    /// Node address to the item slot info read from memory for that node
    ///
    /// Note the addresses are for the *node*, not the PouchItem
    #[cfg_attr(feature = "__ts-binding", ts(type = "Map<Pointer, ItemSlotInfo>"))]
    pub nodes: BTreeMap<Pointer, ItemSlotInfo>,
}

#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct OffsetListInfo {
    /// Address of the mStartEnd node of the list
    pub head_ptr: Pointer,
    /// mStartEnd::mPrev
    pub prev_node_ptr: Pointer,
    /// mStartEnd::mNext
    pub next_node_ptr: Pointer,
    /// mCount
    pub count: i32,
    /// mOffset
    pub offset: i32,
}

/// Common inventory info in different inventory views
#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct InventoryInfo {

    /// Number of tabs (mNumTabs). Should be the length
    /// of the valid section of mTabs and mTabsType
    pub num_tabs: i32,

    pub tabs: Vec<TabInfo>,

}

#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct TabInfo {
    #[cfg_attr(feature = "__ts-binding", ts(type = "number | undefined"))]
    pub item_idx: Option<usize>, // maybe None in GraphView if the ptr is bad
    pub item_ptr: Pointer, // +8 to get the node ptr / maybe 0 (nullptr)
    pub tab_type: i32, // maybe -1, if num_tabs is corrupted
}

/// Information for an item slot in the inventory
///
/// This info is extracted from memory and is used
/// to display the item slot. These data are derived
/// from the PouchItem class from decompilation project.
/// Data that can be looked up from the item's parameters
/// should not be included
#[derive(Debug, Default, Clone, serde::Serialize)]
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

    /// If the item is unaligned (data read from a pointer that is not
    /// a valid PouchItem* to the pool)
    pub unaligned: bool,

    /// Address of the previous node (may not be valid)
    pub prev_node_ptr: Pointer,

    /// Address of the next node (may not be valid)
    pub next_node_ptr: Pointer,

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
