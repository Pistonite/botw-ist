import { Item, ItemStack, MetaOption } from "./type";

class ItemStackImpl implements ItemStack {
	public item: Item;
	life: number;
	get count(): number {
		return this.life;
	}
	get durability(): number {
		return this.life/100.0;
	}
	public equipped: boolean;
	constructor(item: Item, life: number, equipped: boolean) {
		this.item = item;
		this.life = life;
		this.equipped = equipped;
	}

	public modify(option: Partial<ItemStack>): ItemStack {
		const newItem = "item" in option ? option.item as Item : this.item;
		let newLife = this.life;
		if("count" in option){
			newLife = option.count as number;
		}else if("durability" in option){
			newLife = option.durability as number;
			newLife*=100;
		}
		const newEquipped = "equipped" in option ? !!option.equipped:this.equipped;
		return new ItemStackImpl(newItem, newLife, newEquipped);
	}

	public modifyMeta(metaOption: MetaOption): ItemStack {
		let modifyOption: Partial<ItemStack> = {};
		if("life" in metaOption){
			modifyOption = {
				...modifyOption,
				count: metaOption.life
			};
		}
		if("equip" in metaOption){
			modifyOption = {
				...modifyOption,
				equipped: metaOption.equip
			};
		}
		return this.modify(modifyOption);
	}

	public equals(other: ItemStack): boolean {
		return this.equalsExceptForEquipped(other) && this.equipped === other.equipped;
	}
	public equalsExceptForEquipped(other: ItemStack): boolean {
		return this.canStack(other) && this.life === other.count;
	}
	public canStack(other: ItemStack): boolean {
		// TODO metadata
		return this.item === other.item;
	}

}

export const createMaterialStack = (item: Item, count: number): ItemStack => {
	return new ItemStackImpl(item, count, false);
};

export const createEquipmentStack = (item: Item, durability: number, equipped: boolean): ItemStack => {
	return new ItemStackImpl(item, durability*100, equipped);
};
