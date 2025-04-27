/** uking::ui::PouchItemType */
export const PouchItemType = {
    Sword: 0,
    Bow: 1,
    Arrow: 2,
    Shield: 3,
    ArmorHead: 4,
    ArmorUpper: 5,
    ArmorLower: 6,
    Material: 7,
    Food: 8,
    KeyItem: 9,
    Invalid: -1,
} as const;
export type PouchItemType = (typeof PouchItemType)[keyof typeof PouchItemType];
export type PouchItemTypeName = keyof typeof PouchItemType;
export const PouchItemTypeNames = [
    "Sword",
    "Bow",
    "Arrow",
    "Shield",
    "ArmorHead",
    "ArmorUpper",
    "ArmorLower",
    "Material",
    "Food",
    "KeyItem",
] as const satisfies PouchItemTypeName[];

/**
 * uking::ui::PouchCategory. Used as index for mListHeads
 */
export const PouchCategory = {
    Sword: 0,
    Bow: 1,
    Shield: 2,
    Armor: 3,
    Material: 4,
    Food: 5,
    KeyItem: 6,
    Invalid: -1,
} as const;
export type PouchCategory = (typeof PouchCategory)[keyof typeof PouchCategory];
export type PouchCategoryName = keyof typeof PouchCategory;
export const PouchCategoryNames = [
    "Sword",
    "Bow",
    "Shield",
    "Armor",
    "Material",
    "Food",
    "KeyItem",
] as const satisfies PouchCategoryName[];

/** uking::ui::ItemUse */
export const PouchItemUse = {
    WeaponSmallSword: 0,
    WeaponLargeSword: 1,
    WeaponSpear: 2,
    WeaponBow: 3,
    WeaponShield: 4,
    ArmorHead: 5,
    ArmorUpper: 6,
    ArmorLower: 7,
    Item: 8,
    ImportantItem: 9,
    CureItem: 10,
    Invalid: -1,
} as const;
export type PouchItemUse = (typeof PouchItemUse)[keyof typeof PouchItemUse];
export type PouchItemUseName = keyof typeof PouchItemUse;
export const PouchItemUseNames = [
    "WeaponSmallSword",
    "WeaponLargeSword",
    "WeaponSpear",
    "WeaponBow",
    "WeaponShield",
    "ArmorHead",
    "ArmorUpper",
    "ArmorLower",
    "Item",
    "ImportantItem",
    "CureItem",
] as const satisfies PouchItemUseName[];

/** uking::CookEffectId */
export const CookEffect = {
    None: -1,
    LifeRecover: 1,
    LifeMaxUp: 2,
    ResistHot: 4,
    ResistCold: 5,
    ResistElectric: 6,
    AttackUp: 10,
    DefenseUp: 11,
    Quietness: 12,
    // note the name we use internally for skybook is different
    // for decomp, it's MovingSpeed
    AllSpeed: 13,
    GutsRecover: 14,
    ExGutsMaxUp: 15,
    Fireproof: 16,
} as const;
export type CookEffect = (typeof CookEffect)[keyof typeof CookEffect];
export type CookEffectName = keyof typeof CookEffect;
export const CookEffectNames = [
    "", // 0
    "LifeRecover",
    "LifeMaxUp",
    "", // 3
    "ResistHot",
    "ResistCold",
    "ResistElectric",
    "", // 7
    "", // 8
    "", // 9
    "AttackUp",
    "DefenseUp",
    "Quietness",
    "AllSpeed",
    "GutsRecover",
    "ExGutsMaxUp",
    "Fireproof",
] as const satisfies (CookEffectName | "")[];

/**
 * Internal used special status enum (i.e. not part of Pouch or GDT data).
 * The numbers don't have significance outside of Skybook
 *
 * These correspond to modifier icons
 * These are used in generated data - DO NOT CHANGE
 */
export const SpecialStatus = {
    None: 0,
    AddGuard: 1,
    AddGuardPlus: 2,
    AddLife: 3,
    AddLifePlus: 4,
    AddPower: 5,
    AddPowerPlus: 6,
    AllSpeed: 7,
    AttackUp: 8,
    ClimbSpeedUp: 9,
    Critical: 10,
    DefenseUp: 11,
    ExGutsMaxUp: 12,
    Fireproof: 13, // "ResistBurn", not the fire-immunity effect
    GutsRecover: 14,
    LifeMaxUp: 15,
    LongThrow: 16,
    Quietness: 17,
    RapidFire: 18,
    ReduceAncientEnemyDamge: 19, // not a typo
    ResistCold: 20,
    ResistElectric: 21,
    ResistFreeze: 22,
    ResistHot: 23,
    ResistLightning: 24,
    SandMoveSpeedUp: 25,
    SnowMovingSpeed: 26,
    SpreadFire: 27,
    SurfMaster: 28,
    SwimSpeedUp: 29,
    Zoom: 30,
} as const;
export type SpecialStatus = (typeof SpecialStatus)[keyof typeof SpecialStatus];
export type SpecialStatusName = keyof typeof SpecialStatus;
export const SpecialStatusNames = [
    "",
    "AddGuard",
    "AddGuardPlus",
    "AddLife",
    "AddLifePlus",
    "AddPower",
    "AddPowerPlus",
    "AllSpeed",
    "AttackUp",
    "ClimbSpeedUp",
    "Critical",
    "DefenseUp",
    "ExGutsMaxUp",
    "Fireproof", // "ResistBurn", not the fire-immunity effect
    "GutsRecover",
    "LifeMaxUp",
    "LongThrow",
    "Quietness",
    "RapidFire",
    "ReduceAncientEnemyDamge", // not a typo
    "ResistCold",
    "ResistElectric",
    "ResistFreeze",
    "ResistHot",
    "ResistLightning",
    "SandMoveSpeedUp",
    "SnowMovingSpeed",
    "SpreadFire",
    "SurfMaster",
    "SwimSpeedUp",
    "Zoom",
] as const satisfies (SpecialStatusName | "")[];

/** uking::act::WeaponModifier */
export const WeaponModifier = {
    None: 0x0,
    AddPower: 0x1,
    AddLife: 0x2,
    Critical: 0x4,
    LongThrow: 0x8,
    SpreadFire: 0x10,
    Zoom: 0x20,
    RapidFire: 0x40,
    SurfMaster: 0x80,
    AddGuard: 0x100,
    Yellow: 0x80000000,
} as const;
export type WeaponModifier =
    (typeof WeaponModifier)[keyof typeof WeaponModifier];
// intentionally does not have a value to name mapping, since we only
// need the name for the special status to display the modifier
