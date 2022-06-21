import ImageSlate from "assets/img/Slate.png";
import ImageGlider from "assets/img/Glider.png";
import ImageSpiritOrb from "assets/img/SpiritOrb.png";
import ImageLotus from "assets/img/Lotus.png";
import ImageSilentPrincess from "assets/img/SilentPrincess.png";
import ImageHoney from "assets/img/Lotus.png";
import ImageAcorn from "assets/img/SilentPrincess.png";
import ImageFaroshScale from "assets/img/FaroshScale.png";
import ImageFaroshClaw from "assets/img/FaroshClaw.png";
import ImageFaroshHorn from "assets/img/FaroshHorn.png";
import ImageHeartyBass from "assets/img/HeartyBass.png";
import ImageBeetle from "assets/img/Beetle.png";
import ImageOpal from "assets/img/Opal.png";
import ImageDiamond from "assets/img/Diamond.png";
import ImageTail from "assets/img/Tail.png";
import ImageSpring from "assets/img/Spring.png";
import ImageShaft from "assets/img/Shaft.png";
import ImageCore from "assets/img/Core.png";
import ImageWood from "assets/img/Wood.png";
import ImageSpeedFood from "assets/img/SpeedFood.png";
import ImageAxe from "assets/img/Axe.png";
import ImageBow from "assets/img/ForestDwellerBow.png";
import ImageArrow from "assets/img/NormalArrow.png";
import ImageFireArrow from "assets/img/FireArrow.png";
import ImageIceArrow from "assets/img/IceArrow.png";
import ImageShockArrow from "assets/img/ShockArrow.png";
import ImageBombArrow from "assets/img/BombArrow.png";
import ImageAncientArrow from "assets/img/AncientArrow.png";
import ImageShield from "assets/img/PotLid.png";

export enum ItemType {
    Weapon = 0,
    Bow = 1,
	Arrow = 2,
    Shield = 3,
    Material = 4,
    Meal = 5,
    Key = 6
}

export const ItemTypes = [
    ItemType.Weapon,
    ItemType.Bow,
	ItemType.Arrow,
    ItemType.Shield,
    ItemType.Material,
    ItemType.Meal,
    ItemType.Key
]

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

    SpeedFood = "SpeedFood",
	Weapon = "Weapon",
	Bow = "Bow",
	NormalArrow = "NormalArrow",
	FireArrow = "FireArrow",
	IceArrow = "IceArrow",
	ShockArrow = "ShockArrow",
	BombArrow = "BombArrow",
	AncientArrow = "AncientArrow",
	Shield = "Shield"
}

type ItemData = {
	item: Item,
	image: string,
	id: number,
	type: ItemType,
	repeatable: boolean,
	stackable: boolean,
	sortOrder: number,
}

const IdToData: {[id: number]: ItemData} = {};
const ItemToData: {[k in Item]?: ItemData} = {};
const TypeToCount = {
	[ItemType.Weapon]: 0,
	[ItemType.Bow]: 0,
	[ItemType.Arrow]: 0,
	[ItemType.Shield]: 0,
	[ItemType.Material]: 0,
	[ItemType.Key]: 0,
	[ItemType.Meal]: 0,
};
const register = (id: number, item: Item, type: ItemType, image: string, options?: Partial<ItemData>) => {
	const sortOrder = TypeToCount[type];
	TypeToCount[type]++;
	const data: ItemData = {
		item,
		image,
		id,
		type,
		repeatable: true,
		stackable: true,
		sortOrder,
		...(options||{})
	};
	IdToData[id] = data;
	ItemToData[item] = data;
}
/* Do not change the ID once created. Otherwise you would break existing codes */
register(0x00, Item.Slate, ItemType.Key, ImageSlate, {
	repeatable: false,
	stackable: false
});
register(0x01, Item.Glider, ItemType.Key, ImageGlider, {
	repeatable: false,
	stackable: false
});
register(0x02, Item.SpiritOrb, ItemType.Key, ImageSpiritOrb);

register(0x11, Item.Lotus, ItemType.Material, ImageLotus);
register(0x12, Item.SilentPrincess, ItemType.Material, ImageSilentPrincess);
register(0x13, Item.Honey, ItemType.Material, ImageHoney);
register(0x14, Item.Acorn, ItemType.Material, ImageAcorn);
register(0x15, Item.FaroshScale, ItemType.Material, ImageFaroshScale);
register(0x16, Item.FaroshClaw, ItemType.Material, ImageFaroshClaw);
register(0x17, Item.FaroshHorn, ItemType.Material, ImageFaroshHorn);
register(0x18, Item.HeartyBass, ItemType.Material, ImageHeartyBass);
register(0x19, Item.Beetle, ItemType.Material, ImageBeetle);
register(0x1a, Item.Opal, ItemType.Material, ImageOpal);
register(0x10, Item.Diamond, ItemType.Material, ImageDiamond);
register(0x1b, Item.Tail, ItemType.Material, ImageTail);
register(0x1c, Item.Spring, ItemType.Material, ImageSpring);
register(0x1d, Item.Shaft, ItemType.Material, ImageShaft);
register(0x1e, Item.Core, ItemType.Material, ImageCore);
register(0x1f, Item.Wood, ItemType.Material, ImageWood);

register(0x40, Item.SpeedFood, ItemType.Meal, ImageSpeedFood, {
	stackable: false
});

register(0x50, Item.Weapon, ItemType.Weapon, ImageAxe, {
	stackable: false
});

register(0x60, Item.Bow, ItemType.Bow, ImageBow, {
	stackable: false
});
register(0x70, Item.NormalArrow, ItemType.Arrow, ImageArrow);
register(0x71, Item.FireArrow, ItemType.Arrow, ImageFireArrow);
register(0x72, Item.IceArrow, ItemType.Arrow, ImageIceArrow);
register(0x73, Item.ShockArrow, ItemType.Arrow, ImageShockArrow);
register(0x74, Item.BombArrow, ItemType.Arrow, ImageBombArrow);
register(0x75, Item.AncientArrow, ItemType.Arrow, ImageAncientArrow);
register(0x60, Item.Shield, ItemType.Shield, ImageShield, {
	stackable: false
});

export const idToItemData = (id: number): ItemData => IdToData[id];
export const itemToItemData = (item: Item): ItemData => ItemToData[item] as ItemData;
export const itemToArrowType = (item: Item): string => {
	if(itemToItemData(item).type === ItemType.Arrow){
		const str = `${item}`;
		return str.substring(0,str.length-5);
	}
	return "";
}
