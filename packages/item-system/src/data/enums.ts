/**
 * uking::ui::PouchItemType
 */
export enum PouchItemType {
    Sword = 0,
    Bow = 1,
    Arrow = 2,
    Shield = 3,
    ArmorHead = 4,
    ArmorUpper = 5,
    ArmorLower = 6,
    Material = 7,
    Food = 8,
    KeyItem = 9,
    Invalid = -1,
}

/**
 * Enum for inventory category type (tab types)
 *
 * See uking::ui::PouchCategory
 *
 * Used as index for mListHeads
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
export type PouchCategory = typeof PouchCategory[keyof typeof PouchCategory];
export type PouchCategoryName = keyof typeof PouchCategory;
export const PouchCategoryNames: PouchCategoryName[] = [
    "Sword",
    "Bow",
    "Shield",
    "Armor",
    "Material",
    "Food",
    "KeyItem",
];

export const isEquipment = (itemUse: PouchItemType): boolean => {
    return (
        itemUse === PouchItemType.Sword ||
        itemUse === PouchItemType.Bow ||
        itemUse === PouchItemType.Shield
    );
};

/**
 * uking::ui::ItemUse
 */
export enum ItemUse {
    WeaponSmallSword = 0,
    WeaponLargeSword = 1,
    WeaponSpear = 2,
    WeaponBow = 3,
    WeaponShield = 4,
    ArmorHead = 5,
    ArmorUpper = 6,
    ArmorLower = 7,
    Item = 8,
    ImportantItem = 9,
    CureItem = 10,
    Invalid = -1,
}

/** uking::CookEffectId */
export enum CookEffect {
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

/**
 * Internal used special status enum
 *
 * These correspond to modifier icons
 * These are used in generated data - DO NOT CHANGE
 */
export enum SpecialStatus {
    None = 0,
    AddGuard = 1,
    AddGuardPlus,
    AddLife,
    AddLifePlus,
    AddPower,
    AddPowerPlus,
    AllSpeed,
    AttackUp,
    ClimbSpeedUp,
    Critical,
    DefenseUp,
    ExGutsMaxUp,
    Fireproof, // "ResistBurn", not the fire-immunity effect
    GutsRecover,
    LifeMaxUp,
    LongThrow,
    Quietness,
    RapidFire,
    ReduceAncientEnemyDamge, // not a typo
    ResistCold,
    ResistElectric,
    ResistFreeze,
    ResistHot,
    ResistLightning,
    SandMoveSpeedUp,
    SnowMovingSpeed,
    SpreadFire,
    SurfMaster,
    SwimSpeedUp,
    Zoom,
}

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

/** Convert a WeaponModifier to a SpecialStatus */
export const modifierToStatus = (
    modifier: WeaponModifier,
    yellow: boolean,
): SpecialStatus => {
    switch (modifier) {
        case WeaponModifier.AddPower:
            return yellow ? SpecialStatus.AddPowerPlus : SpecialStatus.AddPower;
        case WeaponModifier.AddLife:
            return yellow ? SpecialStatus.AddLifePlus : SpecialStatus.AddLife;
        case WeaponModifier.AddGuard:
            return yellow ? SpecialStatus.AddGuardPlus : SpecialStatus.AddGuard;
        case WeaponModifier.Critical:
            return SpecialStatus.Critical;
        case WeaponModifier.LongThrow:
            return SpecialStatus.LongThrow;
        case WeaponModifier.SpreadFire:
            return SpecialStatus.SpreadFire;
        case WeaponModifier.Zoom:
            return SpecialStatus.Zoom;
        case WeaponModifier.RapidFire:
            return SpecialStatus.RapidFire;
        case WeaponModifier.SurfMaster:
            return SpecialStatus.SurfMaster;
    }
    return SpecialStatus.None;
};

/** Convert a CookEffect to a SpecialStatus */
export const effectToStatus = (effect: CookEffect): SpecialStatus => {
    switch (effect) {
        case CookEffect.LifeMaxUp:
            return SpecialStatus.LifeMaxUp;
        case CookEffect.ResistHot:
            return SpecialStatus.ResistHot;
        case CookEffect.ResistCold:
            return SpecialStatus.ResistCold;
        case CookEffect.ResistElectric:
            return SpecialStatus.ResistElectric;
        case CookEffect.AttackUp:
            return SpecialStatus.AttackUp;
        case CookEffect.DefenseUp:
            return SpecialStatus.DefenseUp;
        case CookEffect.Quietness:
            return SpecialStatus.Quietness;
        case CookEffect.AllSpeed:
            return SpecialStatus.AllSpeed;
        case CookEffect.GutsRecover:
            return SpecialStatus.GutsRecover;
        case CookEffect.ExGutsMaxUp:
            return SpecialStatus.ExGutsMaxUp;
        case CookEffect.Fireproof:
            return SpecialStatus.Fireproof;
    }
    return SpecialStatus.None;
};
