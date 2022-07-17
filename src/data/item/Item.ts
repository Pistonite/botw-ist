import { createMaterialStack } from "./ItemStack";
import { getTabFromType, Item, ItemStack, ItemTab, ItemType } from "./type";

const TypeToCount: Omit<{
    [t in ItemType]: number
}, ItemType.Flag> = {
	[ItemType.Weapon]: 0,
	[ItemType.Bow]: 0,
	[ItemType.Arrow]: 0,
	[ItemType.Shield]: 0,
	[ItemType.ArmorUpper]: 0,
	[ItemType.ArmorMiddle]: 0,
	[ItemType.ArmorLower]: 0,
	[ItemType.Material]: 0,
	[ItemType.Food]: 0,
	[ItemType.Key]: 0,
};

export class ItemImpl implements Item {
	id: string;
	type: ItemType;
	get tab(): ItemTab {
		return getTabFromType(this.type);
	}
	repeatable: boolean;
	stackable: boolean;
	sortOrder = -1;
	image: string;
	configuredAnimatedImage?: string;
	defaultStackFactory?: (item: Item)=>ItemStack;
	constructor(
		id: string, 
		type: ItemType, 
		repeatable: boolean, 
		stackable: boolean, 
		image: string, 
		animatedImage: string|undefined,
		defaultStackFactory: ((item: Item)=>ItemStack)|undefined
	){
		this.id = id;
		this.type = type;
		this.repeatable = repeatable;
		this.stackable = stackable;
		this.image = image;
		this.configuredAnimatedImage = animatedImage;
		this.defaultStackFactory = defaultStackFactory;
		if(type !== ItemType.Flag){
			this.sortOrder = TypeToCount[type];
			TypeToCount[type]++;
		}
	}

	get animatedImage(): string {
		return this.configuredAnimatedImage || this.image;
	}

	createDefaultStack(): ItemStack {
		if(this.defaultStackFactory){
			return this.defaultStackFactory(this);
		}
		return createMaterialStack(this, 1);
	}
    
}
