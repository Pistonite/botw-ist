import { getActorParam } from "./ActorData.ts";
import { CookEffect, isEquipment, ItemUse, PouchItemType } from "./enums.ts";

/**
 * Information to display an item slot
 *
 * These are data derived from the native PouchItem class
 * and simulator runtime info. Data that can be looked up
 * from the item's parameters should not be included here.
 */
export type ItemSlotInfo = {
    /**
     * Name of the actor, from PouchItem::mName
     *
     * This is what will be used to look up extra data for the item
     */
    actorName: string;

    /**
     * PouchItem::mType
     *
     * Note this is raw memory value and may not be a valid enum value
     */
    itemType: PouchItemType | number;

    /**
     * PouchItem::mItemUse
     *
     * Note this is raw memory value and may not be a valid enum value
     */
    itemUse: ItemUse | number;

    /**
     * PouchItem::mValue
     *
     * This is stack size or durability * 100
     */
    value: number;

    /** PouchItem::mEquipped */
    isEquipped: boolean;

    /** PouchItem::mInInventory */
    isInInventory: boolean;

    /**
     * This is either the weapon modifier value,
     * or the HP recovery value for food (in quarter-hearts)
     */
    modEffectValue: number;

    /**
     * For food with a timed effect, this is the duration in seconds.
     * For stamina, this is the raw value
     */
    modEffectDuration: number;

    /**
     * For weapon modifier, this is the flag bitset. For food,
     * this is the sell price
     */
    modSellPrice: number;

    /**
     * Effect ID for the food
     *
     * Note this is raw memory value and may not be a valid enum value
     */
    modEffectId: CookEffect | number;

    /**
     * The level of the effect, *usually* 1-3. However this
     * is the raw memory value and may not be valid
     */
    modEffectLevel: number;

    /**
     * PouchItem::mIngredients. Length should always be 5
     */
    ingredientActorNames: string[];

    /**
     * The item's position in the item list.
     *
     * If the item is in the unallocated pool, this is its position
     * in the unallocated pool (stack). 0 is the top of the stack/beginning
     * of the list
     */
    listPosition: number;

    /** If the item is currently in the unallocated pool */
    unallocated: boolean;

    /**
     * The item's position in the pool
     *
     * This basically serves as a unique pointer to the item
     */
    poolPosition: number;

    /** If the item is in "broken" slot, i.e. will be transferred on reload */
    isInBrokenSlot: boolean;

    /**
     * Number of items held if the item is being held by the player
     */
    holdingCount: number;

    /**
     * Enable the prompt entangled state for this slot
     */
    promptEntangled: boolean;
};

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
        ingredientActorNames: [],
        listPosition: 0,
        unallocated: false,
        poolPosition: 0,
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
