use blueflame::game::{PouchItemType, WeaponModifierInfo};
use skybook_parser::cir;

/// Get a WeaponModifierInfo struct from item meta
pub fn modifier_from_meta(meta: Option<&cir::ItemMeta>) -> Option<WeaponModifierInfo> {
    let meta = meta?;
    let flags = meta.sell_price? as u32;
    let value = meta.life_recover.unwrap_or(0);
    Some(WeaponModifierInfo { flags, value })
}

/// Check if the actor names matches one that uses animated icon.
///
/// Only one animated icon can be shown in the pouch screen for that item
pub fn is_animated_icon_actor(actor: &str) -> bool {
    matches!(
        actor,
        "Obj_DungeonClearSeal"
            | "Obj_WarpDLC"
            | "Obj_HeroSoul_Gerudo"
            | "Obj_HeroSoul_Goron"
            | "Obj_HeroSoul_Rito"
            | "Obj_HeroSoul_Zora"
            | "Obj_DLC_HeroSoul_Gerudo"
            | "Obj_DLC_HeroSoul_Goron"
            | "Obj_DLC_HeroSoul_Rito"
            | "Obj_DLC_HeroSoul_Zora"
            | "Obj_DLC_HeroSeal_Gerudo"
            | "Obj_DLC_HeroSeal_Goron"
            | "Obj_DLC_HeroSeal_Rito"
            | "Obj_DLC_HeroSeal_Zora"
    )
}

/// Convert PouchItemType value to a parser Category
pub fn item_type_to_category(item_type: i32) -> Option<cir::Category> {
    match PouchItemType::from_value(item_type) {
        Some(PouchItemType::Sword) => Some(cir::Category::Weapon),
        Some(PouchItemType::Bow) => Some(cir::Category::Bow),
        Some(PouchItemType::Arrow) => None,
        Some(PouchItemType::Shield) => Some(cir::Category::Shield),
        Some(PouchItemType::ArmorHead) => Some(cir::Category::ArmorHead),
        Some(PouchItemType::ArmorUpper) => Some(cir::Category::ArmorUpper),
        Some(PouchItemType::ArmorLower) => Some(cir::Category::ArmorLower),
        Some(PouchItemType::Material) => Some(cir::Category::Material),
        Some(PouchItemType::Food) => Some(cir::Category::Food),
        Some(PouchItemType::KeyItem) => Some(cir::Category::KeyItem),
        _ => None,
    }
}
