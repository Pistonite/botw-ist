use blueflame::game::{self, PouchCategory, PouchItem, PouchItemType, WeaponModifierInfo};
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
            | "Obj_DLC_HeroSeal_Gerudo"
            | "Obj_DLC_HeroSeal_Goron"
            | "Obj_DLC_HeroSeal_Rito"
            | "Obj_DLC_HeroSeal_Zora"
    ) || is_hero_soul(actor)
}

/// Check if the actor has the HeroSoul tag
pub fn is_hero_soul(actor: &str) -> bool {
    matches!(
        actor,
        "Obj_HeroSoul_Gerudo"
            | "Obj_HeroSoul_Goron"
            | "Obj_HeroSoul_Rito"
            | "Obj_HeroSoul_Zora"
            | "Obj_DLC_HeroSoul_Gerudo"
            | "Obj_DLC_HeroSoul_Goron"
            | "Obj_DLC_HeroSoul_Rito"
            | "Obj_DLC_HeroSoul_Zora"
    )
}

/// Check if the item name spec is a Sword/Bow/Shield
pub fn name_spec_is_weapon(spec: &cir::ItemNameSpec) -> bool {
    let category = match spec {
        cir::ItemNameSpec::Actor(actor) => {
            let Some(category) = item_type_to_category(game::get_pouch_item_type(actor)) else {
                return false;
            };
            category
        }
        cir::ItemNameSpec::Category(category) => *category,
    };

    matches!(
        category,
        cir::Category::Weapon | cir::Category::Bow | cir::Category::Shield
    )
}

/// Check if the actor is a Sword/Bow/Shield
pub fn name_is_weapon(name: &str) -> bool {
    matches!(game::get_pouch_item_type(name), 0 | 1 | 3)
}

pub fn name_is_arrow(name: &str) -> bool {
    game::get_pouch_item_type(name) == PouchItemType::Arrow as i32
}

/// Check if the item can use the drop prompt
pub fn name_spec_can_drop(spec: &cir::ItemNameSpec) -> bool {
    let category = match spec {
        cir::ItemNameSpec::Actor(actor) => match actor.as_str() {
            "Weapon_Sword_070" | "Weapon_Sword_080" | "Weapon_Sword_081" | "Weapon_Sword_502"
            | "Weapon_Bow_071" => return false,
            _ => {
                let Some(category) = item_type_to_category(game::get_pouch_item_type(actor)) else {
                    return false;
                };
                category
            }
        },
        cir::ItemNameSpec::Category(category) => *category,
    };

    matches!(
        category,
        cir::Category::Weapon
            | cir::Category::Bow
            | cir::Category::Shield
            | cir::Category::Material
    )
}

/// Check if items matched by the name could possibly be stackable
pub fn name_spec_could_stack(spec: &cir::ItemNameSpec) -> bool {
    match spec {
        cir::ItemNameSpec::Actor(name) => game::can_stack(name),
        cir::ItemNameSpec::Category(category) => matches!(
            category,
            cir::Category::Armor
                | cir::Category::Material
                | cir::Category::Food
                | cir::Category::KeyItem
        ),
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
        }
    }
}

pub fn name_spec_to_item_type(spec: &cir::ItemNameSpec) -> i32 {
    match spec {
        cir::ItemNameSpec::Actor(actor) => game::get_pouch_item_type(actor),
        cir::ItemNameSpec::Category(category) => category_to_item_type(*category),
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
        }
        cir::ItemNameSpec::Category(category) => category.is_equipment(),
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
    if is_weapon && expected != 0 && (expected & (expected - 1)) == 0 {
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

/// Convert parser category to PouchItemType value
pub fn category_to_item_type(category: cir::Category) -> i32 {
    match category {
        cir::Category::Weapon => 0,
        cir::Category::Bow => 1,
        cir::Category::Shield => 3,
        cir::Category::Armor => 4,
        cir::Category::ArmorHead => 4,
        cir::Category::ArmorUpper => 5,
        cir::Category::ArmorLower => 6,
        cir::Category::Material => 7,
        cir::Category::Food => 8,
        cir::Category::KeyItem => 9,
    }
}

/// Convert parser category to PouchCategory calue
pub fn category_to_pouch_category(category: cir::Category) -> PouchCategory {
    match category {
        cir::Category::Weapon => PouchCategory::Sword,
        cir::Category::Bow => PouchCategory::Bow,
        cir::Category::Shield => PouchCategory::Shield,
        cir::Category::Armor => PouchCategory::Armor,
        cir::Category::ArmorHead => PouchCategory::Armor,
        cir::Category::ArmorUpper => PouchCategory::Armor,
        cir::Category::ArmorLower => PouchCategory::Armor,
        cir::Category::Material => PouchCategory::Material,
        cir::Category::Food => PouchCategory::Food,
        cir::Category::KeyItem => PouchCategory::KeyItem,
    }
}

/// Check if the current item ptr is the start of
/// the next tab (after tab_i), or the end of inventory
pub fn should_go_to_next_tab(
    curr_item_ptr: Ptr![PouchItem],
    mut tab_i: usize,
    num_tabs: usize,
    tab_heads: &[Ptr![PouchItem]; 50],
) -> bool {
    if curr_item_ptr.is_nullptr() {
        return true;
    }
    // we must skip empty tabs and check
    // heads of all tabs after
    while tab_i + 1 < num_tabs {
        let next_head = tab_heads[tab_i + 1];
        if next_head.is_nullptr() {
            tab_i += 1;
            continue;
        }
        return curr_item_ptr == next_head;
    }
    // if we are at last tab, or if all tabs after us
    // are empty, don't stop, item is in this tab still
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
            Self::CanStack => game::can_stack(actor),
            Self::CanStackOrFood => game::can_stack(actor) || actor.starts_with("Item_Cook"),
            Self::Value => true,
        }
    }
}
