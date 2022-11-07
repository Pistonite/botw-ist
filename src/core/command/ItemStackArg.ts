import { ItemStack } from "data/item";

export const AmountAll = "All";
export type AmountAllType = typeof AmountAll;


export class ItemStackArg{
	stack: ItemStack;
	number: number | AmountAllType;
	constructor(stack: ItemStack, number: number | AmountAllType){
		this.stack = stack;
		this.number = number;
	}

	public getStackAndSlotCount(): [ItemStack, number | AmountAllType] {
		if(this.number !== AmountAll && this.stack.item.stackable){
			return [this.stack.modify({count: this.number}), 1];
		}
		return [this.stack, this.number];
	}

	public equals(other: ItemStackArg): boolean {
		return this.stack.equals(other.stack) && this.number === other.number;
	}
}
