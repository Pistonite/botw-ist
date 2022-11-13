import { ItemStack } from "data/item";

export const AmountAll = "All";
export type AmountAllType = typeof AmountAll;


export class ItemStackArg{
	public stack: ItemStack;
	public number: number | AmountAllType;
	constructor(stack: ItemStack, number: number | AmountAllType){
		this.stack = stack;
		this.number = number;
	}

	public getContextStackAndSlotCount(): [ItemStack, number | AmountAllType] {
		if(this.number !== AmountAll && this.stack.item.stackable){
			return [this.stack.modify({count: this.number}), 1];
		}
		return [this.stack, this.number];
	}

	public equals(other: ItemStackArg): boolean {
		return this.stack.equals(other.stack) && this.number === other.number;
	}

}

// converts stacks from command to stacks to add
export const getSlotsToAdd = (stacks: ItemStackArg[]): ItemStack[] => {
	const returnStacks: ItemStack[] = [];
	stacks.forEach(stack=>{
		const [actualStack, count] = stack.getContextStackAndSlotCount();
		if(count === "All"){
			console.log("temp fix");
			returnStacks.push(actualStack.modify({count: -1}));
		}else{
			for(let i=0;i<count;i++){
				returnStacks.push(actualStack);
			}
		}
		
	});
	return returnStacks;
};
