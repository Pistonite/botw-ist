// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * Common (display) info for an item
 */
export type InvView_CommonItem = {
    /**
     * Name of the item actor
     *
     * This is stored in PouchItem::mName, or the
     * PorchItem flag
     */
    actorName: string;
    /**
     * Raw value of the item, could be count or durability
     *
     * This is stored in PouchItem::mValue, or the
     * PorchItem_Value1 flag
     */
    value: number;
    /**
     * Equip flag
     *
     * This is PouchItem::mEquipped or the PorchItem_EquipFlag flag
     */
    isEquipped: boolean;
};
