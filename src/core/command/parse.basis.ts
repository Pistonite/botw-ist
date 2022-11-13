import { ItemType } from "data/item";
import { ASTOneOrMoreIdentifiers, ASTIdentifier, ASTInteger, isOneOrMoreIdentifiers, ASTIdentifierPrime, ASTLiteralItemType, isLiteralKeyItem, isLiteralFood, isLiteralMaterial, isLiteralArmor, isLiteralShield, isLiteralArrow, isLiteralBow } from "./ast";
import { codeBlockFromRange, CodeBlockTree, flattenCodeBlocks, Parser, ParserSafe } from "./type";

export const parseASTInteger: ParserSafe<ASTInteger, number> = (ast) => {
	return [
		ast.value,
		[codeBlockFromRange(ast, "unknown")]// cannot decide the color at this level
	];
};

export const parseASTIdentifier: ParserSafe<ASTIdentifier, string> = (ast) => {
	return [
		ast.value,
		[codeBlockFromRange(ast, "unknown")]// cannot decide the color at this level
	];
};

const MaxIdentifierDepth = 500;

export const parseASTOneOrMoreIdentifiers: Parser<ASTOneOrMoreIdentifiers, string[]> = (ast) => {
	const [ids, codeBlocks, idError] = parseASTIdentifierPrime(ast.mIdentifierPrime1);
	if(!ids){
		return [undefined, codeBlocks, idError];
	}
	const [first, firstCodeBlock] = parseASTIdentifier(ast.mIdentifier0);
	codeBlocks.splice(0,0,firstCodeBlock);
	ids.splice(0,0,first);
	return [ids, codeBlocks, ""];
};

export const parseASTIdentifierPrime: Parser<ASTIdentifierPrime, string[]> = (ast) => {
	const ids: string[] = [];
	const codeBlocks: CodeBlockTree = [];
	let current = ast;
	let depth = 0;
	while(isOneOrMoreIdentifiers(current)){
		if(depth > MaxIdentifierDepth){
			return [undefined, codeBlocks, "Max Identifier Depth Exceeded"];
		}
		const [currentId, currentCodeBlock] = parseASTIdentifier(current.mIdentifier0);
		ids.push(currentId);
		codeBlocks.push(currentCodeBlock);
		current = current.mIdentifierPrime1;
		depth++;
	}
	return [ids, flattenCodeBlocks([], codeBlocks, "unknown"), ""];
};

export const parseASTItemType: ParserSafe<ASTLiteralItemType, ItemType[]> = (ast) => {
	if(isLiteralKeyItem(ast)){
		return [
			[ItemType.Key],
			[
				codeBlockFromRange(ast.literal0, "item.type"),
				codeBlockFromRange(ast.mLiteralItem1, "item.type")
			]
		];
	}
	if(isLiteralArmor(ast)){
		return [[ItemType.ArmorLower, ItemType.ArmorMiddle, ItemType.ArmorUpper], [codeBlockFromRange(ast, "item.type")]];
	}
	let type = ItemType.Weapon;
	if(isLiteralFood(ast)){
		type = ItemType.Food;
	}
	if(isLiteralMaterial(ast)){
		type = ItemType.Material;
	}
	if(isLiteralShield(ast)){
		type = ItemType.Shield;
	}
	if(isLiteralArrow(ast)){
		type = ItemType.Arrow;
	}
	if(isLiteralBow(ast)){
		type = ItemType.Bow;
	}

	return [[type], [codeBlockFromRange(ast, "item.type")]];
};
