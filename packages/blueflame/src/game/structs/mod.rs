#![allow(clippy::too_many_arguments)]

mod gdt;
pub use gdt::*;
mod string;
pub use string::*;
mod pouch;
pub use pouch::*;

use std::fmt;
use std::marker::PhantomData;

use derive_more::derive::Constructor;

#[layered_crate::import]
use game::super_::memory::{MemObject, Ptr};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum PouchItemType {
    Sword = 0x0,
    Bow = 0x1,
    Arrow = 0x2,
    Shield = 0x3,
    ArmorHead = 0x4,
    ArmorUpper = 0x5,
    ArmorLower = 0x6,
    Material = 0x7,
    Food = 0x8,
    KeyItem = 0x9,
    Invalid = -1,
}

impl From<i32> for PouchItemType {
    fn from(value: i32) -> Self {
        match value {
            0x0 => PouchItemType::Sword,
            0x1 => PouchItemType::Bow,
            0x2 => PouchItemType::Arrow,
            0x3 => PouchItemType::Shield,
            0x4 => PouchItemType::ArmorHead,
            0x5 => PouchItemType::ArmorUpper,
            0x6 => PouchItemType::ArmorLower,
            0x7 => PouchItemType::Material,
            0x8 => PouchItemType::Food,
            0x9 => PouchItemType::KeyItem,
            -1 => PouchItemType::Invalid,
            _ => PouchItemType::Invalid, // Handle invalid values
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum ItemUse {
    WeaponSmallSword = 0x0,
    WeaponLargeSword = 0x1,
    WeaponSpear = 0x2,
    WeaponBow = 0x3,
    WeaponShield = 0x4,
    ArmorHead = 0x5,
    ArmorUpper = 0x6,
    ArmorLower = 0x7,
    Item = 0x8,
    ImportantItem = 0x9,
    CureItem = 0xa,
    Invalid = -1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum PouchCategory {
    Sword = 0x0,
    Bow = 0x1,
    Shield = 0x2,
    Armor = 0x3,
    Material = 0x4,
    Food = 0x5,
    KeyItem = 0x6,
    Invalid = -1,
}

#[allow(non_snake_case)]
#[derive(MemObject, Default, Clone)]
#[size(0x10)]
pub struct GrabbedItemInfo {
    #[offset(0x0)]
    item: Ptr![PouchItem],
    #[offset(0x8)]
    _8: bool,
    #[offset(0x9)]
    _9: bool,
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x10)]
pub struct ListNode {
    #[offset(0x0)]
    pub mPrev: Ptr![ListNode],
    #[offset(0x8)]
    pub mNext: Ptr![ListNode],
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy, Debug)]
#[size(0x8)]
pub struct Vector2f {
    #[offset(0x0)]
    pub x: f32,
    #[offset(0x4)]
    pub y: f32,
}

impl PartialEq<Vec<f32>> for Vector2f {
    fn eq(&self, other: &Vec<f32>) -> bool {
        other.len() == 2 && self.x == other[0] && self.y == other[1]
    }
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x14)]
pub struct CookData {
    #[offset(0x0)]
    pub mHealthRecover: i32,
    #[offset(0x4)]
    pub mEffectDuration: i32,
    #[offset(0x8)]
    pub mSellPrice: i32,
    #[offset(0xc)]
    pub mEffect: Vector2f,
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x14)]
pub struct WeaponData {
    #[offset(0x0)]
    mModifierValue: u32,
    #[offset(0x8)]
    mModifier: u32,
}

