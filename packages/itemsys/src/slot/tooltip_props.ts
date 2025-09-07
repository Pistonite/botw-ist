import type {
    InvView_GdtItem,
    InvView_ItemData,
    InvView_OverworldItem,
    InvView_PouchItem,
} from "@pistonite/skybook-api";

import {
    getItemTypeAndUse,
    getActorParam,
    getWeaponModifierStatusPropList,
    isEquipmentType,
    PouchItemType,
    type WeaponModifierStatusProps,
    gdtTypeToPouchItemType,
    isGdtDataEquipmentType,
    isGdtDataFoodType,
    normalizeIngredients,
    type CookEffect,
} from "../data";

import type { ItemSlotContextProps } from "./slot_props.ts";

export type ItemTooltipWithContextProps = ItemTooltipProps & ItemSlotContextProps;
export type ItemTooltipProps = {
    /** The actor name of this item and used to look up item properties */
    actor: string;

    /** For equipments, the durability line is shown with the current and max durability */
    isEquipment: boolean;

    /** Raw value of the item. If not specified, will not be displayed */
    value?: number;

    /** Show the "Equipped" line */
    isEquipped: boolean;

    /** Show the "Translucent" line (when isInInventory is false) */
    isTranslucent: boolean;

    /** If greater than 0, show the "Holding: X" line */
    holdingCount: number;

    /** Info for each of the weapon modifiers. Only displayed if isEquipment is true */
    weaponModifiers: WeaponModifierStatusProps[];

    /** Cook data of the item - always displayed unless undefined */
    cookData?: InvView_ItemData;

    /** If non-empty, show the ingredient list */
    ingredients: string[];

    /** Metadata to display if the item is from Pouch */
    pouchMeta?: ItemTooltipPouchMetadata;

    /** Metadata to display if the item is from GDT */
    gdtMeta?: ItemTooltipGdtMetadata;

    /** If true, show the "Persist over reload" line */
    isInBrokenSlot: boolean;

    /** If true, show the "Entangled" line */
    isEntangled: boolean;

    /** Status of the item, if it's in the overworld */
    overworldStatus?: "equipped" | "held" | "ground";

    /** If the overworld item is about to despawn */
    overworldWillDespawn?: boolean;

    /**
     * Accessibility status of the item
     *
     * - dpad-only: item can only be accessed via dpad
     * - dpad-none: item can't be accessed via dpad
     * - none: item can't be accessed whatsoever
     */
    accessibleStatus?: "dpad-only" | "dpad-none" | "none";

    /** string to show as the profile */
    profile: string;
};

/** See InvView_PouchItem */
export type ItemTooltipPouchMetadata = {
    itemType: number;
    itemUse: number;
    nodeAddr: bigint;
    nodeValid: boolean;
    nodePos: bigint;
    nodePrev: bigint;
    nodeNext: bigint;
    allocatedIndex: number;
    unallocatedIndex: number;
};

/** See InvView_GdtItem */
export type ItemTooltipGdtMetadata = {
    /** Index of this GDT slot */
    index: number;
    /** If the item is sword, the index of the sword */
    indexSword?: number | undefined;
    /** If the item is bow, the index of the bow */
    indexBow?: number | undefined;
    /** If the item is shield, the index of the shield */
    indexShield?: number | undefined;
    /** If the item is food, the index of the food */
    indexFood?: number | undefined;
};

export const getTooltipPropsFromActor = (actor: string, effect?: CookEffect): ItemTooltipProps => {
    return {
        actor,
        isEquipment: false,
        isEquipped: false,
        isTranslucent: false,
        holdingCount: 0,
        weaponModifiers: [],
        cookData: effect
            ? {
                  effectValue: 0,
                  effectId: effect,
                  sellPrice: 0,
                  effectDuration: 0,
                  effectLevel: 0,
              }
            : undefined,
        ingredients: [],
        isInBrokenSlot: false,
        isEntangled: false,
        profile: getActorParam(actor, "profile"),
    };
};

