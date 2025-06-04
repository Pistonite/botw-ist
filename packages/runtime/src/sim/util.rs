use blueflame::game::WeaponModifierInfo;
use skybook_parser::cir;

/// Get a WeaponModifierInfo struct from item meta
pub fn modifier_from_meta(meta: Option<&cir::ItemMeta>) -> Option<WeaponModifierInfo> {
    let meta = meta?;
    let flags = meta.sell_price? as u32;
    let value = meta.life_recover.unwrap_or(0);
    Some(WeaponModifierInfo {
        flags,
        value,
    })
}