impl From<CookData> for WeaponData {
    fn from(cook: CookData) -> Self {
        WeaponData {
            mModifierValue: cook.mSellPrice as u32,
            mModifier: cook.mHealthRecover as u32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum CookEffectId {
    None = -1,
    LifeRecover = 1,
    LifeMaxUp = 2,
    ResistHot = 4,
    ResistCold = 5,
    ResistElectric = 6,
    AttackUp = 10,
    DefenseUp = 11,
    Quietness = 12,
    MovingSpeed = 13,
    GutsRecover = 14,
    ExGutsMaxUp = 15,
    Fireproof = 16,
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Constructor)]
#[size(0x288)]
pub struct CookItem {
    #[offset(0x0)]
    actor_name: FixedSafeString40,
    #[offset(0x58)]
    ingredients: [FixedSafeString40; 5],
    #[offset(0x210)]
    life_recover: f32,
    #[offset(0x214)]
    effect_time: i32,
    #[offset(0x218)]
    sell_price: i32,
    #[offset(0x21c)]
    effect_id: i32,
    #[offset(0x220)]
    vitality_boost: f32,
    #[offset(0x224)]
    is_crit: bool,
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x10)]
struct PtrArrayImpl {
    #[offset(0x0)]
    mPtrNum: i32,
    #[offset(0x4)]
    mPtrNumMax: i32,
    #[offset(0x8)]
    mPtrs: Ptr![Ptr![FixedSafeString40][5]],
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x8)]
struct FreeListNode {
    #[offset(0x0)]
    nextFree: Ptr![FreeListNode],
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x10)]
struct FreeList {
    #[offset(0x0)]
    mFree: FreeListNode,
    #[offset(0x8)]
    mWork: Ptr![FixedSafeString40],
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x20)]
struct SafeStringArray {
    #[offset(0x0)]
    base: PtrArrayImpl,
    #[offset(0x10)]
    mFreeList: FreeList,
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone, Copy)]
#[size(0x200)]
struct FixedSafeStringArray {
    #[offset(0x0)]
    base: SafeStringArray,
    #[offset(0x20)]
    mWork: [u8; 480],
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone)]
#[size(0x298)]
pub struct PouchItem {
    #[offset(0x8)]
    mListNode: ListNode,
    // PouchItemType
    #[offset(0x18)]
    mType: i32,
    // ItemUse
    #[offset(0x1c)]
    mItemUse: i32,
    #[offset(0x20)]
    pub mValue: i32,
    #[offset(0x24)]
    mEquipped: bool,
    #[offset(0x25)]
    mInInventory: bool,
    #[offset(0x28)]
    mName: FixedSafeString40,
    #[offset(0x80)]
    pub mData: CookData,
    #[offset(0x98)]
    mIngredients: FixedSafeStringArray,
}

impl PouchItem {
    pub fn get_name(&self) -> String {
        self.mName.to_string()
    }

    pub fn get_type(&self) -> PouchItemType {
        match self.mType {
            0x0 => PouchItemType::Sword,
            0x1 => PouchItemType::Bow,
            0x2 => PouchItemType::Arrow,
            0x3 => PouchItemType::Shield,
            0x4 => PouchItemType::ArmorHead,
            0x5 => PouchItemType::ArmorUpper,
            0x6 => PouchItemType::ArmorLower,
            0x7 => PouchItemType::Material,
            0x8 => PouchItemType::Food,
            0x9 => PouchItemType::KeyItem,
            -1 => PouchItemType::Invalid,
            _ => PouchItemType::Invalid, // Handle invalid values
        }
    }

    pub fn get_use(&self) -> ItemUse {
        match self.mItemUse {
            0x0 => ItemUse::WeaponSmallSword,
            0x1 => ItemUse::WeaponLargeSword,
            0x2 => ItemUse::WeaponSpear,
            0x3 => ItemUse::WeaponBow,
            0x4 => ItemUse::WeaponShield,
            0x5 => ItemUse::ArmorHead,
            0x6 => ItemUse::ArmorUpper,
            0x7 => ItemUse::ArmorLower,
            0x8 => ItemUse::Item,
            0x9 => ItemUse::ImportantItem,
            0xa => ItemUse::CureItem,
            _ => ItemUse::Invalid,
        }
    }

    pub fn is_weapon(&self) -> bool {
        matches!(self.mItemUse, 0x0..=0x4)
    }
}

impl fmt::Display for PouchItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PouchItem {0}:\n -Type: {1:?}\n -Use: {2:?}\n -Value: {3}\n -Equipped: {4}\n -InInventory: {5}",
            self.get_name(),
            self.get_type(),
            self.get_use(),
            self.mValue,
            self.mEquipped,
            self.mInInventory
        )
    }
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone)]
#[size(0x18)]
pub struct PouchItemOffsetList {
    #[offset(0x0)]
    pub mStartEnd: ListNode,
    #[offset(0x10)]
    pub mCount: i32,
    #[offset(0x14)]
    mOffset: i32,
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone)]
#[size(0x44190)]
pub struct PauseMenuDataMgrLists {
    #[offset(0x0)]
    pub list1: PouchItemOffsetList,
    #[offset(0x18)]
    list2: PouchItemOffsetList,
    // #[offset(0x30)]
    // buffer: [PouchItem; 420],
}

impl PauseMenuDataMgrLists {
    pub fn get_active_item_iter(&self) -> OffsetListIter<PouchItem> {
        self.list1.to_iter()
    }

    pub fn get_inactive_item_iter(&self) -> OffsetListIter<PouchItem> {
        self.list2.to_iter()
    }
}

