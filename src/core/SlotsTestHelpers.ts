import { createMaterialStack, getTabFromType, Item, ItemStack, ItemTab, ItemType } from "data/item";

class MockItem implements Item {
	id: string;
	type: ItemType;
	repeatable: boolean;
	stackable: boolean;
	sortOrder = -1;
	get tab(): ItemTab {
		return getTabFromType(this.type);
	}
	image = "";
	animatedImage= "";
	constructor(id: string, type: ItemType, stackable: boolean, repeatable: boolean){
		this.id = id;
		this.type = type;
		this.stackable = stackable;
		this.repeatable = repeatable;
	}
	createDefaultStack(): ItemStack {
		return createMaterialStack(this, 1);
	}
    
}

export const createArrowMockItem = (id: string): Item => new MockItem(id, ItemType.Arrow, true, true);
export const createMaterialMockItem = (id: string): Item => new MockItem(id, ItemType.Material, true, true);
export const createFoodMockItem = (id: string): Item => new MockItem(id, ItemType.Food, false, true);
export const createKeyMockItem = (id: string): Item => new MockItem(id, ItemType.Key, false, false);
export const createEquipmentMockItem = (id: string, type: ItemType): Item => new MockItem(id, type, false, true);

export const equalsExceptEquip = (a: ItemStack, b: ItemStack): boolean => a.equalsExceptForEquipped(b);
