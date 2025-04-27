import { getActorParam } from "./ActorData.ts";
import {
    CookEffect,
    PouchItemType,
    SpecialStatus,
    SpecialStatusNames,
    WeaponModifier,
} from "./EnumTypes.ts";
import {
    convertCookEffectToSpecialStatus,
    getWeaponSpecialStatusToDisplay,
    isEquipmentType,
} from "./EnumUtils.ts";

/** Display props for special status (i.e. the box shown on the top-right of the item) */
export type StatusProps = {
    /** The special status to display on the item slot */
    status: SpecialStatus;

    /**
     * The icon to display for the special status. Empty string for no icon
     *
     * Some status corresponds to multiple icons, like AddPower
     * for weapon and bow
     */
    statusIcon: string;

    /** Value text to display next to the modifier icon in the item slot */
    iconValue: string;

    /**
     * Use alternative color to display the iconValue. This is to
     * make special status values more visible in the UI (such as WMC meal)
     */
    isAlternativeColor: boolean;
};
/** Get the status props for an item slot */
export const getStatusProps = (
    actor: string,
    itemType: PouchItemType,
    effectId: number,
    effectValue: number,
    sellPrice: number,
): StatusProps => {
    // only display WeaponModifier for equipments
    if (isEquipmentType(itemType)) {
        return getStatusPropsForEquipment(
            actor,
            itemType,
            effectValue,
            sellPrice,
        );
    }
    // get from cook data
    let status: SpecialStatus = SpecialStatus.None;
    if (effectId !== CookEffect.None) {
        // convert function handles invalid effectId correctly, so the cast is fine
        status = convertCookEffectToSpecialStatus(effectId as CookEffect);
    }
    const data = getDefaultStatusPropsForActor(actor);
    // override status if any
    if (status !== SpecialStatus.None) {
        data.status = status;
        data.statusIcon = SpecialStatusNames[status];
    }
    // display price if it's odd, because it's probably used for WMC
    data.isAlternativeColor = sellPrice > 1 && sellPrice % 2 === 1;
    // display the price only if non-0 (normally it will be at least 2)
    data.iconValue = sellPrice > 1 ? `$${sellPrice}` : "";

    return data;
};

/** Get the status props for equipments (sword/bow/shield), i.e. props for displaying weapon modifier */
export const getStatusPropsForEquipment = (
    actor: string,
    itemType: PouchItemType,
    effectValue: number,
    modifierSet: number,
): StatusProps => {
    const status = getWeaponSpecialStatusToDisplay(modifierSet);
    let displayValue = 0;

    if (status === SpecialStatus.None) {
        // don't display any modifier on the item
        return getDefaultStatusPropsForActor(actor);
    }

    // only display value for AddPower and AddGuard
    if (status === SpecialStatus.AddPower) {
        displayValue += effectValue;
    }
    // value is doubled if both AddPower and AddGuard are present for shields
    if (
        itemType === PouchItemType.Shield &&
        (modifierSet & WeaponModifier.AddGuard) !== 0
    ) {
        displayValue += effectValue;
    }

    // Fix multishot icon
    let statusIcon: string = SpecialStatusNames[status] || "";
    if (status === SpecialStatus.SpreadFire) {
        statusIcon = getMultishotIcon(effectValue);
    } else if (itemType === PouchItemType.Bow) {
        // Fix bow attack up
        if (status === SpecialStatus.AddPower) {
            statusIcon = "AddPower_Bow";
        } else if (status === SpecialStatus.AddPowerPlus) {
            statusIcon = "AddPowerPlus_Bow";
        }
    }

    return {
        status,
        statusIcon,
        iconValue: displayValue ? `+${displayValue}` : "",
        isAlternativeColor: false,
    };
};

/** Display props for one specific WeaponModifier.*/
export type WeaponModifierStatusProps = {
    /** The special status corresponding to the modifier */
    status: SpecialStatus;

    /** The special status icon corresponding to the modifier */
    statusIcon: string;

    /**
     * If the modifier is active on the item type
     *
     * (for example, AddGuard is only active on Shields
     */
    active: boolean;

    /**
     * The formatted display value of the modifier
     *
     * Number for AddPower, AddGuard, SpreadFire, percentage
     * for LongThrow, RapidFire, SurfMaster
     */
    modifierValue: string;
};

