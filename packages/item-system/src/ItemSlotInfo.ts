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
    actorName: string,

    /** 
     * PouchItem::mType
     *
     * Note this is raw memory value and may not be a valid enum value
     */
    itemType: PouchItemType | number,

    /**
     * PouchItem::mItemUse
     *
     * Note this is raw memory value and may not be a valid enum value
     */
    itemUse: ItemUse | number,

    /** 
     * PouchItem::mValue
     *
     * This is stack size or durability * 100
     */
    value: number,

    /** PouchItem::mEquipped */
    isEquipped: boolean,

    /** PouchItem::mInInventory */
    isInInventory: boolean,

    /** 
     * This is either the weapon modifier value, 
     * or the HP recovery value for food (in quarter-hearts)
     */
    modEffectValue: number,

    /** 
     * For food with a timed effect, this is the duration in seconds.
     * For stamina, this is the raw value
     */
    modEffectDuration: number,

    /**
     * For weapon modifier, this is the flag bitset. For food,
     * this is the sell price
     */
    modSellPrice: number,

    /**
     * Effect ID for the food
     *
     * Note this is raw memory value and may not be a valid enum value
     */
    modEffectId: CookEffect | number,

    /**
     * The level of the effect, *usually* 1-3. However this 
     * is the raw memory value and may not be valid
     */
    modEffectLevel: number,

    /**
     * PouchItem::mIngredients. Length should always be 5
     */
    ingredientActorNames: string[],

    /** 
     * The item's position in the item list.
     *
     * If the item is in the unallocated pool, this is its position
     * in the unallocated pool (stack). 0 is the top of the stack/beginning
     * of the list
     */
    listPosition: number,

    /** If the item is currently in the unallocated pool */
    unallocated: boolean,

    /** 
     * The item's position in the pool
     *
     * This basically serves as a unique pointer to the item
     */
    poolPosition: number,

    /** If the item is in "broken" slot, i.e. will be transferred on reload */
    isInBrokenSlot: boolean,

    /** 
     * Number of items held if the item is being held by the player
     */
    holdingCount: number,

    /**
     * Enable the prompt entangled state for this slot
     */
    promptEntangled: boolean,
}

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
