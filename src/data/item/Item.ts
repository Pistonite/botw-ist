import { ItemStackImpl } from "./ItemStack";
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
	get localizationKey(): string {
		return `items.${ItemType[this.type]}.${this.id}`;
	}
	type: ItemType;
	get tab(): ItemTab {
		return getTabFromType(this.type);
	}
	repeatable: boolean;
	stackable: boolean;
	sortOrder = -1;
	image: string;
	configuredAnimatedImage?: string;
	priority: number;
	// bow has default zoom. default false
	bowZoom: boolean;
	// bow has default (spread) multishot, default 0
	bowMultishot: number;
	// bow has default rapid fire (centralized multishot), default 0
	bowRapidfire: number;
	isElixir: boolean;
	defaultStack: ItemStack;
	constructor(
		id: string,
		type: ItemType,
		repeatable: boolean,
		stackable: boolean,
		image: string,
		animatedImage: string|undefined,
		priority: number,
		bowZoom: boolean,
		bowMultishot: number,
		bowRapidfire: number,
		isElixir: boolean,
		defaultStackFactory: ((item: Item)=>ItemStack)|undefined
	){
		this.id = id;
		this.type = type;
		this.repeatable = repeatable;
		this.stackable = stackable;
		this.image = image;
		this.configuredAnimatedImage = animatedImage;
		this.priority = priority;
		if(defaultStackFactory){
			this.defaultStack = defaultStackFactory(this);
		}else{
			this.defaultStack = new ItemStackImpl(this);
		}
		
		if(type !== ItemType.Flag){
			this.sortOrder = TypeToCount[type];
			TypeToCount[type]++;
		}
		this.bowZoom = bowZoom;
		this.bowMultishot = bowMultishot;
		this.bowRapidfire = bowRapidfire;
		this.isElixir = isElixir;
	}

	get animatedImage(): string {
		return this.configuredAnimatedImage || this.image;
	}
}
