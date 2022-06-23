import { Item, ItemStack, itemToItemData, ItemType } from "./Item"

export type DisplayableSlot = {
    image: string,
    count: number,
    displayCount: boolean,
    isEquipped: boolean,
    isBrokenSlot: boolean,
}

export interface DisplayableInventory {
    getDisplayedSlots: ()=>DisplayableSlot[]
}

export const itemStackToDisplayableSlot = ({item, count, equipped}: ItemStack, isBrokenSlot: boolean): DisplayableSlot => {
    const data =  itemToItemData(item);
    return {
        image: data.image,
        displayCount: data.stackable && (data.type === ItemType.Arrow || count > 0),
        count,
        isEquipped: equipped,
        isBrokenSlot
    }
}
