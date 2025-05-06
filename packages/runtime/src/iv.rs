
mod __impl {
    use serde::Serialize;

    use crate::pointer::Pointer;

    /// List view of the Pouch Inventory.
    ///
    /// In this view, the inventory is represented as a vector
    /// of items. Unallocated items are not included in the view.
    ///
    /// Only valid if all internal PouchItem pointer references
    /// are valid in PMDM, and mListHeads references (PouchItem**)
    /// are either nullptr, or points to element in mTabs
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_PouchList {
        pub info: InvView_PouchData,
        pub items: Vec<InvView_PouchItem>,
        pub count: i32,
    }

    /// Common Pouch (PMDM) fields in list and graph views
    #[derive(Debug, Default, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_PouchData {
        /// Number of tabs (mNumTabs). Should be the length
        /// of the valid section of mTabs and mTabsType
        pub num_tabs: i32,

        pub tabs: Vec<InvView_PouchTab>,

    }

    /// Data from mTabs and mTabsType in PMDM
    #[derive(Debug, Default, Clone, serde::Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_PouchTab {
        #[cfg_attr(feature = "__ts-binding", ts(type = "number | undefined"))]
        pub item_idx: Option<usize>, // maybe None in GraphView if the ptr is bad
        pub item_ptr: Pointer, // +8 to get the node ptr / maybe 0 (nullptr)
        pub tab_type: i32, // maybe -1, if num_tabs is corrupted
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
    #[allow(non_camel_case_types)]
    pub struct InvView_Gdt {
        pub items: Vec<InvView_GdtItem>,

        /// Master Sword status
        pub master_sword: InvView_GdtMasterSword,
    }

    /// Master Sword status stored in GDT
    #[derive(Debug, Default, Clone, serde::Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
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

    /// View of the items in the overworld (technically not inventory, but convienient to think of
    /// it this way)
    #[derive(Debug, Default, Clone, serde::Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_Overworld {
        pub items: Vec<InvView_OverworldItem>,
    }

    /// Info for an item in the PMDM. This struct can represent both
    /// valid item and invalid items (resulting from ISU corruption)
    #[derive(Debug, Default, Clone, serde::Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_PouchItem {
        /// Common item info
        pub common: InvView_CommonItem,

        /// PouchItem::mType
        ///
        /// Note this is raw memory value and may not be a valid enum value
        pub item_type: i32,

        /// PouchItem::mItemUse
        ///
        /// Note this is raw memory value and may not be a valid enum value
        pub item_use: i32,

        /// PouchItem::mInInventory
        pub is_in_inventory: bool,

        /// For animated items, if this slot would have no icon in the inventory
        pub is_no_icon: bool,

        /// Extra data (CookData or WeaponData) for the item
        pub data: InvView_ItemData,

        /// Ingredients of the item
        pub ingredients: [String; 5],

        /// Number of items held if the item is being held by the player
        pub holding_count: u8,

        /// Enable the prompt entangled state for this slot
        pub prompt_entangled: bool,

        /// Physical address (pointer) of the node.
        ///
        /// This is address of the list node, not the PouchItem.
        /// The PouchItem pointer can be obtained by subtracting 8 from this value
        pub node_addr: Pointer,

        /// Is this a valid node, in the item array
        pub node_valid: bool,

        /// Position of the node
        /// 
        /// If the node is valid, this is the index of the node in the item array.
        /// Otherwise, this is the byte offset (ptrdiff) of the node from beginning of PMDM
        pub node_pos: i128,

        /// Pointer to the previous node
        pub node_prev: Pointer,

        /// Pointer to the next node
        pub node_next: Pointer,

        // both allocated_idx and unallocated_idx is here because
        // it could be theoretically possible that both lists eventually
        // reach this node, as a result of ISU corruption

        /// Position of the node in the allocated list.
        /// i.e. how many times `.next` needs to be followed from the head of the list 
        /// to reach this node.
        ///
        /// If this node is not reachable from the head of the list by following `.next` , this is -1
        pub allocated_idx: i32,

        /// Position of the node in the unallocated list.
        /// i.e. how many times `.next` needs to be followed from the head of the list 
        /// to reach this node.
        ///
        /// If this node is not reachable from the head of the list by following `.next` , this is -1
        pub unallocated_idx: i32,

    }

#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
pub struct InvView_GdtItem {
    /// Common item info
    pub common: InvView_CommonItem,
    /// Index of the item in the GDT (0-419)
    pub idx: u32,
    /// Extra data that will be loaded from GDT based on the item type
    pub data: InvView_GdtItemData,
}

