use crate::game::{FixedSafeString40, ListNode, OffsetList};
use crate::memory::{self, MemObject, Memory, Ptr, offsetof};

#[derive(MemObject, Clone)]
#[size(0x44808)]
pub struct PauseMenuDataMgr {
    #[offset(0x0)]
    vtable: u64,
    #[offset(0x68)]
    pub mList1: OffsetList,
    #[offset(0x80)]
    pub mList2: OffsetList,
    #[offset(0x98)]
    pub mItemBuffer: [PouchItem; 420],
    // #[offset(0x441f8)]
    // mListHeads: [Ptr![Ptr![PouchItem]]; 7],
    #[offset(0x44230)]
    pub mTabs: [Ptr![PouchItem]; 50],
    // PouchItemType
    #[offset(0x443c0)]
    pub mTabsType: [i32; 50],
    #[offset(0x44488)]
    pub mLastAddedItem: Ptr![PouchItem],
    // #[offset(0x44490)]
    // mLastAddedItemTab: i32,
    // #[offset(0x44494)]
    // mLastAddedItemSlot: i32,
    #[offset(0x44498)]
    pub mNumTabs: i32,
    #[offset(0x444a0)]
    pub mGrabbedItems: [GrabbedItemInfo; 5],
    // #[offset(0x44518)]
    // mRitoSoulItem: Ptr![PouchItem],
    // #[offset(0x44520)]
    // mGoronSoulItem: Ptr![PouchItem],
    // #[offset(0x44528)]
    // mZoraSoulItem: Ptr![PouchItem],
    // #[offset(0x44530)]
    // mGerudoSoulItem: Ptr![PouchItem],
    // #[offset(0x44540)]
    // mNewlyAddedItem: PouchItem,
    // #[offset(0x447d8)]
    // mIsPouchForQuest: bool,
    #[offset(0x447e0)]
    pub mEquippedWeapons: [Ptr![PouchItem]; 4],
    // PouchCategory
    // #[offset(0x44800)]
    // pub mCategoryToSort: i32,
}

impl Ptr![PauseMenuDataMgr] {
    /// Get the index of the item in the item buffer, if the pointer
    /// is a valid internal pointer to an item in the buffer
    pub fn get_item_buffer_idx(self, item: Ptr![PouchItem]) -> Option<i32> {
        if item.to_raw() <= self.to_raw() {
            return None;
        }
        let ptr_diff = item.to_raw() - self.to_raw();
        if ptr_diff >= PauseMenuDataMgr::SIZE as u64 {
            return None;
        }
        let ptr_diff = ptr_diff as u32;
        if ptr_diff < offsetof!(self, mItemBuffer) {
            return None;
        }
        let ptr_diff = ptr_diff - offsetof!(self, mItemBuffer);
        // not aligned
        if ptr_diff % PouchItem::SIZE != 0 {
            return None;
        }

        Some((ptr_diff / PouchItem::SIZE) as i32)
    }

    pub fn equipped_weapons(self) -> Ptr![Ptr![PouchItem][4]] {
        Ptr!(&self->mEquippedWeapons).reinterpret_array()
    }

    pub fn grabbed_items(self) -> Ptr![GrabbedItemInfo[5]] {
        Ptr!(&self->mGrabbedItems).reinterpret_array()
    }

    pub fn item_buffer(self) -> Ptr![PouchItem[5]] {
        Ptr!(&self->mItemBuffer).reinterpret_array()
    }
}

#[derive(MemObject, Default, Copy, Clone)]
#[size(0x10)]
pub struct GrabbedItemInfo {
    #[offset(0x0)]
    pub mItem: Ptr![PouchItem],
    #[offset(0x8)]
    pub mIsActorSpawned: bool,
    // #[offset(0x9)]
    // _9: bool,
}

#[derive(MemObject, Default, Clone)]
#[size(0x298)]
pub struct PouchItem {
    #[offset(0x8)]
    pub mListNode: ListNode,
    #[offset(0x18)]
    pub mType: i32,
    #[offset(0x1c)]
    pub mItemUse: i32,
    #[offset(0x20)]
    pub mValue: i32,
    #[offset(0x24)]
    pub mEquipped: bool,
    #[offset(0x25)]
    pub mInInventory: bool,
    #[offset(0x28)]
    pub mName: FixedSafeString40,
    #[offset(0x80)]
    pub mHealthRecover: i32, // also modifier value
    #[offset(0x84)]
    pub mEffectDuration: i32,
    #[offset(0x88)]
    pub mSellPrice: i32, // also modifier flags
    #[offset(0x8c)]
    pub mEffectId: f32,
    #[offset(0x90)]
    pub mEffectLevel: f32,
    #[offset(0x98)]
    pub mIngredients: PtrArrayImpl_FixedSafeString40,
}

