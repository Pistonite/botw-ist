import { getTabFromType, Item, ItemStack, ItemTab, ItemType } from "data/item";
/* import-validate-exempt*/import { ItemStackImpl } from "data/item/ItemStack";

class MockItem implements Item {
	id: string;
	get localizationKey(): string{
		return this.id;
	}
	type: ItemType;
	repeatable: boolean;
	stackable: boolean;
	sortOrder = -1;
	get tab(): ItemTab {
		return getTabFromType(this.type);
	}
	get tabOrArrow(): ItemTab | ItemType.Arrow {
		return this.type === ItemType.Arrow ? ItemType.Arrow : this.tab;
	}
	image = "";
	animatedImage= "";
	priority = 0;

	bowZoom = false;
	bowMultishot = 0;
	bowRapidfire = 0;
	isElixir = false;
	constructor(id: string, type: ItemType, stackable: boolean, repeatable: boolean){
		this.id = id;
		this.type = type;
		this.stackable = stackable;
		this.repeatable = repeatable;
	}

	get defaultStack(): ItemStack{
		return new ItemStackImpl(this);
	}

}

export const createMaterialStack = (item: Item, count: number): ItemStack => {
	return item.defaultStack.modify({count});
};

export const createEquipmentStack = (item: Item, durability: number, equipped: boolean): ItemStack => {
	return item.defaultStack.modify({durability, equipped});
};

export const createArrowMockItem = (id: string): Item => new MockItem(id, ItemType.Arrow, true, true);
export const createMaterialMockItem = (id: string): Item => new MockItem(id, ItemType.Material, true, true);
export const createFoodMockItem = (id: string): Item => new MockItem(id, ItemType.Food, false, true);
export const createFoodMockItemStackable = (id: string): Item => new MockItem(id, ItemType.Food, true, true);
export const createKeyMockItem = (id: string): Item => new MockItem(id, ItemType.Key, false, false);
export const createKeyMockItemStackable = (id: string): Item => new MockItem(id, ItemType.Key, true, true);
export const createEquipmentMockItem = (id: string, type: ItemType): Item => new MockItem(id, type, false, id!=="MasterSword");

export const equalsExceptEquip = (a: ItemStack, b: ItemStack): boolean => a.equalsExcept(b, "equipped");

export const createMockItems = (ids: string[]): Record<string, Item> =>  {
	const items: Record<string, Item> = {};
	ids.forEach(id=>{
		const idLower = id.toLowerCase();
		if (id.startsWith("Arrow")){
			items[idLower] = createArrowMockItem(id);
		} else if (id.startsWith("Material")){
			items[idLower] = createMaterialMockItem(id);
		} else if (id.startsWith("Food")){
			items[idLower] = createFoodMockItem(id);
		} else if (id.startsWith("FoodStackable")){
			items[idLower] = createKeyMockItemStackable(id);
		} else if (id.startsWith("Key")){
			items[idLower] = createKeyMockItem(id);
		} else if (id.startsWith("KeyStackable")){
			items[idLower] = createKeyMockItemStackable(id);
		} else if (id.startsWith("Weapon")){
			items[idLower] = createEquipmentMockItem(id, ItemType.Weapon);
		} else if (id.startsWith("Shield")){
			items[idLower] = createEquipmentMockItem(id, ItemType.Shield);
		} else if (id.startsWith("Bow")){
			items[idLower] = createEquipmentMockItem(id, ItemType.Bow);
		} else if (id.startsWith("ArmorUpper")){
			items[idLower] = createEquipmentMockItem(id, ItemType.ArmorUpper);
		} else if (id.startsWith("ArmorMiddle")){
			items[idLower] = createEquipmentMockItem(id, ItemType.ArmorMiddle);
		} else if (id.startsWith("ArmorLower")){
			items[idLower] = createEquipmentMockItem(id, ItemType.ArmorLower);
		}
	});
	return items;
};

export const createMockItemSearch = (items: Record<string, Item>) => (id: string): ItemStack | undefined => {
	return items[id.replaceAll("*", "").toLowerCase()]?.defaultStack;
};
