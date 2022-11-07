import { ItemStack } from "data/item";
import { ItemStackArg } from "./ItemStackArg";

// export const joinItemStackString = (initial: string, stacks: ItemStackCommandWrapper[]): string => {
// 	const parts: string[] = [initial];
// 	stacks.forEach(({stack, number})=>{
// 		parts.push(""+number);
// 		parts.push(itemStackToString(stack));
// 	});
// 	return parts.join(" ");
// };

// const itemStackToString = (stack: ItemStack): string => {
// 	return stack.item.id;
// };

// converts stacks from command to stacks to add
export const processWrappers = (stacks: ItemStackArg[]): ItemStack[] => {
	const returnStacks: ItemStack[] = [];
	stacks.forEach(stack=>{
		const [actualStack, count] = stack.getStackAndSlotCount();
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

export const isAddVerb = (token: string): boolean => {
	return keywordMatchAny(token, [
		"get",
		"cook",
		"add",
		"pickup",
		"buy"
	]);
};

export const isRemoveVerb = (token: string): boolean => {
	return keywordMatchAny(token, [
		"remove",
		"sell",
		"eat",
		"drop",
		"with"
	]);
};

export const keywordMatch = (token: string, keyword: string): boolean => {
	return token.toLowerCase() === keyword;
};

export const keywordMatchAny = (token: string, keywords: string[]): boolean => {
	for(let i=0;i<keywords.length;i++){
		if(keywordMatch(token, keywords[i])){
			return true;
		}
	}
	return false;
};
