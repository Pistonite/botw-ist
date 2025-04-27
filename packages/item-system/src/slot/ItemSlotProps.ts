import type { ActorSpriteProps } from "botw-item-assets";
import type {
    InvView_GdtItem,
    InvView_PouchItem,
} from "@pistonite/skybook-api";

import {
    type CookEffect,
    CookEffectNames,
    gdtTypeToPouchItemType,
    getActorParam,
    getDefaultStatusPropsForActor,
    getItemTypeAndUse,
    getStatusProps,
    getStatusPropsForEquipment,
    isEquipmentType,
    isGdtDataEquipmentType,
    isGdtDataFoodType,
    PouchItemType,
    type StatusProps,
} from "../data";

/** Props to display an item slot */
export type ItemSlotProps = {
    /** The actor name of this item and used to look up item properties */
    actor: string;

    /** Effect for elixir icon (ignored if actor is not elixir) */
    elixirEffect?: string;

    /** If the item should display with "equipped" (blue) background */
    isEquipped: boolean;

    /** If the item should display with a transparent background to indicate IsInInventory = false (i.e. Translucent) */
    isTranslucent: boolean;

    /** If not undefined, the item will display with "x{Count}" at the bottom-left. Note 0 will also be displayed */
    count?: number | undefined;

    /**
     * If not undefined, the item will display the formatted durability at the botoom-left. Note 0 will also be displayed
     * This should be the durability value (raw value/100)
     */
    durability?: number | undefined;

    /** If true, show the red background for broken slots */
    isInBrokenSlot: boolean;

    /** If true, show the entangled icon */
    isEntangled: boolean;

    /** If greater than 0, display the holding indicator */
    holdingCount: number;
} & StatusProps &
    Pick<ActorSpriteProps, "blank" | "deactive" | "badlyDamaged">;

export const getSlotPropsFromActor = (
    actor: string,
    effect?: CookEffect,
): ItemSlotProps => {
    const [realItemType] = getItemTypeAndUse(actor);
    const isEquipment = isEquipmentType(realItemType);
    const status = effect
        ? getStatusProps(actor, PouchItemType.Food, effect, 0, 0)
        : getDefaultStatusPropsForActor(actor);
    return {
        actor,
        elixirEffect: effect ? CookEffectNames[effect as number] || "" : "",
        isEquipped: false,
        isTranslucent: false,
        durability: isEquipment
            ? getDurability(
                  isEquipment,
                  getActorParam(actor, "generalLife") * 100,
              )
            : undefined,
        isInBrokenSlot: false,
        isEntangled: false,
        holdingCount: 0,
        ...status,
    };
};

export const getSlotPropsFromPouchItem = (
    item: InvView_PouchItem,
    list1Count: number,
): ItemSlotProps => {
    const { actorName, value, isEquipped } = item.common;
    const isAbility = isChampionAbilityActor(actorName);
    const canStack = getActorParam(actorName, "canStack");
    const [realItemType] = getItemTypeAndUse(actorName);
    const isEquipment = isEquipmentType(realItemType);

    const status = getStatusProps(
        actorName,
        realItemType,
        item.data.effectId,
        item.data.effectValue,
        item.data.sellPrice,
    );

    return {
        actor: actorName,
        elixirEffect: CookEffectNames[item.data.effectId] || undefined,
        isEquipped: !isAbility && isEquipped,
        isTranslucent: !item.isInInventory,
        count: getCount(isEquipment, value, canStack),
        durability: getDurability(isEquipment, value),
        isInBrokenSlot: item.allocatedIdx >= list1Count,
        isEntangled: item.promptEntangled,
        holdingCount: item.holdingCount,
        ...status,
        blank: item.isNoIcon,
        deactive: getDeactive(isAbility, isEquipped, actorName, value),
        badlyDamaged: isEquipment && value <= 300,
    };
};

export const getSlotPropsFromGdtItem = (
    item: InvView_GdtItem,
): ItemSlotProps => {
    const { actorName, value, isEquipped } = item.common;
    const isAbility = isChampionAbilityActor(actorName);
    const canStack = getActorParam(actorName, "canStack");

    const gdtType = item.data.type;
    const data = item.data;
    const isEquipment = isGdtDataEquipmentType(data);
    const isFood = isGdtDataFoodType(data);
    let status: StatusProps;
    if (isEquipment) {
        const { value, flag } = data.data.info;
        status = getStatusPropsForEquipment(
            actorName,
            gdtTypeToPouchItemType(gdtType),
            value,
            flag,
        );
    } else if (isFood) {
        const { effectId, effectValue, sellPrice } = data.data.info;
        status = getStatusProps(
            actorName,
            PouchItemType.Food,
            effectId,
            effectValue,
            sellPrice,
        );
    } else {
        status = getDefaultStatusPropsForActor(actorName);
    }

    return {
        actor: actorName,
        elixirEffect: isFood
            ? CookEffectNames[data.data.info.effectId] || undefined
            : undefined,
        isEquipped: !isAbility && isEquipped,
        isTranslucent: false,
        count: getCount(isEquipment, value, canStack),
        durability: getDurability(isEquipment, value),
        isInBrokenSlot: false,
        isEntangled: false,
        holdingCount: 0,
        ...status,
        deactive: getDeactive(isAbility, isEquipped, actorName, value),
        // don't show badlyDamaged for GDT items
    };
};

const isChampionAbilityActor = (actor: string) => {
    return /^Obj_(DLC_)?HeroSoul_(Gerudo|Goron|Rito|Zora)$/.test(actor);
};

const getCount = (
    isEquipment: boolean,
    value: number,
    canStack: boolean,
): number | undefined => {
    if (isEquipment) {
        return undefined;
    }
    if (canStack || value > 0) {
        return value;
    }
    return undefined;
};

const getDurability = (
    isEquipment: boolean,
    value: number,
): number | undefined => {
    if (isEquipment) {
        return value / 100;
    }
    return undefined;
};

const getDeactive = (
    isAbility: boolean,
    isEquipped: boolean,
    actorName: string,
    value: number,
): boolean => {
    if (isAbility && !isEquipped) {
        return true;
    }
    if (actorName === "Weapon_Sword_070" && value <= 0) {
        return true;
    }
    return false;
};
