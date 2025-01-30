import { getActorParam } from "./ActorData.ts";
import {
    CookEffect,
    effectToStatus,
    modifierToStatus,
    PouchItemType,
    SpecialStatus,
    WeaponModifier,
} from "./enums.ts";
import type { ItemSlotInfo } from "./ItemSlotInfo.ts";

/** Modifier display info derived from ItemSlotInfo and ActorData */
export type ModifierInfo = {
    /** The special status to display on the item slot */
    status: SpecialStatus;
    /**
     * The icon to display for the special status
     *
     * Some status corresponds to multiple icons, like AddPower
     * for weapon and bow
     */
    statusIcon: string;

    /** Value to display next to the modifier icon in the item slot */
    iconValue: string;

    /** Extra details for each of the weapon modifiers */
    details: ModifierDetail[];
};

/** Extra detail for each of the weapon modifiers */
export type ModifierDetail = {
    /** The special status corresponding to the modifier */
    status: SpecialStatus;
    /** The special status icon corresponding to the modifier */
    statusIcon: string;
    /** If the modifier is active on the item type */
    active: boolean;
    /**
     * The display value of the modifier
     *
     * Number for AddPower, AddGuard, SpreadFire, percentage
     * for LongThrow, RapidFire, SurfMaster
     */
    modifierValue: string;
};

/** Get the modifier info for an item slot */
export const getModifierInfo = (info: ItemSlotInfo): ModifierInfo => {
    const { actorName, itemType, modEffectId, modSellPrice } = info;
    // only display WeaponModifier for equipments
    if (
        itemType === PouchItemType.Sword ||
        itemType === PouchItemType.Bow ||
        itemType === PouchItemType.Shield
    ) {
        return getModifierInfoForEquipments(info);
    }
    // get from cook data
    let status = SpecialStatus.None;
    if (modEffectId !== CookEffect.None) {
        status = effectToStatus(modEffectId);
    }
    const defaultData = getDefaultModifierInfoForActor(actorName);
    // override status if any
    if (status !== SpecialStatus.None) {
        defaultData.status = status;
        defaultData.statusIcon = SpecialStatus[status];
    }
    // display price if it's odd, because it's probably used for WMC
    const iconValue =
        modSellPrice > 1 && modSellPrice % 2 === 1 ? `$${modSellPrice}` : "";
    defaultData.iconValue = iconValue;

    return defaultData;
};

