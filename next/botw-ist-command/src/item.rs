pub struct ItemStack {
    item_type: u32,
    item_use: u32,
    value: i32,
    equipped: bool,
    in_inventory: bool,
    name: String,
    data: ItemCookData,
    ingredients: [String; 5]
}

#[repr(C)]
pub struct ItemCookData {
    health_recover: i32,
    effect_duration: i32,
    sell_price: i32,
    cook_effect: f32,
    effect_level: f32
}

#[repr(C)]
pub struct ItemWeaponData {
}

pub struct ItemMeta {
    pub life: f32,
    pub equip: bool,
    pub price: u32,
    pub hp: u32,
    pub modifier: u32, // enum
    pub upgrade: u32
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid meta key: {0}")]
    InvalidMetaKey(String),
    #[error("Invalid meta value for key `{0}`: {1}")]
    InvalidMetaValue(String, String),
}
