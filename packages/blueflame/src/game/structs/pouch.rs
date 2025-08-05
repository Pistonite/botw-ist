use crate::game::{FixedSafeString40, ListNode, OffsetList, gdt};
use crate::memory::{self, MemObject, Memory, Ptr, mem, offsetof};

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
    #[offset(0x441f8)]
    pub mListHeads: [Ptr![Ptr![PouchItem]]; 7],
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
    #[offset(0x447d8)]
    pub mIsPouchForQuest: bool,
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
        if !ptr_diff.is_multiple_of(PouchItem::SIZE) {
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

    pub fn is_item_being_grabbed(
        self,
        item: Ptr![PouchItem],
        memory: &Memory,
    ) -> Result<bool, memory::Error> {
        let grabbed_items = self.grabbed_items();
        for i in 0..5 {
            let grabbed_item = grabbed_items.ith(i);
            mem! { memory:
                let grabbed_item = *(&grabbed_item->mItem);
            }
            if grabbed_item == item {
                return Ok(true);
            }
            // we know, probably, the array doesn't have holes
            if grabbed_item.is_nullptr() {
                return Ok(false);
            }
        }
        Ok(false)
    }

    pub fn item_buffer(self) -> Ptr![PouchItem[420]] {
        Ptr!(&self->mItemBuffer).reinterpret_array()
    }

    pub fn tabs(self) -> Ptr![Ptr![PouchItem][50]] {
        Ptr!(&self->mTabs).reinterpret_array()
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
    #[offset(0x0)]
    pub vtable: u64,
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
    pub mIngredients: FixedObjArray5_FixedSafeString40,
}

impl PouchItem {
    const VTABLE_OFFSET_150: u32 = 0x02476C90;
}

impl Ptr![PouchItem] {
    /// PouchItem constructor
    pub fn construct(self, memory: &mut Memory) -> Result<(), memory::Error> {
        // TODO --160
        let vptr = memory.main_start() + PouchItem::VTABLE_OFFSET_150 as u64;

        // we can take a shortcut to construct the ingr array,
        // rather than construct and emplace 5 elements
        let ingr_ptr_buffer_ptr = Ptr!(&self->mIngredients.mPtrBuffer).to_raw();
        let ingr_ptr_buffer: [Ptr![FixedSafeString40]; 5] = [
            Ptr!(&self->mIngredients.mStringOrNode_0),
            Ptr!(&self->mIngredients.mStringOrNode_1),
            Ptr!(&self->mIngredients.mStringOrNode_2),
            Ptr!(&self->mIngredients.mStringOrNode_3),
            Ptr!(&self->mIngredients.mStringOrNode_4),
        ];
        let ingr_ptr_first = Ptr!(&self->mIngredients.mStringOrNode_0).to_raw();
        mem! { memory:
            *(&self->vtable) = vptr;
            *(&self->mListNode) = ListNode::default();
            *(&self->mType) = -1;
            *(&self->mItemUse) = -1;
            *(&self->mValue) = 0;
            *(&self->mEquipped) = false;
            *(&self->mInInventory) = true;
            *(&self->mHealthRecover) = 0;
            *(&self->mEffectDuration) = 0;
            *(&self->mSellPrice) = 0;
            *(&self->mEffectId) = -1f32;
            *(&self->mEffectLevel) = 0f32;
            *(&self->mIngredients.mPtrNum) = 5;
            *(&self->mIngredients.mPtrNumMax) = 5;
            *(&self->mIngredients.mPtrs) = ingr_ptr_buffer_ptr.into();
            *(&self->mIngredients.mNextFree) = 0;
            *(&self->mIngredients.mFirst) = ingr_ptr_first;
            *(&self->mIngredients.mPtrBuffer) = ingr_ptr_buffer;
        }
        Ptr!(&self->mName).construct(memory)?;
        Ptr!(&self->mIngredients.mStringOrNode_0).construct(memory)?;
        Ptr!(&self->mIngredients.mStringOrNode_1).construct(memory)?;
        Ptr!(&self->mIngredients.mStringOrNode_2).construct(memory)?;
        Ptr!(&self->mIngredients.mStringOrNode_3).construct(memory)?;
        Ptr!(&self->mIngredients.mStringOrNode_4).construct(memory)?;
        Ok(())
    }
    /// WeaponModifier constructor from PouchItem, must be non-null
    pub fn to_modifier_info(self, memory: &Memory) -> Result<WeaponModifierInfo, memory::Error> {
        mem! { memory:
            let item_type = *(&self->mType);
        }
        if item_type <= 3 {
            mem! { memory:
                let value = *(&self->mHealthRecover);
                let flags = *(&self->mSellPrice);
            }
            return Ok(WeaponModifierInfo {
                flags: flags as u32,
                value,
            });
        }
        Ok(WeaponModifierInfo::default())
    }

    pub fn ith_ingredient(
        self,
        i: u64,
        memory: &Memory,
    ) -> Result<Ptr![FixedSafeString40], memory::Error> {
        mem! { memory:
            let ingredients = *(&self->mIngredients.mPtrs);
            let ingredients_i = *(ingredients.ith(i));
        }
        Ok(ingredients_i)
    }
}

#[allow(non_camel_case_types)]
#[derive(MemObject, Default, Clone)]
#[size(0x200)]
pub struct FixedObjArray5_FixedSafeString40 {
    #[offset(0x0)]
    mPtrNum: i32, // PouchItem constructor sets this to 5
    #[offset(0x4)]
    mPtrNumMax: i32, // 5
    #[offset(0x8)]
    mPtrs: Ptr![Ptr![FixedSafeString40][5]], // points to mPtrBuffer
    #[offset(0x10)]
    mNextFree: u64,
    #[offset(0x18)]
    mFirst: u64,
    #[offset(0x20)]
    mStringOrNode_0: FixedSafeString40, // union of Node* and FixedSafeString40
    #[offset(0x78)]
    mStringOrNode_1: FixedSafeString40,
    #[offset(0xD0)]
    mStringOrNode_2: FixedSafeString40,
    #[offset(0x128)]
    mStringOrNode_3: FixedSafeString40,
    #[offset(0x180)]
    mStringOrNode_4: FixedSafeString40,
    #[offset(0x1d8)]
    mPtrBuffer: [Ptr![FixedSafeString40]; 5],
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, MemObject)]
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

    pub fn to_category(self) -> PouchCategory {
        match self {
            PouchItemType::Sword => PouchCategory::Sword,
            PouchItemType::Bow | PouchItemType::Arrow => PouchCategory::Bow,
            PouchItemType::Shield => PouchCategory::Shield,
            PouchItemType::ArmorHead | PouchItemType::ArmorUpper | PouchItemType::ArmorLower => {
                PouchCategory::Armor
            }
            PouchItemType::Material => PouchCategory::Material,
            PouchItemType::Food => PouchCategory::Food,
            PouchItemType::KeyItem => PouchCategory::KeyItem,
            PouchItemType::Invalid => PouchCategory::Invalid,
        }
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

impl gdt::FlagIndex for PouchCategory {
    fn to_index(self) -> Option<usize> {
        match self {
            Self::Invalid => None,
            _ => Some(self as usize),
        }
    }
}