export const getTooltipPropsFromPouchItem = (
    item: InvView_PouchItem,
    isInBrokenSlot: boolean,
): ItemTooltipProps => {
    const { actorName, value, isEquipped } = item.common;

    // display modifier list and cook data based on the item type of the actor,
    // not the raw value in the memory, in case of corruption
    const [realItemType] = getItemTypeAndUse(actorName);
    const isEquipment = isEquipmentType(realItemType as PouchItemType);
    const weaponModifiers = isEquipment
        ? getWeaponModifierStatusPropList(
              actorName,
              realItemType,
              item.data.effectValue,
              item.data.sellPrice,
          )
        : [];
    // show the raw cook data only on food
    const isFood = realItemType === PouchItemType.Food;
    const cookData = isFood ? { ...item.data } : undefined;

    const accessibleStatus = item.accessible
        ? item.dpadAccessible
            ? undefined
            : "dpad-none"
        : item.dpadAccessible
          ? "dpad-only"
          : "none";

    return {
        actor: actorName,
        isEquipment,
        value,
        isEquipped,
        isTranslucent: !item.isInInventory,
        holdingCount: item.holdingCount,
        weaponModifiers,
        cookData,
        // ingredients can be on any item
        ingredients: normalizeIngredients(item.ingredients),
        pouchMeta: {
            itemType: item.itemType,
            itemUse: item.itemUse,
            nodeAddr: item.nodeAddr,
            nodeValid: item.nodeValid,
            nodePos: item.nodePos,
            nodePrev: item.nodePrev,
            nodeNext: item.nodeNext,
            allocatedIndex: item.allocatedIdx,
            unallocatedIndex: item.unallocatedIdx,
        },
        isInBrokenSlot,
        isEntangled: item.promptEntangled,
        accessibleStatus,
        profile: getActorParam(actorName, "profile"),
    };
};

export const getTooltipPropsFromGdtItem = (item: InvView_GdtItem): ItemTooltipProps => {
    const { actorName, value, isEquipped } = item.common;

    const gdtType = item.data.type;
    const data = item.data;
    const isEquipment = isGdtDataEquipmentType(data);
    const weaponModifiers = isEquipment
        ? getWeaponModifierStatusPropList(
              actorName,
              gdtTypeToPouchItemType(gdtType),
              data.info.value,
              data.info.flag,
          )
        : [];
    const isFood = isGdtDataFoodType(data);
    const cookData = isFood
        ? {
              ...data.info,
          }
        : undefined;
    const ingredients = isFood ? normalizeIngredients(data.ingredients) : [];

    return {
        actor: actorName,
        isEquipment,
        value,
        isEquipped,
        isTranslucent: false,
        holdingCount: 0,
        weaponModifiers,
        cookData,
        ingredients,
        gdtMeta: {
            index: item.idx,
            indexSword: item.data.type === "sword" ? item.data.idx : undefined,
            indexBow: item.data.type === "bow" ? item.data.idx : undefined,
            indexShield: item.data.type === "shield" ? item.data.idx : undefined,
            indexFood: item.data.type === "food" ? item.data.idx : undefined,
        },
        isInBrokenSlot: false,
        isEntangled: false,
        profile: getActorParam(actorName, "profile"),
    };
};

export const getTooltipPropsFromOverworldItem = (item: InvView_OverworldItem): ItemTooltipProps => {
    const actorName = item.actor;
    const [itemType] = getItemTypeAndUse(actorName);
    const isEquipment = item.type === "equipped" || item.type === "ground-equipment";

    const weaponModifiers = isEquipment
        ? getWeaponModifierStatusPropList(
              actorName,
              itemType,
              item.modifier.value,
              item.modifier.flag,
          )
        : [];

    let overworldStatus: ItemTooltipProps["overworldStatus"] = "ground";
    const overworldWillDespawn = item.type === "ground-item" && item.despawning;
    if (item.type === "equipped") {
        overworldStatus = "equipped";
    } else if (item.type === "held") {
        overworldStatus = "held";
    }

    return {
        actor: actorName,
        isEquipment,
        value: isEquipment ? item.value : undefined,
        isEquipped: false, // we use the overworld status, not inventory equipped status
        isTranslucent: false,
        holdingCount: 0, // we use the overworld status
        weaponModifiers,
        ingredients: [],
        isInBrokenSlot: false,
        isEntangled: false,
        overworldStatus,
        overworldWillDespawn,
        profile: getActorParam(actorName, "profile"),
    };
};
