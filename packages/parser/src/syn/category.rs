use teleparse::derive_syntax;

use crate::syn;

/// Category specifier
#[derive_syntax]
#[derive(Debug)]
pub enum Category {
    Weapon(CatWeapon),
    Bow(CatBow),
    Shield(CatShield),
    Armor(CatArmor),
    ArmorHead(CatArmorHead),
    ArmorUpper(CatArmorUpper),
    ArmorLower(CatArmorLower),
    Material(CatMaterial),
    Food(CatFood),
    KeyItem(CatKeyItem),
}

/// The "weapon" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatWeapon {
    Singular(syn::KwWeapon),
    Plural(syn::KwWeapons),
}

/// The "bow" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatBow {
    Singular(syn::KwBow),
    Plural(syn::KwBows),
}

/// The "shield" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatShield {
    Singular(syn::KwShield),
    Plural(syn::KwShields),
}

/// The "armor" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatArmor {
    Singular(syn::KwArmor),
    Plural(syn::KwArmors),
}

/// The "armor" tab, "ArmorHead" category
#[derive_syntax]
#[derive(Debug)]
pub enum CatArmorHead {
    ArmorHead(syn::KwArmorHead),
    HeadArmor(syn::KwHeadArmor),
    HeadArmors(syn::KwHeadArmors),
}

/// The "armor" tab, "ArmorUpper" category
#[derive_syntax]
#[derive(Debug)]
pub enum CatArmorUpper {
    ArmorBody(syn::KwArmorBody),
    BodyArmor(syn::KwBodyArmor),
    BodyArmors(syn::KwBodyArmors),
    ArmorChest(syn::KwArmorChest),
    ChestArmor(syn::KwChestArmor),
    ChestArmors(syn::KwChestArmors),
    ArmorUpper(syn::KwArmorUpper),
    UpperArmor(syn::KwUpperArmor),
    UpperArmors(syn::KwUpperArmors),
}

/// The "armor" tab, "ArmorLower" category
#[derive_syntax]
#[derive(Debug)]
pub enum CatArmorLower {
    ArmorLeg(syn::KwArmorLeg),
    ArmorLegs(syn::KwArmorLegs),
    LegArmor(syn::KwLegArmor),
    LegArmors(syn::KwLegArmors),
    ArmorLower(syn::KwArmorLower),
    LowerArmor(syn::KwLowerArmor),
    LowerArmors(syn::KwLowerArmors),
}

/// The "material" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatMaterial {
    Singular(syn::KwMaterial),
    Plural(syn::KwMaterials),
}

/// The "food" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatFood {
    Singular(syn::KwFood),
    Plural(syn::KwFoods),
}

/// The "key item" tab/category
#[derive_syntax]
#[derive(Debug)]
pub enum CatKeyItem {
    Singular(syn::KwKeyItem),
    Plural(syn::KwKeyItems),
}
