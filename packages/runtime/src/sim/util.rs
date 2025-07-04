use blueflame::game::{self, PouchItem, PouchItemType, WeaponModifierInfo};
use blueflame::memory::Ptr;
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

pub fn name_spec_is_weapon(spec: &cir::ItemNameSpec) -> bool {
    let category = match spec {
        cir::ItemNameSpec::Actor(actor) => {
            let Some(category) = item_type_to_category(game::get_pouch_item_type(actor)) else {
                return false;
            };
            category
        },
        cir::ItemNameSpec::Category(category) => *category,
    };

    matches!(category, cir::Category::Weapon | cir::Category::Bow
    | cir::Category::Shield)
}

/// Check if the item can use the drop prompt
pub fn name_spec_can_drop(spec: &cir::ItemNameSpec) -> bool {
    let category = match spec {
        cir::ItemNameSpec::Actor(actor) => {
            match actor.as_str() {
                "Weapon_Sword_070"
                |
                "Weapon_Sword_080"
                |
                "Weapon_Sword_081"
                |
                "Weapon_Sword_502" |
                "Weapon_Bow_071" => return false,
                _ => {
                    let Some(category) = item_type_to_category(game::get_pouch_item_type(actor)) else {
                        return false;
                    };
                    category
                }
            }
        },
        cir::ItemNameSpec::Category(category) => *category,
    };

    matches!(category, cir::Category::Weapon | cir::Category::Bow
    | cir::Category::Shield | cir::Category::Material)
}

/// Check if items matched by the name could possibly be stackable
pub fn name_spec_could_stack(spec: &cir::ItemNameSpec) -> bool {
    match spec {
        cir::ItemNameSpec::Actor(name) => game::can_stack(name),
        cir::ItemNameSpec::Category(category) => matches!(
        category,
            cir::Category::Armor | cir::Category::Material
            | cir::Category::Food | cir::Category::KeyItem
    )
    }
}

/// Check if the item name matches the item name spec
pub fn name_spec_matches(spec: &cir::ItemNameSpec, name: &str) -> bool {
    match spec {
        cir::ItemNameSpec::Actor(actor) => actor == name,
        cir::ItemNameSpec::Category(category) => {
            let Some(item_category) = item_type_to_category(game::get_pouch_item_type(name)) else {
                return false;
            };
            let category = *category;

            // if "Armor" is used for matching, matches all kinds of armor
            if category == cir::Category::Armor {
                return category == item_category.coerce_armor();
            }
            category == item_category
        },
    }
}

/// Check if the modifier meta matches
///
/// If the item not is Weapon/Bow/Shield category, expected and actual must match exactly,
/// otherwise:
/// - If `expected` only has one bit on, returns true if that bit is also on in `actual`,
/// - Otherwise, must match exactly
pub fn modifier_meta_matches(item: &cir::ItemNameSpec, expected: i32, actual: i32) -> bool {
    let is_weapon = match item {
        cir::ItemNameSpec::Actor(actor) => {
    matches!(game::get_pouch_item_type(actor), 0 | 1 | 3)
        },
        cir::ItemNameSpec::Category(category) => 
            category.is_equipment()
    };
    modifier_meta_matches_by_is_weapon(is_weapon, expected, actual)
}

/// See [`modifier_meta_matches`]
pub fn modifier_meta_matches_by_name(item: &str, expected: i32, actual: i32) -> bool {
    let is_weapon = matches!(game::get_pouch_item_type(item), 0 | 1 | 3);
    modifier_meta_matches_by_is_weapon(is_weapon, expected, actual)
}

/// See [`modifier_meta_matches`]
pub fn modifier_meta_matches_by_is_weapon(is_weapon: bool, expected: i32, actual: i32) -> bool {
    if is_weapon && expected != 0 && (expected & (expected-1)) == 0{
        return (expected & actual) != 0;
    }
    expected == actual
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

/// Check if the current item ptr is the start of
/// the next tab (after tab_i), or the end of inventory
pub fn should_go_to_next_tab(
    curr_item_ptr: Ptr![PouchItem],
    tab_i: usize,
    num_tabs: usize,
    tab_heads: &[Ptr![PouchItem]; 50],
) -> bool {
    if curr_item_ptr.is_nullptr() {
        return true;
    }
    if tab_i < num_tabs - 1 {
        let next_head = tab_heads[tab_i + 1];
        return curr_item_ptr == next_head;
    }
    false
}

/// Method to count how many items are available to operate on
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountingMethod {
    /// Each slot is 1
    Slot,
    /// If the item has the CanStack tag, use its value, otherwise is 1
    CanStack,
    /// If the item has the CanStack tag, or is a food, use its value, otherwise is 1
    CanStackOrFood,
    /// Use the value for each slot
    Value,
}

impl CountingMethod {
    /// Return true if this method should cound this actor using the stack value (return false if
    /// 1 should be used instead of value)
    pub fn should_use_value(self, actor: &str) -> bool {
        match self {
            Self::Slot => false,
            Self::CanStack => game::can_stack(&actor),
            Self::CanStackOrFood => game::can_stack(&actor) || actor.starts_with("Item_Cook"),
            Self::Value => true,
        }
    }
}

// pub struct PeTarget {
//     pub tab: usize,
//     pub slot: usize,
//     /// Behavior if the slot is empty
//     pub empty_behavior: PeBehavior,
//     /// Behavior if the slot is translucent
//     pub translucent_behavior: PeBehavior,
// }
//
//
// /// Behavior of PE when targeting empty or translucent slot
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum PeBehavior {
//     /// doesn't do anything
//     None,
//     /// Targets the first slot instead
//     First,
//     /// Targets the slot anyway
//     Force,
// }
