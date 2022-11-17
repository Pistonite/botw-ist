import { MetaModifyOption } from "data/item";
import { ItemStackArg } from "./ItemStackArg";
import {
	ASTArgumentItemStacksAllowAllMaybeFromSlot,
	ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier,
	ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1,
	ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2,
	ASTArgumentItemStacksAllowAllMaybeFromSlotAMetadata,
	ASTArgumentItemStacksMaybeFromSlot,
	ASTArgumentItemStacksMaybeFromSlotAIdentifier,
	ASTArgumentItemStacksMaybeFromSlotAIdentifierC1,
	ASTArgumentItemStacksMaybeFromSlotAIdentifierC2,
	ASTArgumentItemStacksMaybeFromSlotAMetadata,
	ASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
	ASTArgumentOneOrMoreItemsMaybeFromSlot,
	ASTArgumentSingleItemAllowAllMaybeFromSlot,
	ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifier,
	ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1,
	ASTArgumentSingleItemMaybeFromSlot,
	ASTArgumentSingleItemMaybeFromSlotAIdentifier,
	ASTArgumentSingleItemMaybeFromSlotAIdentifierC1,
	ASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot,
	isArgumentItemStacksAllowAllMaybeFromSlot,
	isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1,
	isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2,
	isArgumentItemStacksMaybeFromSlotAIdentifierC1,
	isArgumentItemStacksMaybeFromSlotAIdentifierC2,
	isArgumentSingleItemAllowAllMaybeFromSlot,
	isArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1,
	isArgumentSingleItemMaybeFromSlot,
	isArgumentSingleItemMaybeFromSlotAIdentifierC1,
	isClauseFromSlot,
	isEpsilon
} from "./ast";
import { parseASTIdentifier } from "./parse.basis";
import { parseASTClauseSlot } from "./parse.clause.slot";
import { parseASTAmountOrAll, parsedItemSearch } from "./parse.item";
import { parseASTMetadata } from "./parse.metadata";
import { AmountAllType, codeBlockFromRange, CodeBlockTree, delegateParse, delegateParseItem, flattenCodeBlocks, Parser, ParserItem } from "./type";

export const parseASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot:
    ParserItem<ASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot, [ItemStackArg[], number]>
= (ast, search) => {
	if(isEpsilon(ast)){
		return [[[], 1/*default slot*/], [], ""];
	}
	const withBlock = codeBlockFromRange(ast.literal0, "keyword.other");
	const codeBlocks: CodeBlockTree = [withBlock];
	return delegateParseItem(
		ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
		search,
		parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
		x=>x,
		codeBlocks
	);
};

export const parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot:
    ParserItem<ASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot | ASTArgumentOneOrMoreItemsMaybeFromSlot, [ItemStackArg[], number]>
= (ast, search) => {
	if(isArgumentSingleItemAllowAllMaybeFromSlot(ast) || isArgumentSingleItemMaybeFromSlot(ast)){
		const [result, resultBlocks, resultError] = parseASTArgumentSingleItemAllowAllMaybeFromSlot(ast);
		if(!result){
			return [undefined, resultBlocks, resultError];
		}
		const [ids, meta, slot] = result;
		return delegateParseItem(
			[ids, resultBlocks, meta],
			search,
			parsedItemSearch,
			(stack)=>[[new ItemStackArg(stack, 1)], slot]
		);
	}
	const [result, resultBlocks, resultError] = parseASTArgumentItemStacksAllowAllMaybeFromSlot(ast);
	if(!result){
		return [undefined, resultBlocks, resultError];
	}
	const [tempItems, slot] = result;
	const stackArgs: ItemStackArg[] = [];
	for(let i=0;i<tempItems.length;i++){
		const [amount, ids, meta] = tempItems[i];
		if(amount === undefined){
			throw new Error("parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot: amount is undefined");
		}
		const [stack, _, itemError] = parsedItemSearch([ids, resultBlocks, meta], search);
		if(!stack){
			return [undefined, resultBlocks, itemError];
		}
		stackArgs.push(new ItemStackArg(stack, amount));
	}
	return [[stackArgs, slot], resultBlocks, ""];
};

const parseASTArgumentSingleItemAllowAllMaybeFromSlot:
    Parser<ASTArgumentSingleItemAllowAllMaybeFromSlot | ASTArgumentSingleItemMaybeFromSlot, [string[], MetaModifyOption, number]>
= (ast) => {
	const [ firstIdentifier, firstIdentifierBlocks ] = parseASTIdentifier(ast.mIdentifier0);

	const codeBlocks: CodeBlockTree = [flattenCodeBlocks([],firstIdentifierBlocks, "item.name")];
	
	return delegateParse(
		isArgumentSingleItemAllowAllMaybeFromSlot(ast)
			? ast.mArgumentSingleItemAllowAllMaybeFromSlotAIdentifier1
			: ast.mArgumentSingleItemMaybeFromSlotAIdentifier1,
		parseSingleItemAIdentifier,
		(result)=>{
			result[0].splice(0, 0, firstIdentifier);
			return result;
		},
		codeBlocks);
	
};

const parseSingleItemAIdentifier:
    Parser<ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifier | ASTArgumentSingleItemMaybeFromSlotAIdentifier, [string[], MetaModifyOption, number]>
= (ast) => {
	if(isEpsilon(ast)){
		return [[[], {}, 1 /*default slot*/], [], ""];
	}
	if(isClauseFromSlot(ast)){
		return delegateParse(
			ast,
			parseASTClauseSlot,
			(number) => [[], {}, number],
		);
	}
	if(isArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1(ast) || isArgumentSingleItemMaybeFromSlotAIdentifierC1(ast)){
		return delegateParse(ast, parseC1, (result)=>[[], ...result]);
	}
	return parseASTArgumentSingleItemAllowAllMaybeFromSlot(ast);
};