#[allow(non_snake_case)]
#[derive(MemObject, Clone)]
#[size(0x44808)]
pub struct PauseMenuDataMgr {
    #[offset(0x68)]
    pub mItemLists: PauseMenuDataMgrLists,
    #[offset(0x441f8)]
    mListHeads: [Ptr![Ptr![PouchItem]]; 7],
    #[offset(0x44230)]
    mTabs: [Ptr![PouchItem]; 50],
    // PouchItemType
    #[offset(0x443c0)]
    mTabsType: [i32; 50],
    #[offset(0x44488)]
    pub mLastAddedItem: Ptr![PouchItem],
    #[offset(0x44490)]
    mLastAddedItemTab: i32,
    #[offset(0x44494)]
    mLastAddedItemSlot: i32,
    #[offset(0x44498)]
    mNumTabs: i32,
    #[offset(0x444a0)]
    mGrabbedItems: [GrabbedItemInfo; 5],
    #[offset(0x44518)]
    mRitoSoulItem: Ptr![PouchItem],
    #[offset(0x44520)]
    mGoronSoulItem: Ptr![PouchItem],
    #[offset(0x44528)]
    mZoraSoulItem: Ptr![PouchItem],
    #[offset(0x44530)]
    mGerudoSoulItem: Ptr![PouchItem],
    #[offset(0x44540)]
    mNewlyAddedItem: PouchItem,
    #[offset(0x447d8)]
    mIsPouchForQuest: bool,
    #[offset(0x447e0)]
    mEquippedWeapons: [Ptr![PouchItem]; 4],
    // PouchCategory
    #[offset(0x44800)]
    pub mCategoryToSort: i32,
}

// impl PauseMenuDataMgr {
//     pub fn get_last_item_added_name(&self, mem: &Memory) -> Result<String, Error> {
//         let last_pouch_item = self.mLastAddedItem.deref(mem)?;
//         Ok(last_pouch_item.mName.to_string())
//     }
//
//     pub fn get_newly_added_name(&self) -> String {
//         self.mNewlyAddedItem.get_name()
//     }
//
//     pub fn get_newly_added_item(&self) -> PouchItem {
//         self.mNewlyAddedItem
//     }
//
//     pub fn get_active_item_iter(&self) -> OffsetListIter<PouchItem> {
//         self.mItemLists.get_active_item_iter()
//     }
//
//     pub fn get_inactive_item_iter(&self) -> OffsetListIter<PouchItem> {
//         self.mItemLists.get_inactive_item_iter()
//     }
// }

#[allow(non_snake_case)]
#[derive(MemObject, Clone)]
#[size(0x98)]
pub struct InfoData {
    #[offset(0x40)]
    pub mHashesBytes: Ptr![u8],
    #[offset(0x48)]
    pub mHashes: Ptr![u32],
    #[offset(0x50)]
    pub mActorsBytes: Ptr![u8],
    #[offset(0x58)]
    pub mActorOffsets: Ptr![u32],
    #[offset(0x60)]
    pub mTagsIdx: i32,
    #[offset(0x78)]
    pub mNumActors: i32,
}

pub struct OffsetListIter<T> {
    start_end_node: ListNode,
    current_node: Ptr![ListNode],
    pub offset: i32,
    pub count: i32,
    index: i32,
    _marker: PhantomData<T>,
}

impl PouchItemOffsetList {
    pub fn to_iter(&self) -> OffsetListIter<PouchItem> {
        OffsetListIter::<PouchItem>::new(self.mStartEnd, self.mOffset, self.mCount)
    }
}

impl<T> OffsetListIter<T> {
    pub fn new(start_end_node: ListNode, offset: i32, count: i32) -> Self {
        OffsetListIter::<T> {
            start_end_node,
            current_node: start_end_node.mNext,
            _marker: PhantomData,
            offset,
            count,
            index: 0,
        }
    }
}

// impl OffsetListIter<PouchItem> {
//     pub fn get_entry(&self) -> Ptr<PouchItem> {
//         self.current_node.get_entry_ptr::<PouchItem>(self.offset)
//     }
//
//     pub fn next(&mut self, mem: &Memory) -> Result<Ptr<PouchItem>, Error> {
//         let return_node = self.get_entry();
//         self.index += 1;
//         self.current_node = self.current_node.deref(mem)?.mNext;
//         Ok(return_node)
//     }
//
//     pub fn has_next(&self) -> bool {
//         self.index < self.count
//     }
// }

// impl Ptr<ListNode> {
//     pub fn get_entry_ptr<T>(&self, offset: i32) -> Ptr<T> {
//         Ptr::new(
//             (self.get_addr() as i64 - (offset as i64))
//                 .try_into()
//                 .unwrap(),
//         )
//     }
// }

