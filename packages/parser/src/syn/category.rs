use teleparse::derive_syntax;

use super::token::{
    KwArmor, KwArmors, KwBow, KwBows, KwFood, KwFoods, KwKeyItem, KwKeyItems, KwMaterial,
    KwMaterials, KwShield, KwShields, KwWeapon, KwWeapons,
};

/// Category specifier
#[derive_syntax]
#[derive(Debug)]
pub enum Category {
    Weapon(CatWeapon),
    Bow(CatBow),
    Shield(CatShield),
    Armor(CatArmor),
    Material(CatMaterial),
    Food(CatFood),
    KeyItem(CatKeyItem),
}

/// The "weapon" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatWeapon {
    Singular(KwWeapon),
    Plural(KwWeapons),
}

/// The "bow" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatBow {
    Singular(KwBow),
    Plural(KwBows),
}

/// The "shield" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatShield {
    Singular(KwShield),
    Plural(KwShields),
}

/// The "armor" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatArmor {
    Singular(KwArmor),
    Plural(KwArmors),
}

/// The "material" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatMaterial {
    Singular(KwMaterial),
    Plural(KwMaterials),
}

/// The "food" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatFood {
    Singular(KwFood),
    Plural(KwFoods),
}

/// The "key item" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatKeyItem {
    Singular(KwKeyItem),
    Plural(KwKeyItems),
}
