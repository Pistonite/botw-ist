import { ItemStack, itemToItemData, ItemType } from "./Item";
import { getDisplayValue } from "./Localization";

export type DisplayableSlot = {
    image: string,
    description: string,
    count: number,
    displayCount: boolean,
    isEquipped: boolean,
    isBrokenSlot: boolean,
}

export interface DisplayableInventory {
    getDisplayedSlots: (isIconAnimated: boolean)=>DisplayableSlot[]
}

export const itemStackToDisplayableSlot = ({item, count, equipped}: ItemStack, isBrokenSlot: boolean, isIconAnimated: boolean): DisplayableSlot => {
	const data =  itemToItemData(item);
	return {
		image: isIconAnimated ? data.animatedImage ?? data.image : data.image,
		description: getDisplayValue(`description.${ItemType[data.type]}.${data.item}`, data.item),
		// for unstackable items (food/key items) display count if count > 1, even if it's unstackable
		displayCount: data.stackable ? data.type === ItemType.Arrow || count > 0 : count > 1,
		count,
		isEquipped: equipped,
		isBrokenSlot
	};
};
