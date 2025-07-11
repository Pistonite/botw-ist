use blueflame::game::{self, PouchCategory, PouchItemType, gdt};
use blueflame::processor::Process;

use crate::iv;

use super::{Error, coherence_error, try_mem};

/// Read the items stored as GDT flags from the process memory
pub fn extract_gdt_view(proc: &Process) -> Result<iv::Gdt, Error> {
    let gdt_ptr = try_mem!(
        gdt::trigger_param_ptr(proc.memory()),
        e,
        "failed to load gdt pointer: {e}"
    );
    // we can't use the proxy! macro since the return error is ViewError
    let guard = proc.proxies().trigger_param.read(proc.memory());
    let gdt = try_mem!(
        guard.get(gdt_ptr),
        e,
        "failed to load gdt trigger param: {e}"
    );

    extract_from_trigger_param(gdt)
}

/// Extract items stored in a TriggerParam instance (could be in memory or in save)
///
/// The implementation is similar to `uking::ui::PauseMenuDataMgr::doLoadFromGameData`.
/// However, the implementation is "correct", i.e. it will associate `i`th Weapon/Sword/Shield/Food
/// with the actual `i`th item of that type.
fn extract_from_trigger_param(gdt: &gdt::TriggerParam) -> Result<iv::Gdt, Error> {
    macro_rules! get_flag {
        (($($fd:tt)*) $name:literal) => {{
            let Some(x) = gdt.by_name::<gdt::fd!($($fd)*)>($name) else {
                coherence_error!("cannot read {} flag", $name);
            };
            x
        }}
    }
    macro_rules! get_flag_value {
        (($($fd:tt)*) $name:literal) => {{
            *{get_flag!(($($fd)*) $name)}.get()
        }}
    }
    const ITEM_MAX: usize = 420;
    let porchitem = get_flag!((str64[]) "PorchItem");
    let porchitem_equipflag = get_flag!((bool[]) "PorchItem_EquipFlag");
    let porchitem_value1 = get_flag!((s32[]) "PorchItem_Value1");
    let porchsword_flagsp = get_flag!((s32[]) "PorchSword_FlagSp");
    let porchsword_valuesp = get_flag!((s32[]) "PorchSword_ValueSp");
    let porchbow_flagsp = get_flag!((s32[]) "PorchBow_FlagSp");
    let porchbow_valuesp = get_flag!((s32[]) "PorchBow_ValueSp");
    let porchshield_flagsp = get_flag!((s32[]) "PorchShield_FlagSp");
    let porchshield_valuesp = get_flag!((s32[]) "PorchShield_ValueSp");
    let stamina_recover = get_flag!((vec2f[]) "StaminaRecover");
    let cook_effect0 = get_flag!((vec2f[]) "CookEffect0");
    let cook_effect1 = get_flag!((vec2f[]) "CookEffect1");
    let cook_material0 = get_flag!((str64[]) "CookMaterialName0");
    let cook_material1 = get_flag!((str64[]) "CookMaterialName1");
    let cook_material2 = get_flag!((str64[]) "CookMaterialName2");
    let cook_material3 = get_flag!((str64[]) "CookMaterialName3");
    let cook_material4 = get_flag!((str64[]) "CookMaterialName4");

    let mut items = vec![];
    for i in 0..ITEM_MAX {
        let Some(name) = porchitem.get_at(i) else {
            coherence_error!("cannot read PorchItem[{i}]");
        };
        if name.is_empty() {
            break;
        }
        let Some(is_equipped) = porchitem_equipflag.get_at(i) else {
            coherence_error!("cannot read PorchItem_EquipFlag[{i}]");
        };
        let Some(value) = porchitem_value1.get_at(i) else {
            coherence_error!("cannot read PorchItem_Value1[{i}]");
        };
        let item_common = iv::CommonItem {
            actor_name: name.to_string(),
            value: *value,
            is_equipped: *is_equipped,
        };
        items.push(iv::GdtItem {
            common: item_common,
            idx: i as u32,
            data: iv::GdtItemData::None,
        });
    }

    let mut sword_idx = 0u32;
    let mut bow_idx = 0u32;
    let mut shield_idx = 0u32;
    let mut food_idx = 0u32;

    for item in items.iter_mut() {
        const SWORD: i32 = PouchItemType::Sword as i32;
        const BOW: i32 = PouchItemType::Bow as i32;
        const SHIELD: i32 = PouchItemType::Shield as i32;
        const FOOD: i32 = PouchItemType::Food as i32;
        let item_type = game::get_pouch_item_type(&item.common.actor_name);
        match item_type {
            // s32 defaults to 0 when OOB
            SWORD => {
                let value = porchsword_valuesp
                    .get_at(sword_idx)
                    .copied()
                    .unwrap_or_default();
                let flag = porchsword_flagsp
                    .get_at(sword_idx)
                    .copied()
                    .unwrap_or_default();
                item.data = iv::GdtItemData::Sword {
                    idx: sword_idx,
                    info: iv::WeaponModifier { flag, value },
                };
                sword_idx += 1;
            }
            BOW => {
                let value = porchbow_valuesp
                    .get_at(bow_idx)
                    .copied()
                    .unwrap_or_default();
                let flag = porchbow_flagsp.get_at(bow_idx).copied().unwrap_or_default();
                item.data = iv::GdtItemData::Bow {
                    idx: bow_idx,
                    info: iv::WeaponModifier { flag, value },
                };
                bow_idx += 1;
            }
            SHIELD => {
                let value = porchshield_valuesp
                    .get_at(shield_idx)
                    .copied()
                    .unwrap_or_default();
                let flag = porchshield_flagsp
                    .get_at(shield_idx)
                    .copied()
                    .unwrap_or_default();
                item.data = iv::GdtItemData::Bow {
                    idx: shield_idx,
                    info: iv::WeaponModifier { flag, value },
                };
                shield_idx += 1;
            }
            FOOD => {
                // if OOB, the value will fall back to the previous value on stack
                let mut x = 0f32;
                let mut y = 0f32;
                let (effect_value, effect_duration) =
                    stamina_recover.get_at(food_idx).copied().unwrap_or((x, y));
                x = effect_value;
                y = effect_duration;
                let (effect_id, effect_level) =
                    cook_effect0.get_at(food_idx).copied().unwrap_or((x, y));
                x = effect_id;
                y = effect_level;
                let (sell_price, unused) = cook_effect1.get_at(food_idx).copied().unwrap_or((x, y));

                // the first material technically falls back to the item_name on stack,
                // but this is just the view, so it's probably fine we don't simulate that
                // (there's no known exploit for it either)
                let mut name_ref = "";
                let ingr0 = cook_material0
                    .get_at(food_idx)
                    .map(|x| x.as_str())
                    .unwrap_or(name_ref)
                    .to_string();
                name_ref = &ingr0;
                let ingr1 = cook_material1
                    .get_at(food_idx)
                    .map(|x| x.as_str())
                    .unwrap_or(name_ref)
                    .to_string();
                name_ref = &ingr1;
                let ingr2 = cook_material2
                    .get_at(food_idx)
                    .map(|x| x.as_str())
                    .unwrap_or(name_ref)
                    .to_string();
                name_ref = &ingr2;
                let ingr3 = cook_material3
                    .get_at(food_idx)
                    .map(|x| x.as_str())
                    .unwrap_or(name_ref)
                    .to_string();
                name_ref = &ingr3;
                let ingr4 = cook_material4
                    .get_at(food_idx)
                    .map(|x| x.as_str())
                    .unwrap_or(name_ref)
                    .to_string();

                item.data = iv::GdtItemData::Food {
                    idx: food_idx,
                    info: iv::ItemData {
                        effect_value: effect_value as i32,       // f32 -> i32 cast
                        effect_duration: effect_duration as i32, // f32 -> i32 cast
                        sell_price: sell_price as i32,           // f32 -> i32 cast
                        effect_id,
                        effect_level,
                    },
                    unused_effect_1y: unused,
                    ingredients: [ingr0, ingr1, ingr2, ingr3, ingr4],
                };

                food_idx += 1;
            }
            _ => {
                // no extra data needed for other types
            }
        }
    }

    // MISC other flags
    let master_sword = iv::GdtMasterSword {
        is_true_form: get_flag_value!((bool) "Open_MasterSword_FullPower"),
        add_power: get_flag_value!((s32) "MasterSword_Add_Power"),
        add_beam_power: get_flag_value!((s32) "MasterSword_Add_BeamPower"),
        recover_time: get_flag_value!((f32) "MasterSwordRecoverTime"),
    };
    let is_open_item_category = get_flag!((bool[]) "IsOpenItemCategory");
    let info = iv::GdtInvInfo {
        num_weapon_slots: get_flag_value!((s32) "WeaponPorchStockNum"),
        num_bow_slots: get_flag_value!((s32) "BowPorchStockNum"),
        num_shield_slots: get_flag_value!((s32) "ShieldPorchStockNum"),
        sword_tab_discovered: is_open_item_category
            .get_at(PouchCategory::Sword)
            .copied()
            .unwrap_or_default(),
        bow_tab_discovered: is_open_item_category
            .get_at(PouchCategory::Bow)
            .copied()
            .unwrap_or_default(),
        shield_tab_discovered: is_open_item_category
            .get_at(PouchCategory::Shield)
            .copied()
            .unwrap_or_default(),
        armor_tab_discovered: is_open_item_category
            .get_at(PouchCategory::Armor)
            .copied()
            .unwrap_or_default(),
        material_tab_discovered: is_open_item_category
            .get_at(PouchCategory::Material)
            .copied()
            .unwrap_or_default(),
        food_tab_discovered: is_open_item_category
            .get_at(PouchCategory::Food)
            .copied()
            .unwrap_or_default(),
        key_item_tab_discovered: is_open_item_category
            .get_at(PouchCategory::KeyItem)
            .copied()
            .unwrap_or_default(),
    };

    Ok(iv::Gdt {
        items,
        master_sword,
        info,
    })
}