export const getWeaponModifierStatusPropList = (
    actor: string,
    itemType: PouchItemType,
    effectValue: number,
    modifierSet: number,
): WeaponModifierStatusProps[] => {
    const output: WeaponModifierStatusProps[] = [];
    const yellow = (modifierSet & WeaponModifier.Yellow) !== 0;
    if (modifierSet & WeaponModifier.AddPower) {
        output.push({
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
            modifierValue: `${effectValue}`,
        });
    }
    if (modifierSet & WeaponModifier.AddLife) {
        output.push({
            status: SpecialStatus.AddLife,
            statusIcon: yellow ? "AddLifePlus" : "AddLife",
            active: true, // durability up, always active, although it doesn't do anything
            modifierValue: "",
        });
    }
    if (modifierSet & WeaponModifier.AddGuard) {
        output.push({
            status: SpecialStatus.AddGuard,
            statusIcon: yellow ? "AddGuardPlus" : "AddGuard",
            active: itemType === PouchItemType.Shield,
            modifierValue: `${effectValue}`,
        });
    }
    if (modifierSet & WeaponModifier.Critical) {
        output.push({
            status: SpecialStatus.Critical,
            statusIcon: "Critical",
            active: itemType === PouchItemType.Sword,
            modifierValue: "",
        });
    }
    if (modifierSet & WeaponModifier.LongThrow) {
        output.push({
            status: SpecialStatus.LongThrow,
            statusIcon: "LongThrow",
            active: itemType === PouchItemType.Sword,
            modifierValue: getModifierPercentDifference(effectValue),
        });
    }
    if (modifierSet & WeaponModifier.SpreadFire) {
        // This is guess on how it works based on experience
        // multishot is capped at 10. The faster the bow shoots, fewer arrows come out
        const bowChargeRate = getActorParam(actor, "bowArrowChargeRate");
        const quickShotMultiplier =
            (modifierSet & WeaponModifier.RapidFire) !== 0
                ? effectValue / 1000
                : 1;
        const multishot = 10 / (bowChargeRate * quickShotMultiplier);
        const modifierValue =
            multishot >= 10 ? "10" : `~${multishot.toFixed(2)}`;
        output.push({
            status: SpecialStatus.SpreadFire,
            statusIcon: getMultishotIcon(effectValue),
            active: itemType === PouchItemType.Bow,
            modifierValue,
        });
    }
    if (modifierSet & WeaponModifier.Zoom) {
        output.push({
            status: SpecialStatus.Zoom,
            statusIcon: "Zoom",
            active: itemType === PouchItemType.Bow,
            modifierValue: "",
        });
    }
    if (modifierSet & WeaponModifier.RapidFire) {
        output.push({
            status: SpecialStatus.RapidFire,
            statusIcon: "RapidFire",
            active: itemType === PouchItemType.Bow,
            modifierValue: getModifierPercentDifference(effectValue),
        });
    }
    if (modifierSet & WeaponModifier.SurfMaster) {
        output.push({
            status: SpecialStatus.SurfMaster,
            statusIcon: "SurfMaster",
            active: itemType === PouchItemType.Shield,
            modifierValue: getModifierPercentDifference(effectValue),
        });
    }

    return output;
};

/** Get the default props for actor based on its params (e.g. armor effect or bow default multishot) */
export const getDefaultStatusPropsForActor = (
    actorName: string,
): StatusProps => {
    const status = getActorParam(actorName, "specialStatus");

    let statusIcon: string = SpecialStatusNames[status] || "";
    // convert to the correct multishot icon
    if (status === SpecialStatus.SpreadFire) {
        const num = getActorParam(actorName, "bowLeadShotNum");
        statusIcon = getMultishotIcon(num);
    }
    return {
        status,
        statusIcon,
        iconValue: "",
        isAlternativeColor: false,
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
