import { getElixir } from "./elixir";
import { ExDataImpl } from "./extra";
import { CookEffect, ExData, Item, ItemStack, ItemType, MetaModifyOption } from "./type";

type ItemStackModifyOption = {
	-readonly [P in keyof ItemStack]: ItemStack[P]
}

export class ItemStackImpl implements ItemStack {
	_item: Item;
	get item(): Item {
		// elixir check
		if(this._item.isElixir){
			return getElixir(this.foodEffect);
		}
		return this._item;
	}
	private life: number = 1;
	get count(): number {
		return this.life;
	}
	get durability(): number {
		return this.life/100.0;
	}
	public equipped: boolean = false;
	public foodEffect: CookEffect = CookEffect.None;
	private exData: ExData = new ExDataImpl();
	get weaponModifier(): number {
		return this.exData.modifierType;
	}
	get weaponValue(): number {
		return this.exData.modifierValue;
	}
	get foodSellPrice(): number {
		return this.exData.sellPrice;
	}
	get foodHpRecover(): number {
		return this.exData.hearts;
	}
	constructor(item: Item) {
		this._item = item;
	}

	public modify(option: Partial<ItemStack>): ItemStack {
		const newItem = "item" in option ? option.item as Item : this.item;
		const newStack = new ItemStackImpl(newItem);
		let newLife = this.life;
		if("count" in option){
			newLife = option.count as number;
		}else if("durability" in option){
			newLife = option.durability as number;
			newLife*=100;
		}
		newStack.life = newLife;
		newStack.equipped = "equipped" in option ? !!option.equipped:this.equipped;
		newStack.foodEffect = option.foodEffect ?? this.foodEffect;
		newStack.exData.modifierType = option.weaponModifier ?? option.foodSellPrice ?? this.weaponModifier;
		newStack.exData.modifierValue = option.weaponValue ?? option.foodHpRecover ?? this.weaponValue;

		return newStack;
	}

	public modifyMeta(metaOption: MetaModifyOption): ItemStack {
		let modifyOption: Partial<ItemStackModifyOption> = {};
		if("life" in metaOption){
			modifyOption.count = metaOption.life ?? 0;
		}
		if("equip" in metaOption){
			modifyOption.equipped = !!metaOption.equip;
		}
		if("price" in metaOption){
			modifyOption.foodSellPrice = metaOption.price ?? 0;
		}
		if("hp" in metaOption){
			modifyOption.foodHpRecover = metaOption.hp ?? 0;
		}
		if("cookEffect" in metaOption){
			modifyOption.foodEffect = metaOption.cookEffect ?? 0;
		}
		return this.modify(modifyOption);
	}

	public equals(other: ItemStack): boolean {
		return this.equalsExcept(other);
	}

	public equalsExcept(other: ItemStack, ...keys: (keyof ItemStack)[]): boolean {
		// If we grow to like 20 keys we could use a set.. but I doubt it
		if(this.item !== other.item){
			return false;
		}
		if(!keys.includes("equipped") && this.equipped !== other.equipped){
			return false;
		}
		if(!keys.includes("count") && !keys.includes("durability") && this.life !== other.count){
			return false;
		}
		if(!keys.includes("foodSellPrice") && !keys.includes("weaponModifier") && this.foodSellPrice !== other.foodSellPrice){
			return false;
		}
		if(!keys.includes("foodHpRecover") && !keys.includes("weaponValue") && this.foodHpRecover !== other.foodHpRecover){
			return false;
		}
		if(!keys.includes("foodEffect") && this.foodEffect !== other.foodEffect){
			return false;
		}

		return true;
	}
}

