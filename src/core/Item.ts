import Images from "assets/img";

export enum ItemType {
    Weapon = 0,
    Bow = 1,
	Arrow = 2,
    Shield = 3,
	Armor = 4,
    Material = 5,
    Meal = 6,
    Key = 7
}

export const ItemTypes = [
	ItemType.Weapon,
	ItemType.Bow,
	ItemType.Arrow,
	ItemType.Shield,
	ItemType.Material,
	ItemType.Meal,
	ItemType.Key
];

export type ItemStack = {
    item: Item,
    count: number,
    equipped: boolean
}

export enum Item {
    Slate = "Slate",
    Glider = "Glider",
    SpiritOrb = "SpiritOrb",
    
    Lotus = "Lotus",
    SilentPrincess = "SilentPrincess",
    Honey = "Honey",
    Acorn = "Acorn",
    FaroshScale = "FaroshScale",
    FaroshClaw = "FaroshClaw",
    FaroshHorn = "FaroshHorn",
    HeartyBass = "HeartyBass",
    Beetle = "Beetle",
    Opal = "Opal",
    Diamond = "Diamond",
    Tail = "Tail",
    Spring = "Spring",
    Shaft = "Shaft",
    Core = "Core",
    Wood = "Wood",

	Rushroom = "Rushroom",
	Screw = "Screw",
	HyruleBass = "HyruleBass",
	LizalfosHorn = "LizalfosHorn",
	LizalfosTalon = "LizalfosTalon",

    SpeedFood = "SpeedFood",
	EnduraFood = "EnduraFood",
	Weapon = "Weapon",
	Bow = "Bow",
	NormalArrow = "NormalArrow",
	FireArrow = "FireArrow",
	IceArrow = "IceArrow",
	ShockArrow = "ShockArrow",
	BombArrow = "BombArrow",
	AncientArrow = "AncientArrow",
	Shield = "Shield",

	Apple = "Apple",
	HylianShroom = "HylianShroom",
	SpicyPepper = "SpicyPepper",
	EnduraShroom = "EnduraShroom",
	HeartyRadish = "HeartyRadish",
	BigHeartyRadish = "BigHeartyRadish",
	Fairy = "Fairy",

	MasterSword = "MasterSword",
}

type ItemData = {
	item: Item,
	image: string,
	type: ItemType,
	repeatable: boolean,
	stackable: boolean,
	sortOrder: number,
}

const ItemToData: {[k in Item]?: ItemData} = {};
const TypeToCount = {
	[ItemType.Weapon]: 0,
	[ItemType.Bow]: 0,
	[ItemType.Arrow]: 0,
	[ItemType.Shield]: 0,
	[ItemType.Armor]: 0,
	[ItemType.Material]: 0,
	[ItemType.Key]: 0,
	[ItemType.Meal]: 0,
};
const register = (_id: number, item: Item, type: ItemType, options?: Partial<ItemData>) => {
	const sortOrder = TypeToCount[type];
	TypeToCount[type]++;
	const data: ItemData = {
		item,
		image: Images[`${item}`],
		type,
		repeatable: true,
		stackable: true,
		sortOrder,
		...options||{}
	};
	// if(id in IdToData){
	// 	console.error("Multiple items registered to the same id: "+id+", ("+item+")");
	// }
	//IdToData[id] = data;
	ItemToData[item] = data;
};

register(0x00, Item.Slate, ItemType.Key, {
	repeatable: false,
	stackable: false
});
register(0x01, Item.Glider, ItemType.Key, {
	repeatable: false,
	stackable: false
});
register(0x02, Item.SpiritOrb, ItemType.Key);
register(0, Item.Apple, ItemType.Material);
register(0, Item.SpicyPepper, ItemType.Material);
register(0x11, Item.Lotus, ItemType.Material);
register(0, Item.EnduraShroom, ItemType.Material);
register(0, Item.HylianShroom, ItemType.Material);
register(0x20, Item.Rushroom, ItemType.Material);
register(0, Item.BigHeartyRadish, ItemType.Material);
register(0, Item.HeartyRadish, ItemType.Material);
register(0x12, Item.SilentPrincess, ItemType.Material);
register(0x13, Item.Honey, ItemType.Material);
register(0x14, Item.Acorn, ItemType.Material);
register(0x15, Item.FaroshScale, ItemType.Material);
register(0x16, Item.FaroshClaw, ItemType.Material);
register(0x17, Item.FaroshHorn, ItemType.Material);
register(0x18, Item.HeartyBass, ItemType.Material);
register(0x21, Item.HyruleBass, ItemType.Material);
register(0, Item.Fairy, ItemType.Material);
register(0x19, Item.Beetle, ItemType.Material);
register(0x1a, Item.Opal, ItemType.Material);
register(0x10, Item.Diamond, ItemType.Material);
register(0x23, Item.LizalfosHorn, ItemType.Material);
register(0x24, Item.LizalfosTalon, ItemType.Material);
register(0x1b, Item.Tail, ItemType.Material);
register(0x22, Item.Screw, ItemType.Material);
register(0x1c, Item.Spring, ItemType.Material);
register(0x1d, Item.Shaft, ItemType.Material);
register(0x1e, Item.Core, ItemType.Material);
register(0x1f, Item.Wood, ItemType.Material);

register(0x40, Item.SpeedFood, ItemType.Meal, {
	stackable: false
});
register(0, Item.EnduraFood, ItemType.Meal, {
	stackable: false
});
register(0x50, Item.Weapon, ItemType.Weapon, {
	image: Images.Axe,
	stackable: false
});
register(0, Item.MasterSword, ItemType.Weapon, {
	stackable: false,
})

register(0x60, Item.Bow, ItemType.Bow, {
	image: Images.ForestDwellerBow,
	stackable: false
});
register(0x70, Item.NormalArrow, ItemType.Arrow);
register(0x71, Item.FireArrow, ItemType.Arrow);
register(0x72, Item.IceArrow, ItemType.Arrow);
register(0x73, Item.ShockArrow, ItemType.Arrow);
register(0x74, Item.BombArrow, ItemType.Arrow);
register(0x75, Item.AncientArrow, ItemType.Arrow);
register(0x80, Item.Shield, ItemType.Shield, {
	image: Images.PotLid,
	stackable: false
});

//export const idToItemData = (id: number): ItemData => IdToData[id];
export const itemToItemData = (item: Item): ItemData => ItemToData[item] as ItemData;
export const itemToArrowType = (item: Item): string => {
	if(itemToItemData(item).type === ItemType.Arrow){
		const str = `${item}`;
		return str.substring(0,str.length-5);
	}
	return "";
};