const parseC1: Parser<ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1 | ASTArgumentSingleItemMaybeFromSlotAIdentifierC1, [MetaModifyOption, number]> = (ast) => {
	const [meta, metaBlocks, metaError] = parseASTMetadata(ast.mMetadata0);
	if(!meta){
		return [undefined, metaBlocks, metaError];
	}
	if(isEpsilon(ast.mMaybeClauseFromSlot1)){
		return [[meta, 1 /*default slot*/], metaBlocks, ""];
	}
	return delegateParse(
		ast.mMaybeClauseFromSlot1,
		parseASTClauseSlot,
		(number)=>[meta, number],
		metaBlocks
	);
};

const parseASTArgumentItemStacksAllowAllMaybeFromSlot:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlot | ASTArgumentItemStacksMaybeFromSlot, [[(number|AmountAllType|undefined), string[], MetaModifyOption][], number]>
= (ast) => {
	const allowAll = isArgumentItemStacksAllowAllMaybeFromSlot(ast);
	const [amount, amountBlocks] = parseASTAmountOrAll(allowAll ? ast.mAmountOrAll0 : ast.mInteger0);
	const [firstId, idBlocks] = parseASTIdentifier(ast.mIdentifier1);
	const codeBlocks = [amountBlocks, flattenCodeBlocks([], idBlocks, "item.name")];
	return delegateParse(
		allowAll ? ast.mArgumentItemStacksAllowAllMaybeFromSlotAIdentifier2 : ast.mArgumentItemStacksMaybeFromSlotAIdentifier2,
		parseASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier,
		(result) => {
			const [tempItems]=result;
			if(tempItems.length === 0){
				// if amount and this id is the only item, return it
				tempItems.push([amount, [firstId], {}]);
				return result;
			}
			const [firstTempItem] = tempItems;
			if(firstTempItem[0] !== undefined){
				// If amount not undefined, this is the end of an item, create a new item
				tempItems.splice(0,0, [amount, [firstId], {}]);
			}else{
				// otherwise add id and amount
				firstTempItem[0] = amount;
				firstTempItem[1].splice(0,0,firstId);
			}

			return result;
		},
		codeBlocks
	);
};

const parseASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier | ASTArgumentItemStacksMaybeFromSlotAIdentifier, [[(number|AmountAllType|undefined), string[], MetaModifyOption][], number]>
= (ast) => {
	if(isEpsilon(ast)){
		return [[[], 1 /*default slot*/], [], ""];
	}
	if(isClauseFromSlot(ast)){
		return delegateParse(
			ast,
			parseASTClauseSlot,
			number=>[[], number]
		);
	}
	if(isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1(ast) || isArgumentItemStacksMaybeFromSlotAIdentifierC1(ast)){
		return parseItemStackC1(ast);
	}
	if(isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2(ast) || isArgumentItemStacksMaybeFromSlotAIdentifierC2(ast)){
		return parseItemStackC2(ast);
	}
	return parseASTArgumentItemStacksAllowAllMaybeFromSlot(ast);

};

const parseItemStackC1:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1 | ASTArgumentItemStacksMaybeFromSlotAIdentifierC1, [[(number|AmountAllType|undefined), string[], MetaModifyOption][], number]>
= (ast) => {
	const [meta, metaBlocks, metaError] = parseASTMetadata(ast.mMetadata0);
	if(!meta){
		return [undefined, metaBlocks, metaError];
	}
	return delegateParse(
		isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1(ast)
			? ast.mArgumentItemStacksAllowAllMaybeFromSlotAMetadata1
			: ast.mArgumentItemStacksMaybeFromSlotAMetadata1,
		parseItemStackAMetadata,
		(result)=>{
			// since this starts with meta, create a new (temp) item
			result[0].splice(0,0, [undefined, [], meta]);
			return result;
		},
		metaBlocks
	);
};

const parseItemStackC2:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2 | ASTArgumentItemStacksMaybeFromSlotAIdentifierC2, [[(number|AmountAllType|undefined), string[], MetaModifyOption][], number]>
= (ast) => {
	// this is middle of identifiers
	const [id, idBlocks] = parseASTIdentifier(ast.mIdentifier0);
	const codeBlocks = flattenCodeBlocks([], idBlocks, "item.name");
	return delegateParse(
		isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2(ast)
			? ast.mArgumentItemStacksAllowAllMaybeFromSlotAIdentifier1
			: ast.mArgumentItemStacksMaybeFromSlotAIdentifier1,
		parseASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier,
		(result)=>{
			// We want to add id to the front of current ids
			const [tempItems] = result;
			if(tempItems.length === 0){
				// this is the last item, create
				tempItems.push([undefined, [id], {}]);
				return result;
			}
			const firstTempItem = tempItems[0];
			const [tempAmount,tempIds] = firstTempItem;
			if(tempAmount !== undefined){
				// If amount not undefined, this is the end of an item, create a new item
				tempItems.splice(0,0, [undefined, [id], {}]);
			}else{
				// otherwise add id to the list of ids for current item
				tempIds.splice(0,0, id);
			}

			return result;
		},
		codeBlocks
	);
};

const parseItemStackAMetadata:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAMetadata | ASTArgumentItemStacksMaybeFromSlotAMetadata, [[(number|AmountAllType|undefined), string[], MetaModifyOption][], number]>
= (ast) => {
	if(isEpsilon(ast)){
		return [[[], 1 /*default slot*/], [], ""];
	}
	if(isClauseFromSlot(ast)){
		return delegateParse(
			ast,
			parseASTClauseSlot,
			number=>[[], number]
		);

	}
	return parseASTArgumentItemStacksAllowAllMaybeFromSlot(ast);
};