/// Extra flags to load for GDT items
#[derive(Debug, Default, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
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
        info: InvView_WeaponModifier
    },
    /// Bow info, loaded from PorchBow_FlagSp and PorchBow_ValueSp
    Bow { 
        /// Index of the data in the PorchBow_* array
        idx: u32, 
        /// modifier data loaded from GDT
        info: InvView_WeaponModifier
    },
    /// Shield info, loaded from PorchShield_FlagSp and PorchShield_ValueSp
    Shield { 
        /// Index of the data in the PorchShield_* array
        idx: u32, 
        /// modifier data loaded from GDT
        info: InvView_WeaponModifier
    },
    /// Food info, loaded from StaminaRecover, CookEffect0, and CookEffect1 flags
    Food { 
        /// Index of the data in the various GDT array that stores food info
        idx: u32, 
        /// Food info loaded from GDT
        info: InvView_ItemData, 
        /// The y value of CookEffect1
        unused_effect_1y: f32, 
        /// The ingredient names, loaded from CookMaterialX flags, where X is 0-4
        ingredients: [String; 5]
    },
}

    /// Item info for something in the overworld
#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
#[cfg_attr(feature = "__ts-binding", ts(export))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
#[allow(non_camel_case_types)]
    pub enum InvView_OverworldItem {
        /// Equipment on the player (weapons and armors - actor name and value)
        ///
        /// Modifier is 0,0 if no modifier
        Equipped { 
           actor: String, value: i32, modifier: InvView_WeaponModifier },
        /// Holding in your hand (holding one)
        Held { actor: String},
        /// Equipment on the ground (actor name and value)
        ///
        /// Modifier is 0,0 if no modifier
        GroundEquipment{actor:String, value:i32, modifier: InvView_WeaponModifier},
        /// Other items dropped on the ground
        GroundItem{actor:String},
    }


    /// Common (display) info for an item
    #[derive(Debug, Default, Clone, serde::Serialize)]
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

    /// Weapon modifier info, which is a bitflag for modifier type and a modifier value
    #[derive(Debug, Default, Clone, serde::Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_WeaponModifier {
        pub flag: i32,
        pub value: i32,
    }

    /// Cook or weapon data
    #[derive(Debug, Default, Clone, serde::Serialize)]
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
}
pub use __impl::InvView_PouchList as PouchList;
pub use __impl::InvView_PouchData as PouchData;
pub use __impl::InvView_PouchItem as PouchItem;
pub use __impl::InvView_PouchTab as PouchTab;
pub use __impl::InvView_Gdt as Gdt;
pub use __impl::InvView_GdtItem as GdtItem;
pub use __impl::InvView_GdtItemData as GdtItemData;
pub use __impl::InvView_GdtMasterSword as GdtMasterSword;
pub use __impl::InvView_Overworld as Overworld;
pub use __impl::InvView_OverworldItem as OverworldItem;
pub use __impl::InvView_CommonItem as CommonItem;
pub use __impl::InvView_ItemData as ItemData;
pub use __impl::InvView_WeaponModifier as WeaponModifier;





// not worrying about supporting graph view yet

// #[derive(Debug, Default, Clone, serde::Serialize)]
// #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
// #[cfg_attr(feature = "__ts-binding", ts(export))]
// #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
// #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
// #[serde(rename_all = "camelCase")]
// pub struct PouchGraphView {
//     pub info: InventoryInfo,
//     pub list1: OffsetListInfo,
//     pub list2: OffsetListInfo,
//     /// Node address to the item slot info read from memory for that node
//     ///
//     /// Note the addresses are for the *node*, not the PouchItem
//     #[cfg_attr(feature = "__ts-binding", ts(type = "Map<Pointer, PouchItemInfo>"))]
//     pub nodes: BTreeMap<Pointer, PouchItemInfo>,
// }
//
// #[derive(Debug, Default, Clone, serde::Serialize)]
// #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
// #[cfg_attr(feature = "__ts-binding", ts(export))]
// #[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
// #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
// #[serde(rename_all = "camelCase")]
// pub struct OffsetListInfo {
//     /// Address of the mStartEnd node of the list
//     pub head_ptr: Pointer,
//     /// mStartEnd::mPrev
//     pub prev_node_ptr: Pointer,
//     /// mStartEnd::mNext
//     pub next_node_ptr: Pointer,
//     /// mCount
//     pub count: i32,
//     /// mOffset
//     pub offset: i32,
// }



