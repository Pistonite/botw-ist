import { ItemStack } from "data/item";

export class ItemStackCommandWrapper {
	stack: ItemStack;
	number: number;
	constructor(stack: ItemStack, number: number){
		this.stack = stack;
		this.number = number;
	}

	public getStackAndSlotCount(): [ItemStack, number] {
		if(this.stack.item.stackable){
			return [this.stack.modify({count: this.number}), 1];
		}
		return [this.stack, this.number];
	}
}
