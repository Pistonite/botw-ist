// Type of the item
export enum ItemType {
    Weapon = 0,
    Bow = 1,
    Arrow = 2,
    Shield = 3,
    ArmorUpper = 4,
    ArmorMiddle = 5,
    ArmorLower = 6,
    Material = 7,
    Food = 8,
    Key = 9,
    Flag = -1 // flags in game data, not actual items. such as HasRitoSoulPlus
}
// Which tab the item is in. These specifically matches ItemType in case we need it in the future
export enum ItemTab {
    Weapon = 0,
    Bow = 1,
    Shield = 3,
    Armor = 4,
    Material = 7,
    Food = 8,
    Key = 9,
    None = -1,
}

export const iterateItemTabs = (): ItemTab[] => [
	ItemTab.Weapon,
	ItemTab.Bow,
	ItemTab.Shield,
	ItemTab.Armor,
	ItemTab.Material,
	ItemTab.Food,
	ItemTab.Key
];

export const getTabFromType = (type: ItemType): ItemTab => {
	switch(type){
		case ItemType.Weapon:
			return ItemTab.Weapon;
		case ItemType.Bow:
		case ItemType.Arrow:
			return ItemTab.Bow;
		case ItemType.Shield:
			return ItemTab.Shield;
		case ItemType.ArmorUpper:
		case ItemType.ArmorMiddle:
		case ItemType.ArmorLower:
			return ItemTab.Armor;
		case ItemType.Material:
			return ItemTab.Material;
		case ItemType.Food:
			return ItemTab.Food;
		case ItemType.Key:
			return ItemTab.Key;
		default:
			return ItemTab.None;
	}
};
export interface Item {
    // The id of Item, which is its name in UpperCamelCase in English as it appears in English, with special characters like ' removed and + turned into Plus
    // The only special case is that the key item Thunderhelm is named ThunderHelmKey
    readonly id: string,
    // key for localization
    readonly localizationKey: string,
    // The type of the item
    readonly type: ItemType
    // if this is false, the item will not be added to pouch if one already exists
    readonly repeatable: boolean,
    // if the item is stackable
    readonly stackable: boolean,
    // sort order of the item
    readonly sortOrder: number,
    // which tab the item is in
    readonly tab: ItemTab,
    // webpack loaded image
    readonly image: string,
    // animated image. If the item is not animated, this is the same as image
    readonly animatedImage: string,
    // priority when matching items in the same category. Can be positive or negative
    readonly priority: number,
    // Default bow properties
    readonly bowZoom: boolean,
    readonly bowMultishot: number,
    readonly bowRapidfire: number,
    // if the item is an elixir
    readonly isElixir: boolean,
    // get item stack with this item and default metadata (durability and default state for weapons, cookdata for meals, etc)
    readonly defaultStack: ItemStack,
}

// ItemStack is an immutable object holding information of a slot
export interface ItemStack {
    // type of the item
    readonly item: Item,
    // how many in slot. for weapon, this is durability*100
    readonly count: number,
    // durability of equipment. for material, this is count/100
    readonly durability: number,
    // if slot is equipped
    readonly equipped: boolean,
    // Cook effect (None if not food)
    readonly foodEffect: CookEffect,
    // Weapon modifier bits (None if not weapon)
    readonly weaponModifier: number,
    // Sell price (0 if not food)
    readonly foodSellPrice: number,
    // Hearts recovered for food (0 if not food)
    readonly foodHpRecover: number,
    // Weapon modifier value (0 if not weapon/bow/shield)
    readonly weaponValue: number,
    // function to create a new stack based on this stack and option
    modify(option: Partial<ItemStack>): ItemStack,
    // function to create a new stack based on this stack and meta option
    modifyMeta(metaOption: MetaModifyOption): ItemStack,
    // check if 2 stacks are equal: same item, count, equipped and metadata
    equals(other: ItemStack): boolean,
    // equals except the specified meta keys
    equalsExcept(other: ItemStack, ...keys: (keyof ItemStack)[]): boolean,
}

export const dumpItemStack = (stack: ItemStack) => {

	return {
		item: {
			id: stack.item.id
		},
		count: stack.count,
		equipped: stack.equipped,
		foodEffect: stack.foodEffect,
		weaponModifier: stack.weaponModifier,
		weaponValue: stack.weaponValue
	};
};

export type ItemIdMap = { [id: string]: Item};

// the extra data on an item stack
export type MetaModifyOption = Partial<{
    // life value, count or durability*100
    life: number,
    // equipped.
    equip: boolean,
    // food sell price or weapon modifier
    price: number,
    // modifier hearts recover value
    hp: number,
    // food effect
    cookEffect: CookEffect
}>;

// JS bitwise operations are 32 bits
// but the numbers are 64 bits
export const WeaponModifier = {
	None: 0,
	AttackUp: 1,
	DurabilityUp: 1 << 1,
	CriticalHit: 1 << 2,
	LongThrow: 1 << 3,
	MultiShot: 1 << 4,
	Zoom: 1 << 5,
	QuickShot: 1 << 6,
	SurfMaster: 1 << 7,
	GuardUp: 1 << 8,
	Yellow: 1 << 31
} as const;

export const getWeaponModifierName = (modifier: number): string => {
	for(const name in WeaponModifier){
		if (WeaponModifier[name as keyof typeof WeaponModifier] === modifier){
			return name;
		}
	}
	return "";
};

export enum CookEffect {
    None,
    Chilly, // Alias: hotresist
    Spicy, // Alias: coldresist
    Electro,
    Sneaky,// Alias: stealth
    Energizing,
    Enduring,
    Hasty, // Alias: speed
    Mighty,
    Tough,
    Fireproof,
    Hearty,
}

export const iterateCookEffect = (): CookEffect[] => [
	CookEffect.None,
	CookEffect.Chilly,
	CookEffect.Spicy,
	CookEffect.Electro,
	CookEffect.Sneaky,
	CookEffect.Energizing,
	CookEffect.Enduring,
	CookEffect.Hasty,
	CookEffect.Mighty,
	CookEffect.Tough,
	CookEffect.Fireproof,
	CookEffect.Hearty,
];

export interface ExData {
    hearts: number,
    modifierValue: number,
    sellPrice: number,
    modifierType: number
}
