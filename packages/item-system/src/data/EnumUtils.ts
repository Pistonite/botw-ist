import { getActorParam } from "./ActorData.ts";
import {
    CookEffect,
    PouchCategory,
    PouchItemType,
    PouchItemUse,
    SpecialStatus,
    WeaponModifier,
} from "./EnumTypes.ts";

export const isEquipmentType = (itemType: PouchItemType): boolean => {
    return (
        itemType === PouchItemType.Sword ||
        itemType === PouchItemType.Bow ||
        itemType === PouchItemType.Shield
    );
};

/**
 * Convert a single weapon modifier to a special status. Only works
 * if the input corresponds to a single modifier, not a combination (bitflag)
 */
export const convertWeaponModifierToSpecialStatus = (
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
export const convertCookEffectToSpecialStatus = (effect: CookEffect): SpecialStatus => {
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

/** Port of getType and getItemUse in PMDM */
export const getItemTypeAndUse = (actorName: string): [PouchItemType, PouchItemUse] => {
    const isArrow = getActorParam(actorName, "isArrow");
    if (isArrow) {
        return [PouchItemType.Arrow, PouchItemUse.Item];
    }
    const profile = getActorParam(actorName, "profile");
    switch (profile) {
        case "WeaponSmallSword":
            return [PouchItemType.Sword, PouchItemUse.WeaponSmallSword];
        case "WeaponLargeSword":
            return [PouchItemType.Sword, PouchItemUse.WeaponLargeSword];
        case "WeaponSpear":
            return [PouchItemType.Sword, PouchItemUse.WeaponSpear];
        case "WeaponBow":
            return [PouchItemType.Bow, PouchItemUse.WeaponBow];
        case "WeaponShield":
            return [PouchItemType.Shield, PouchItemUse.WeaponShield];
        case "ArmorHead":
            return [PouchItemType.ArmorHead, PouchItemUse.ArmorHead];
        case "ArmorUpper":
            return [PouchItemType.ArmorUpper, PouchItemUse.ArmorUpper];
        case "ArmorLower":
            return [PouchItemType.ArmorLower, PouchItemUse.ArmorLower];
        case "HouseReins":
            return [PouchItemType.KeyItem, PouchItemUse.Item];
    }

    // port for hasTag(CookResult) and hasTag(RoastItem) since we don't extract
    // that to ActorData
    const isFood =
        actorName.startsWith("Item_Cook_") ||
        actorName.startsWith("Item_RoastFish_") ||
        actorName.startsWith("Item_Roast_") ||
        actorName.startsWith("Item_Chilled_") ||
        actorName.startsWith("Item_ChilledFish_") ||
        actorName === "Item_Boiled_01";
    const foodType = isFood ? PouchItemType.Food : PouchItemType.Material;
    if (profile !== "Item" && profile !== "PlayerItem") {
        return [foodType, PouchItemUse.Item];
    }

    if (getActorParam(actorName, "isCureItem")) {
        return [foodType, PouchItemUse.CureItem];
    }

    if (getActorParam(actorName, "isImportant")) {
        return [PouchItemType.KeyItem, PouchItemUse.ImportantItem];
    }

    return [foodType, PouchItemUse.Item];
};

/**
 * Port of 0x7100aa7290 in 1.5.0
 *
 * See https://discord.com/channels/269611402854006785/269616041435332608/1041497732474482698
 *
 * This function is used to select the modifier to display in the inventory from the bitset
 */
export const getWeaponSpecialStatusToDisplay = (modifierSet: number): SpecialStatus => {
    const applicableModifiers = [
        WeaponModifier.AddPower,
        WeaponModifier.AddLife,
        WeaponModifier.AddGuard,
        WeaponModifier.Critical,
        WeaponModifier.LongThrow,
        WeaponModifier.SpreadFire,
        WeaponModifier.Zoom,
        WeaponModifier.RapidFire,
        WeaponModifier.SurfMaster,
    ];

    let selectedModifier: WeaponModifier = WeaponModifier.None;
    for (let i = 0; i < applicableModifiers.length; i++) {
        if ((applicableModifiers[i] & modifierSet) !== WeaponModifier.None) {
            selectedModifier = applicableModifiers[i];
            break;
        }
    }

    return convertWeaponModifierToSpecialStatus(
        selectedModifier,
        (modifierSet & WeaponModifier.Yellow) !== 0,
    );
};

export const getPouchCategoryFromType = (type: number): PouchCategory => {
    switch (type) {
        case PouchItemType.Sword:
            return PouchCategory.Sword;
        case PouchItemType.Bow:
        case PouchItemType.Arrow:
            return PouchCategory.Bow;
        case PouchItemType.Shield:
            return PouchCategory.Shield;
        // note only ArmorHead is usually possible in the tabs data
        case PouchItemType.ArmorHead:
        case PouchItemType.ArmorUpper:
        case PouchItemType.ArmorLower:
            return PouchCategory.Armor;
        case PouchItemType.Material:
            return PouchCategory.Material;
        case PouchItemType.Food:
            return PouchCategory.Food;
        case PouchItemType.KeyItem:
            return PouchCategory.KeyItem;
    }
    return PouchCategory.Invalid;
};
