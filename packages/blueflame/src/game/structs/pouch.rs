use crate::game::{OffsetList, ListNode, FixedSafeString40};
use crate::memory::{self, MemObject, Memory, Ptr};

#[derive(MemObject, Clone)]
#[size(0x44808)]
pub struct PauseMenuDataMgr {
    #[offset(0x68)]
    pub mList1: OffsetList,
    #[offset(0x80)]
    pub mList2: OffsetList,
    #[offset(0x98)]
    pub mItemBuffer: [PouchItem; 420],
    #[offset(0x441f8)]
    mListHeads: [Ptr![Ptr![PouchItem]]; 7],
    #[offset(0x44230)]
    pub mTabs: [Ptr![PouchItem]; 50],
    // PouchItemType
    #[offset(0x443c0)]
    pub mTabsType: [i32; 50],
    // #[offset(0x44488)]
    // pub mLastAddedItem: Ptr![PouchItem],
    // #[offset(0x44490)]
    // mLastAddedItemTab: i32,
    // #[offset(0x44494)]
    // mLastAddedItemSlot: i32,
    #[offset(0x44498)]
    pub mNumTabs: i32,
    // #[offset(0x444a0)]
    // mGrabbedItems: [GrabbedItemInfo; 5],
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
    // #[offset(0x447e0)]
    // mEquippedWeapons: [Ptr![PouchItem]; 4],
    // PouchCategory
    // #[offset(0x44800)]
    // pub mCategoryToSort: i32,
}

#[derive(MemObject, Default, Clone)]
#[size(0x298)]
pub struct PouchItem {
    #[offset(0x8)]
    mListNode: ListNode,
    #[offset(0x18)]
    mType: i32,
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
    mIngredients: PtrArrayImpl_FixedSafeString40,
}

#[allow(non_camel_case_types)]
#[derive(MemObject, Default, Clone, Copy)]
#[size(0x10)]
struct PtrArrayImpl_FixedSafeString40 {
    #[offset(0x0)]
    mPtrNum: i32,
    #[offset(0x4)]
    mPtrNumMax: i32,
    #[offset(0x8)]
    mPtrs: Ptr![Ptr![FixedSafeString40][5]],
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