export const getModifierInfoForEquipments = (
    info: ItemSlotInfo,
): ModifierInfo => {
    const { actorName, itemType, modEffectValue, modSellPrice } = info;
    const specialStatus = getWeaponSpecialStatusToDisplay(modSellPrice);
    let iconValue = 0;

    if (specialStatus === SpecialStatus.None) {
        return getDefaultModifierInfoForActor(actorName);
    }

    // only display value for AddPower and AddGuard
    if (specialStatus === SpecialStatus.AddPower) {
        iconValue += modEffectValue;
    }
    // value is doubled if both AddPower and AddGuard are present for shields
    if (
        itemType === PouchItemType.Shield &&
        (modSellPrice & WeaponModifier.AddGuard) !== 0
    ) {
        iconValue += modEffectValue;
    }

    // Fix multishot icon
    let statusIcon = SpecialStatus[specialStatus];
    if (specialStatus === SpecialStatus.SpreadFire) {
        statusIcon = getMultishotIcon(modEffectValue);
    }

    // Fix bow attack up
    if (itemType === PouchItemType.Bow) {
        if (specialStatus === SpecialStatus.AddPower) {
            statusIcon = "AddPower_Bow";
        } else if (specialStatus === SpecialStatus.AddPowerPlus) {
            statusIcon = "AddPowerPlus_Bow";
        }
    }

    // get effect details
    const details: ModifierDetail[] = [];
    const yellow = (modSellPrice & WeaponModifier.Yellow) !== 0;
    if (modSellPrice & WeaponModifier.AddPower) {
        details.push({
            status: SpecialStatus.AddPower,
            statusIcon:
                itemType === PouchItemType.Bow
                    ? yellow
                        ? "AddPowerPlus_Bow"
                        : "AddPower_Bow"
                    : yellow
                      ? "AddPowerPlus"
                      : "AddPower",
            active: true, // attack up is always active
            modifierValue: `${modEffectValue}`,
        });
    }
    if (modSellPrice & WeaponModifier.AddLife) {
        details.push({
            status: SpecialStatus.AddLife,
            statusIcon: yellow ? "AddLifePlus" : "AddLife",
            active: true, // durability up, always active, although it doesn't do anything
            modifierValue: "",
        });
    }
    if (modSellPrice & WeaponModifier.AddGuard) {
        details.push({
            status: SpecialStatus.AddGuard,
            statusIcon: yellow ? "AddGuardPlus" : "AddGuard",
            active: itemType === PouchItemType.Shield,
            modifierValue: `${modEffectValue}`,
        });
    }
    if (modSellPrice & WeaponModifier.Critical) {
        details.push({
            status: SpecialStatus.Critical,
            statusIcon: "Critical",
            active: itemType === PouchItemType.Sword,
            modifierValue: "",
        });
    }
    if (modSellPrice & WeaponModifier.LongThrow) {
        details.push({
            status: SpecialStatus.LongThrow,
            statusIcon: "LongThrow",
            active: itemType === PouchItemType.Sword,
            modifierValue: getModifierPercentDifference(modEffectValue),
        });
    }
    if (modSellPrice & WeaponModifier.SpreadFire) {
        // This is guess on how it works based on experience
        // multishot is capped at 10. The faster the bow shoots, fewer arrows come out
        const bowChargeRate = getActorParam(actorName, "bowArrowChargeRate");
        const quickShotMultiplier =
            (modSellPrice & WeaponModifier.RapidFire) !== 0
                ? modEffectValue / 1000
                : 1;
        const multishot = 10 / (bowChargeRate * quickShotMultiplier);
        const modifierValue =
            multishot >= 10 ? "10" : `~${multishot.toFixed(2)}`;
        details.push({
            status: SpecialStatus.SpreadFire,
            statusIcon: getMultishotIcon(modEffectValue),
            active: itemType === PouchItemType.Bow,
            modifierValue,
        });
    }
    if (modSellPrice & WeaponModifier.Zoom) {
        details.push({
            status: SpecialStatus.Zoom,
            statusIcon: "Zoom",
            active: itemType === PouchItemType.Bow,
            modifierValue: "",
        });
    }
    if (modSellPrice & WeaponModifier.RapidFire) {
        details.push({
            status: SpecialStatus.RapidFire,
            statusIcon: "RapidFire",
            active: itemType === PouchItemType.Bow,
            modifierValue: getModifierPercentDifference(modEffectValue),
        });
    }
    if (modSellPrice & WeaponModifier.SurfMaster) {
        details.push({
            status: SpecialStatus.SurfMaster,
            statusIcon: "SurfMaster",
            active: itemType === PouchItemType.Shield,
            modifierValue: getModifierPercentDifference(modEffectValue),
        });
    }

    return {
        status: specialStatus,
        statusIcon,
        iconValue: iconValue ? `+${iconValue}` : "",
        details,
    };
};

const getWeaponSpecialStatusToDisplay = (
    modifierSet: number,
): SpecialStatus => {
    // 0x7100aa7290 in 1.5.0
    // https://discord.com/channels/269611402854006785/269616041435332608/1041497732474482698
    // select the modifier to display from the bitset
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

    return modifierToStatus(
        selectedModifier,
        (modifierSet & WeaponModifier.Yellow) !== 0,
    );
};

export const getDefaultModifierInfoForActor = (
    actorName: string,
): ModifierInfo => {
    const status = getActorParam(actorName, "specialStatus");

    let statusIcon = SpecialStatus[status];
    // convert to the correct multishot icon
    if (status === SpecialStatus.SpreadFire) {
        const num = getActorParam(actorName, "bowLeadShotNum");
        statusIcon = getMultishotIcon(num);
    }
    return {
        status,
        statusIcon,
        iconValue: "",
        details: [],
    };
};

export const getMultishotIcon = (num: number): string => {
    if (num <= 3) {
        return "SpreadFire_3";
    }
    if (num <= 5) {
        return "SpreadFire_5";
    }
    return "SpreadFire_X";
};

const getModifierPercentDifference = (value: number): string => {
    if (value === 1000) {
        return "0%";
    }
    const percentage = (value - 1000) / 10;

    return `${percentage > 0 ? "+" : ""}${percentage.toFixed(1)}%`;
};
