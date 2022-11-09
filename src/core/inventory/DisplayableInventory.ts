import { ItemStack, ItemType } from "data/item";
import { SlotDisplay } from "./types";

export class SlotDisplayForItemStack implements SlotDisplay {
	private stack: ItemStack;
	isBrokenSlot: boolean = false;
	isIconAnimated: boolean = false;
	propertyString: string = "";
	propertyClassName: string = "";
	constructor(stack: ItemStack){
		this.stack = stack;
	}

	get image(): string {
		return this.isIconAnimated ? this.stack.item.animatedImage : this.stack.item.image;
	}

	get count(): number | undefined {
		if(this.shouldDisplayDurability()){
			return undefined;
		}
		// not 1: always display
		// 1: display if stackable
		if(this.stack.item.stackable || this.stack.count !==  1){
			return this.stack.count;
		}
		return undefined;
	}

	get durability(): string | undefined {
		if (!this.shouldDisplayDurability()){
			return undefined;
		}
		const durability = this.stack.durability;
		return Number.isInteger(durability) ? durability + "" : durability.toPrecision(4);
	}

	get isEquipped(): boolean {
		return this.stack.equipped;
	}

	get modifierImage(): string | undefined{
		return undefined; //TODO
	}

	get modifierText(): string | undefined{
		return undefined; //TODO
	}

	public getTooltip(translate: (s: string) => string): [string, string][] {
		return this.stack.getTooltip(translate);
	}

	private shouldDisplayDurability(): boolean {
		return [ItemType.Weapon, ItemType.Bow, ItemType.Shield].includes(this.stack.item.type);
	}
}

