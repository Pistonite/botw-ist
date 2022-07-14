import Images from "assets/img";

import itemDataJson from "config/items.json";
type ItemDataObject = { type: string, options?: Partial<ItemData> };
const itemMap = itemDataJson as Record<string, ItemDataObject>;

export enum ItemType {
    Weapon = 0,
    Bow = 1,
    Arrow = 2,
    Shield = 3,
    Armor = 4,
    Material = 5,
    Food = 6,
    Key = 7
}

export const ItemTypes = [
	ItemType.Weapon,
	ItemType.Bow,
	ItemType.Arrow,
	ItemType.Shield,
	ItemType.Armor,
	ItemType.Material,
	ItemType.Food,
	ItemType.Key
];

export type ItemStack = {
    item: string,
    count: number,
    equipped: boolean
}

type ItemData = {
	item: string,
	image: string,
	type: ItemType,
	repeatable: boolean,
	stackable: boolean,
  animated: boolean
	animatedImage?: string,
	sortOrder: number,
}

const ItemToData: Record<string, ItemData> = {};
const TypeToCount = {
	[ItemType.Weapon]: 0,
	[ItemType.Bow]: 0,
	[ItemType.Arrow]: 0,
	[ItemType.Shield]: 0,
	[ItemType.Armor]: 0,
	[ItemType.Material]: 0,
	[ItemType.Key]: 0,
	[ItemType.Food]: 0,
};
const register = (item: string, type: ItemType, options?: Partial<ItemData>) => {
	const sortOrder = TypeToCount[type];
	TypeToCount[type]++;
	const data: ItemData = {
		item,
		type,
		repeatable: true,
		stackable: true,
		animated: false,
		sortOrder,
		...options||{},
		// If defined, the "image" on the options object is actually an image key. Thus, we must resolve it after
		// options are applied to override it with the correct value (falling back on item name if undefined)
		animatedImage: options?.animated ? Images[`${ItemType[type]}/${options?.image ?? item}Animated`] : undefined,
		image: Images[`${ItemType[type]}/${options?.image ?? item}`],
	};
	ItemToData[item] = data;
};

for (const item in itemMap) {
	const data: ItemDataObject = itemMap[item];
	register(item, ItemType[data.type as keyof typeof ItemType], data.options);
}

export const itemToItemData = (item: string): ItemData => ItemToData[item] as ItemData;
export const itemExists = (item: string): boolean => !!itemToItemData(item);
export const itemToArrowType = (item: string): string => {
	if(itemToItemData(item).type === ItemType.Arrow){
		const str = `${item}`;
		return str.substring(0,str.length-5);
	}
	return "";
};

export const getAllItems = (): string[] => Object.keys(ItemToData);