pub struct PorchWeaponData {
    pub modifier: i32,
    pub modifier_value: i32,
}

pub struct PorchCookData {
    pub health_recover: i32,
    pub effect_duration: i32,
    pub sell_price: i32,
    pub effect: (f32, f32),
    pub ingredients: [String; 5],
}

pub struct PorchItem {
    pub r#type: PouchItemType,
    pub value: i32,
    pub name: String,
    pub cook_data: Option<PorchCookData>,
    pub weapon_data: Option<PorchWeaponData>,
}
impl PorchItem {
    fn default(name: String, typ: PouchItemType, value: i32) -> Self {
        PorchItem {
            name,
            value,
            r#type: typ,
            cook_data: None,
            weapon_data: None,
        }
    }
    fn weapon(name: String, typ: PouchItemType, value: i32, data: PorchWeaponData) -> Self {
        PorchItem {
            name,
            value,
            r#type: typ,
            cook_data: None,
            weapon_data: Some(data),
        }
    }
    fn food(name: String, typ: PouchItemType, value: i32, data: PorchCookData) -> Self {
        PorchItem {
            name,
            value,
            r#type: typ,
            cook_data: Some(data),
            weapon_data: None,
        }
    }
}

// pub struct GameDataItemIter<'a,  'y, 'z> {
//     index: usize,
//     num_swords: usize,
//     num_bows: usize,
//     num_shields: usize,
//     num_food: usize,
//
//     core: &'a Core< 'y, 'z>,
//     pouch_item_lookup: PouchItemTypeLookup,
// }
// impl<'a,  'y, 'z> GameDataItemIter<'a,  'y, 'z> {
//     const MAX_LEN: usize = 420;
//
//     pub fn new(core: &'a Core< 'y, 'z>, pouch_item_lookup: PouchItemTypeLookup) -> Self {
//         GameDataItemIter {
//             index: 0,
//             num_swords: 0,
//             num_bows: 0,
//             num_shields: 0,
//             num_food: 0,
//
//             core,
//             pouch_item_lookup,
//         }
//     }
//
//     pub fn next_item(&mut self) -> Result<PorchItem, Error> {
//         let trigger_param_addr = self.core.mem.get_trigger_param_addr();
//         let trigger_param = self
//             .core
//             .proxies
//             .get_trigger_param(self.core.mem, trigger_param_addr)?;
//         let item_name = trigger_param
//             .get_string64_array_value("PorchItem", self.index)
//             .ok_or_else(|| {
//                 Error::Mem(crate::memory::Error::Unexpected(format!(
//                     "item at index {} does not exist",
//                     self.index
//                 )))
//             })?;
//         let typ = self.pouch_item_lookup.get_type(&item_name);
//         let value = trigger_param
//             .get_s32_array_value("PorchItem_Value1", self.index)
//             .ok_or_else(|| {
//                 Error::Mem(crate::memory::Error::Unexpected(format!(
//                     "item at index {} does not exist",
//                     self.index
//                 )))
//             })?;
//         match typ {
//             PouchItemType::Sword => {
//                 let modifier = trigger_param
//                     .get_s32_array_value("PorchSword_FlagSp", self.num_swords)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "sword at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let modifier_value = trigger_param
//                     .get_s32_array_value("PorchSword_ValueSp", self.num_swords)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "sword at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let weapon_data = PorchWeaponData {
//                     modifier,
//                     modifier_value,
//                 };
//                 self.num_swords += 1;
//                 self.index += 1;
//                 Ok(PorchItem::weapon(item_name, typ, value, weapon_data))
//             }
//             PouchItemType::Bow => {
//                 let modifier = trigger_param
//                     .get_s32_array_value("PorchBow_FlagSp", self.num_bows)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "bow at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let modifier_value = trigger_param
//                     .get_s32_array_value("PorchBow_ValueSp", self.num_bows)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "bow at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let weapon_data = PorchWeaponData {
//                     modifier,
//                     modifier_value,
//                 };
//                 self.num_bows += 1;
//                 self.index += 1;
//                 Ok(PorchItem::weapon(item_name, typ, value, weapon_data))
//             }
//             PouchItemType::Shield => {
//                 let modifier = trigger_param
//                     .get_s32_array_value("PorchShield_FlagSp", self.num_shields)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "shield at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let modifier_value = trigger_param
//                     .get_s32_array_value("PorchShield_ValueSp", self.num_shields)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "shield at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let weapon_data = PorchWeaponData {
//                     modifier,
//                     modifier_value,
//                 };
//                 self.num_shields += 1;
//                 self.index += 1;
//                 Ok(PorchItem::weapon(item_name, typ, value, weapon_data))
//             }
//             PouchItemType::Food => {
//                 let (health_recover, effect_duration) = trigger_param
//                     .get_vector2f_array_value("StaminaRecover", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let effect = trigger_param
//                     .get_vector2f_array_value("CookEffect0", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let (sell_price, _) = trigger_param
//                     .get_vector2f_array_value("CookEffect1", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m1 = trigger_param
//                     .get_string64_array_value("CookMaterialName0", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m2 = trigger_param
//                     .get_string64_array_value("CookMaterialName1", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m3 = trigger_param
//                     .get_string64_array_value("CookMaterialName2", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m4 = trigger_param
//                     .get_string64_array_value("CookMaterialName3", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 let m5 = trigger_param
//                     .get_string64_array_value("CookMaterialName4", self.num_food)
//                     .ok_or_else(|| {
//                         Error::Mem(crate::memory::Error::Unexpected(format!(
//                             "food at index {} does not exist",
//                             self.index
//                         )))
//                     })?;
//                 self.num_food += 1;
//                 self.index += 1;
//                 Ok(PorchItem::food(
//                     item_name,
//                     typ,
//                     value,
//                     PorchCookData {
//                         health_recover: health_recover as i32,
//                         effect_duration: effect_duration as i32,
//                         sell_price: sell_price as i32,
//                         effect,
//                         ingredients: [m1, m2, m3, m4, m5],
//                     },
//                 ))
//             }
//             PouchItemType::ArmorHead
//             | PouchItemType::ArmorUpper
//             | PouchItemType::ArmorLower
//             | PouchItemType::Arrow
//             | PouchItemType::Material
//             | PouchItemType::KeyItem
//             | PouchItemType::Invalid => {
//                 self.index += 1;
//                 Ok(PorchItem::default(item_name, typ, value))
//             }
//         }
//     }
//
//     pub fn has_next(&self) -> bool {
//         self.index < Self::MAX_LEN
//     }
// }
//
// pub struct PouchItemTypeLookup {
//     profiles: HashMap<String, String>,
//     tags: HashMap<String, Vec<String>>,
// }
// impl PouchItemTypeLookup {
//     pub fn new() -> Self {
//         // TODO: --cleanup: unwraps
//         let file = std::fs::File::open("res/item_data.json").unwrap();
//         let reader = std::io::BufReader::new(file);
//         let json: HashMap<String, serde_json::Value> = serde_json::from_reader(reader).unwrap();
//         let mut profiles = HashMap::new();
//         for (key, value) in json {
//             if let Some(profile) = value.get("profile").and_then(|c| c.as_str()) {
//                 profiles.insert(key, profile.to_string());
//             }
//         }
//
//         let file = std::fs::File::open("res/actor_tags.json").unwrap();
//         let reader = std::io::BufReader::new(file);
//         let tags: HashMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();
//
//         Self { profiles, tags }
//     }
//
//     pub fn get_type(&self, name: &String) -> PouchItemType {
//         let profile = self.profiles.get(name);
//         let tags = self.tags.get(name);
//         if let (Some(profile), Some(tags)) = (profile, tags) {
//             if tags.contains(&String::from("Arrow")) {
//                 return PouchItemType::Arrow;
//             }
//
//             if profile == "WeaponSmallSword"
//                 || profile == "WeaponLongSword"
//                 || profile == "WeaponSpear"
//             {
//                 return PouchItemType::Sword;
//             }
//             if profile == "WeaponBow" {
//                 return PouchItemType::Bow;
//             }
//             if profile == "WeaponShield" {
//                 return PouchItemType::Shield;
//             }
//             if profile == "ArmorHead" {
//                 return PouchItemType::ArmorHead;
//             }
//             if profile == "ArmorUpper" {
//                 return PouchItemType::ArmorUpper;
//             }
//             if profile == "ArmorLower" {
//                 return PouchItemType::ArmorLower;
//             }
//             if profile == "HorseReins" {
//                 return PouchItemType::KeyItem;
//             }
//
//             if tags.contains(&String::from("CookResult"))
//                 || tags.contains(&String::from("RoastItem"))
//             {
//                 return PouchItemType::Food;
//             }
//
//             if tags.contains(&String::from("Important")) {
//                 return PouchItemType::KeyItem;
//             }
//
//             PouchItemType::Material
//         } else {
//             PouchItemType::Invalid
//         }
//     }
// }
