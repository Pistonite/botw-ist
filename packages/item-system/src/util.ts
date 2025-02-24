import type { ItemSlotInfo } from "@pistonite/skybook-api";

import { getActorParam } from "./data/ActorData.ts";
import {
    CookEffect,
    isEquipment,
    ItemUse,
    PouchItemType,
} from "./data/enums.ts";

/** Populate item slot info from actor name and optional properties */
export const makeItemSlotInfo = (
    actorName: string,
    rest: Partial<ItemSlotInfo> = {},
): ItemSlotInfo => {
    const [itemType, itemUse] = getItemTypeAndUse(actorName);
    const value = isEquipment(itemType)
        ? getActorParam(actorName, "generalLife") * 100
        : 1;
    return {
        actorName,
        itemType,
        itemUse,
        value,
        isEquipped: false,
        isInInventory: true,
        modEffectValue: 0,
        modEffectDuration: 0,
        modSellPrice: 0,
        modEffectId: CookEffect.None,
        modEffectLevel: 0,
        ingredientActorNames: ["", "", "", "", ""],
        listPos: 0,
        unallocated: false,
        poolPos: 0,
        isInBrokenSlot: false,
        holdingCount: 0,
        promptEntangled: false,
        ...rest,
    };
};

export const getItemTypeAndUse = (
    actorName: string,
): [PouchItemType, ItemUse] => {
    const profile = getActorParam(actorName, "profile");
    switch (profile) {
        case "WeaponSmallSword":
            return [PouchItemType.Sword, ItemUse.WeaponSmallSword];
        case "WeaponLargeSword":
            return [PouchItemType.Sword, ItemUse.WeaponLargeSword];
        case "WeaponSpear":
            return [PouchItemType.Sword, ItemUse.WeaponSpear];
        case "WeaponBow":
            return [PouchItemType.Bow, ItemUse.WeaponBow];
        case "WeaponShield":
            return [PouchItemType.Shield, ItemUse.WeaponShield];
        case "ArmorHead":
            return [PouchItemType.ArmorHead, ItemUse.ArmorHead];
        case "ArmorUpper":
            return [PouchItemType.ArmorUpper, ItemUse.ArmorUpper];
        case "ArmorLower":
            return [PouchItemType.ArmorLower, ItemUse.ArmorLower];
    }

    const isFood = actorName.startsWith("Item_Cook_");
    if (profile !== "Item" && profile !== "PlayerItem") {
        return [
            isFood ? PouchItemType.Food : PouchItemType.Material,
            ItemUse.Item,
        ];
    }

    if (getActorParam(actorName, "isCureItem")) {
        return [
            isFood ? PouchItemType.Food : PouchItemType.Material,
            ItemUse.CureItem,
        ];
    }

    if (getActorParam(actorName, "isImportant")) {
        return [PouchItemType.KeyItem, ItemUse.ImportantItem];
    }

    return [isFood ? PouchItemType.Food : PouchItemType.Material, ItemUse.Item];
};