#[allow(non_camel_case_types)]
#[derive(MemObject, Default, Clone, Copy)]
#[size(0x10)]
pub struct PtrArrayImpl_FixedSafeString40 {
    #[offset(0x0)]
    mPtrNum: i32,
    #[offset(0x4)]
    mPtrNumMax: i32,
    #[offset(0x8)]
    mPtrs: Ptr![Ptr![FixedSafeString40][5]],
    // there is a buffer after this, but we don't need it
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, MemObject)]
#[size(0x8)]
pub struct WeaponModifierInfo {
    #[offset(0x0)]
    pub flags: u32,
    #[offset(0x4)]
    pub value: i32,
}

#[derive(MemObject, Clone)]
#[size(0x228)]
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
    vitality_boost: f32, // i.e effect level
    #[offset(0x224)]
    is_crit: bool,
}

impl Ptr![CookItem] {
    pub fn construct(self, m: &mut Memory) -> Result<(), memory::Error> {
        Ptr!(&self->actor_name).construct(m)?;
        for i in 0..5 {
            self.ith_ingredient(i).construct(m)?;
        }
        Ok(())
    }

    pub fn ith_ingredient(self, i: u64) -> Ptr![FixedSafeString40] {
        // TODO --fix-array-macro
        let ingredients = Ptr!(&self->ingredients)
            .reinterpret_array::<FixedSafeString40, { FixedSafeString40::SIZE }, 5>();
        ingredients.ith(i)
    }
}

/// Enum `uking::ui::PouchItemType`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PouchItemType {
    Sword = 0,
    Bow = 1,
    Arrow = 2,
    Shield = 3,
    ArmorHead = 4,
    ArmorUpper = 5,
    ArmorLower = 6,
    Material = 7,
    Food = 8,
    KeyItem = 9,
    #[default]
    Invalid = -1,
}

impl PouchItemType {
    /// Get a string description of the value. If valid, returns the name
    /// of the type, otherwise returns `Unknown(value in hex)`
    pub fn describe(value: i32) -> String {
        match Self::from_value(value) {
            Some(x) => format!("{x:?}"),
            None => format!("Unknown(0x{:08x})", value as u32),
        }
    }

    /// Convert a raw value into the PouchItemType enum
    pub fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::Sword,
            1 => Self::Bow,
            2 => Self::Arrow,
            3 => Self::Shield,
            4 => Self::ArmorHead,
            5 => Self::ArmorUpper,
            6 => Self::ArmorLower,
            7 => Self::Material,
            8 => Self::Food,
            9 => Self::KeyItem,
            -1 => Self::Invalid,
            _ => return None,
        })
    }
}

/// Enum `uking::ui::ItemUse`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PouchItemUse {
    WeaponSmallSword = 0,
    WeaponLargeSword = 1,
    WeaponSpear = 2,
    WeaponBow = 3,
    WeaponShield = 4,
    ArmorHead = 5,
    ArmorUpper = 6,
    ArmorLower = 7,
    Item = 8,
    ImportantItem = 9,
    CureItem = 10,
    #[default]
    Invalid = -1,
}

impl PouchItemUse {
    /// Get a string description of the value. If valid, returns the name
    /// of the type, otherwise returns `Unknown(value in hex)`
    pub fn describe(value: i32) -> String {
        match Self::from_value(value) {
            Some(x) => format!("{x:?}"),
            None => format!("Unknown(0x{:08x})", value as u32),
        }
    }

    /// Convert a raw value into the PouchItemUse enum
    pub fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::WeaponSmallSword,
            1 => Self::WeaponLargeSword,
            2 => Self::WeaponSpear,
            3 => Self::WeaponBow,
            4 => Self::WeaponShield,
            5 => Self::ArmorHead,
            6 => Self::ArmorUpper,
            7 => Self::ArmorLower,
            8 => Self::Item,
            9 => Self::ImportantItem,
            10 => Self::CureItem,
            -1 => Self::Invalid,
            _ => return None,
        })
    }
}

/// Enum `uking::CookEffectId`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CookEffectId {
    #[default]
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

impl CookEffectId {
    /// Get a string description of the value. If valid, returns the name
    /// of the type, otherwise returns `Unknown(value in hex)`
    pub fn describe(value: f32) -> String {
        match Self::from_value(value) {
            Some(x) => format!("{x:?}"),
            None => format!("Unknown(0x{:08x})", value as u32),
        }
    }

    /// Convert a raw value into the CookEffectId enum
    pub fn from_value(value: f32) -> Option<Self> {
        Some(match value {
            -1.0 => Self::None,
            1.0 => Self::LifeRecover,
            2.0 => Self::LifeMaxUp,
            4.0 => Self::ResistHot,
            5.0 => Self::ResistCold,
            6.0 => Self::ResistElectric,
            10.0 => Self::AttackUp,
            11.0 => Self::DefenseUp,
            12.0 => Self::Quietness,
            13.0 => Self::MovingSpeed,
            14.0 => Self::GutsRecover,
            15.0 => Self::ExGutsMaxUp,
            16.0 => Self::Fireproof,
            _ => return None,
        })
    }
}
