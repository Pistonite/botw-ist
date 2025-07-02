//! Item data for the pouch

mod __impl {
    use crate::runtime::iv;
    use serde::Serialize;

    /// List view of the Pouch Inventory.
    ///
    /// In this view, the inventory is represented as a vector
    /// of items. Unallocated items are not included in the view.
    ///
    /// This view can only available if PMDM is not corrupted
    #[derive(Debug, Clone, PartialEq, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_PouchList {
        /// Count of list1, as set in list1 (i.e. mCount)
        pub count: i32,
        /// Actual items in list1
        pub items: Vec<InvView_PouchItem>,
        /// If tab data is valid (no overflow is detected)
        pub are_tabs_valid: bool,
        /// Number of tabs (mNumTabs). Should be the length
        /// of the valid section of mTabs and mTabsType.
        pub num_tabs: i32,
        /// The actual tabs (mTabs and mTabsType), up to the tab
        /// where both mTabs[i] is nullptr and mTabsType[i] is -1
        pub tabs: Vec<InvView_PouchTab>,
        /// Type of the screen currently on
        pub screen: InvView_Screen,
    }

    impl Default for InvView_PouchList {
        fn default() -> Self {
            Self {
                count: 0,
                items: vec![],
                are_tabs_valid: true,
                num_tabs: 0,
                tabs: vec![],
                screen: InvView_Screen::default(),
            }
        }
    }

    /// Data from mTabs and mTabsType in PMDM
    ///
    /// Only available if PMDM is not corrupted
    #[derive(Debug, Default, PartialEq, Clone, Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_PouchTab {
        /// Index of the item in the list.
        ///
        /// -1 if nullptr, which is when the tab is empty
        pub item_idx: i32,
        /// The type of the tab (in mTabsType), -1 if invalid
        pub tab_type: i32,
        // TODO: do we need num items in the tab here?
    }

    /// Type of screen currently being shown. This is technically
    /// not part of the pouch, but easier to think this way
    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy, serde::Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub enum InvView_Screen {
        #[default]
        Overworld,
        Inventory,
        Shop,
    }

    /// Info for an item in the PMDM. This struct can represent both
    /// valid item and invalid items (resulting from ISU corruption)
    #[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]
    #[cfg_attr(feature = "__ts-binding", derive(ts_rs::TS))]
    #[cfg_attr(feature = "__ts-binding", ts(export))]
    #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    #[serde(rename_all = "camelCase")]
    #[allow(non_camel_case_types)]
    pub struct InvView_PouchItem {
        /// Common item info
        pub common: iv::CommonItem,

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
        pub data: iv::ItemData,

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
        pub node_addr: iv::Pointer,

        /// Is this a valid node, in the item array
        pub node_valid: bool,

        /// Position of the node
        ///
        /// If the node is valid, this is the index of the node in the item array.
        /// Otherwise, this is the byte offset (ptrdiff) of the node from beginning of PMDM
        pub node_pos: i128,

        /// Pointer to the previous node
        pub node_prev: iv::Pointer,

        /// Pointer to the next node
        pub node_next: iv::Pointer,

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

        /// If the tab data is valid, the index of the tab this item is in.
        /// Note that this may not be consecutive for consecutive items,
        /// as there could be empty tabs
        pub tab_idx: i32,

        /// If the tab data is valid, the slot of the item in the tab.
        ///
        /// This is usually 0-20. For arrows, this is the actual position to be displayed
        /// (i.e. first arrow would be 5 if there are 5 bow slots, including empty)
        pub tab_slot: i32,

        /// If the item is accessible (visible) in the inventory
        ///
        /// Not accessible cases include:
        /// - mCount is 0, whole inventory is not accessible
        /// - Weapon/Bow/Shield is outside of the slot range
        pub accessible: bool,

        /// If the item is accessible via the dpad menu
        pub dpad_accessible: bool,
    }
}
pub use __impl::InvView_PouchItem as PouchItem;
pub use __impl::InvView_PouchList as PouchList;
pub use __impl::InvView_PouchTab as PouchTab;
pub use __impl::InvView_Screen as Screen;
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
