import { ItemStack, ItemType } from "data/item";

export type DisplayableSlot = {
	// image to display
    image: string,
	// localization key for description
    descKey: string,
	// count of stack
    count?: number,
	// durability of stack
	durability?: string,
	// if the stack is equipped
    isEquipped: boolean,
	// if the slot is broken (i.e in the count offset region)
    isBrokenSlot: boolean,
}

export interface DisplayableInventory {
    getDisplayedSlots: (isIconAnimated: boolean)=>DisplayableSlot[]
}

export const itemStackToDisplayableSlot = ({item, count, durability, equipped}: ItemStack, isBrokenSlot: boolean, isIconAnimated: boolean): DisplayableSlot => {
	// for unstackable items (food/key items) display count if count > 1, even if it's unstackable
	const shouldDisplayDurability = item.type === ItemType.Weapon || item.type === ItemType.Bow || item.type === ItemType.Shield;
	const displayDurability = Number.isInteger(durability) ? durability + "" : durability.toPrecision(4);
	const shouldDisplayCount = !shouldDisplayDurability && (item.stackable ? item.type === ItemType.Arrow || count > 0 : count > 1);
	return {
		image: isIconAnimated ? item.animatedImage : item.image,
		descKey: `items.${ItemType[item.type]}.${item.id}`,
		count: shouldDisplayCount?count:undefined,
		durability: shouldDisplayDurability?displayDurability:undefined,
		isEquipped: equipped,
		isBrokenSlot
	};
};
