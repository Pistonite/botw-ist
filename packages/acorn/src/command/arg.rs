pub struct StartGameArgs {}


pub struct GetItemArgs {
    /// Name of the item actor to get
    pub actor: String,

    /// Value to use for the item if given.
    ///
    /// If not given, equipment should receive their default durability
    /// (generalLife). Other items should use 1 as the value
    pub value: Option<i32>,

    pub modifier: Option<WeaponModifierInfo>,

    pub cook_item: Option<CookItemData>,
}

/// uking::CookItem in cookManager.h
pub struct CookItemData {
    pub ingredients: Vec<String>,
    pub life_recover: f32,
    pub effect_time: i32,
    pub sell_price: i32,
    pub effect_id: CookEffect,
    pub effect_level: f32,
    pub is_crit: bool
}

/// uking::CookEffectId in cookManager.h
#[repr(i32)]
pub enum CookEffect {
    None = -1,
    LifeRecover = 1,
    LifeMaxUp = 2,
    ResistHot = 4,
    ResistCold = 5,
    ResistElectric = 6,
    AttackUp = 10,
    DefenseUp = 11,
    Quietness = 12,
    // note the name we use internally for skybook is different
    // for decomp, it's MovingSpeed
    AllSpeed = 13,
    GutsRecover = 14,
    ExGutsMaxUp = 15,
    Fireproof = 16,
}

/// uking::act::WeaponModifierInfo in actWeapon.h
pub struct WeaponModifierInfo {
    pub flags: u32,
    pub value: i32
}

#[repr(u32)]
pub enum WeaponModifier {
    None = 0,
    AddPower = 0x1,
    AddLife = 0x2,
    Critical = 0x4,
    LongThrow = 0x8,
    SpreadFire = 0x10,
    Zoom = 0x20,
    RapidFire = 0x40,
    SurfMaster = 0x80,
    AddGuard = 0x100,
    Yellow = 0x80000000,
}

