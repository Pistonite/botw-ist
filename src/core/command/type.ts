import { SimulationState } from "core/SimulationState";
import { ItemStack } from "data/item";

export const Colors = {
	"unknown": "CommandColorUnknown",
	"meta.key": "CommandColorMetaKey",
	"meta.value": "CommandColorMetaValue",
	"meta.const": "CommandColorMetaConst",
	"delimiter": "CommandColorDelimiter",
	"item.name": "CommandColorItemName",
	"item.amount": "CommandColorItemAmount",
	"keyword.command": "CommandColorKeywordCommand",
	"slot.number": "CommandColorSlotNumber",
	"keyword.other": "CommandColorKeywordOther",
	"identifier.other": "CommandColorIdentifierOther"

};

export type ItemSearchFunction = (word: string)=>ItemStack|undefined;

export type CodeBlock = {
	color: keyof typeof Colors,
	start: number, //inclusive
	end: number // exclusive
};

export const codeBlockFromRange = (range: readonly [number, number] | {range: readonly [number, number]}, color: keyof typeof Colors): CodeBlock => {
	if("range" in range){
		return codeBlockFromRange(range.range, color);
	}
	return {
		color,
		start: range[0],
		end: range[1]
	};
}


export type CodeBlockTree = (CodeBlock | CodeBlockTree)[]
const firstCodeBlock = (blocks: CodeBlockTree): CodeBlock => {
	let current = blocks[0];
	while(Array.isArray(current)){
		current = current[0];
	}
	return current;
}

const lastCodeBlock = (blocks: CodeBlockTree): CodeBlock => {
	let current = blocks[blocks.length-1];
	while(Array.isArray(current)){
		current = current[current.length-1];
	}
	return current;
}

export const flattenCodeBlocks = (output: CodeBlock[], blocks: CodeBlockTree, color?: keyof typeof Colors):CodeBlock[] => {
	if(color){
		output.push({
			color,
			start: firstCodeBlock(blocks).start,
			end: lastCodeBlock(blocks).end
		});
		return output;
	}

	blocks.forEach(blockOrBlocks=>{
		if(Array.isArray(blockOrBlocks)){
			flattenCodeBlocks(output, blockOrBlocks);
		}else{
			output.push(blockOrBlocks);
		}
	});
	return output;
}

// Function to parse AST
export type Parser<A, T> = (ast: A) => [T | undefined, CodeBlockTree, string];
export type ParserSafe<A, T> = (ast: A) => [T, CodeBlockTree];
export type ParserItem<A, T> = (ast: A, searchFunc: ItemSearchFunction) => [T | undefined, CodeBlockTree, string];

export const withNoError = <T>(result: (T|CodeBlockTree|string)[]): [T, CodeBlockTree, string] => {
	result.push("");
	return result as [T, CodeBlockTree, string];
}

export const delegateParse = <A, T, T2>(
	ast: A, 
	f: Parser<A, T>, 
	transformer: (t: T, c: CodeBlockTree) => T2, 
	codeBlocks?: CodeBlockTree
): [T2 | undefined, CodeBlockTree, string] => {
	const result: [T|T2|undefined, CodeBlockTree, string] = f(ast);
	if(codeBlocks){
		codeBlocks.push(result[1]);
		result[1] = codeBlocks;
	}
	if(result[0] === undefined) {
		return result as [undefined, CodeBlockTree, string];
	}
	//in place replace
	result[0] = transformer(result[0] as T, result[1]);
	
	return result as [T2, CodeBlockTree, string];
}

export const delegateParseSafe = <A, T, T2>(
	ast: A, 
	f: ParserSafe<A, T>, 
	transformer: (t: T) => T2, 
	codeBlocks?: CodeBlockTree
): [T2, CodeBlockTree] => {
	const result: [T|T2, CodeBlockTree] = f(ast);
	//in place replace
	result[0] = transformer(result[0] as T);
	if(codeBlocks){
		codeBlocks.push(result[1]);
		result[1] = codeBlocks;
	}
	return result as [T2, CodeBlockTree];
}

export const delegateParseItem = <A, T, T2>(
	ast: A, 
	searchFunc: ItemSearchFunction, 
	f: ParserItem<A, T>, 
	transformer: (t: T, c: CodeBlockTree) => T2, 
	codeBlocks?: CodeBlockTree
): [T2 | undefined, CodeBlockTree, string] => {
	const result: [T|T2|undefined, CodeBlockTree, string] = f(ast, searchFunc);
	//in place replace
	if(codeBlocks){
		codeBlocks.push(result[1]);
		result[1] = codeBlocks;
	}
	if(result[0] === undefined) {
		return result as [undefined, CodeBlockTree, string];
	}
	
	result[0] = transformer(result[0] as T, result[1]);
	return result as [T2, CodeBlockTree, string];
}


