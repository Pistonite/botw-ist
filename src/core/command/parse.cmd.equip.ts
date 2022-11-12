import { SimulationState } from "core/SimulationState";
import { Item, ItemType } from "data/item";
import { arrayShallowEqual } from "data/util";
import { ASTCommandEquip, ASTCommandUnequip, ASTCommandUnequipAll, isEpsilon, isLiteralAll } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTItemType } from "./parse.basis";
import { parseASTArgumentSingleItemMaybeInSlot } from "./parse.clause.inslot";
import { codeBlockFromRange, CodeBlockTree, delegateParseItem, delegateParseSafe, ParserItem, ParserSafe } from "./type";

export class CommandEquip extends AbstractProperCommand  {
	private item: Item;
	private slot: number;
	constructor(item: Item, slot: number, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.item = item;
		this.slot = slot-1;// change to 0-based
	}

	public execute(state: SimulationState): void {
		state.equip(this.item, this.slot);
	}
    public equals(other: Command): boolean {
        return other instanceof CommandEquip && other.item === this.item && this.slot === other.slot;
    }
}

export class CommandUnequip extends AbstractProperCommand {
	private item: Item;
	private slot: number;
	constructor(item: Item, slot: number, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.item = item;
		this.slot = slot-1;// change to 0-based
	}

	public execute(state: SimulationState): void {
		state.unequip(this.item, this.slot);
	}
    public equals(other: Command): boolean {
        return other instanceof CommandUnequip && other.item === this.item && this.slot === other.slot;
    }
}

export class CommandUnequipAll extends AbstractProperCommand  {
	private types: ItemType[];
	constructor(types: ItemType[], codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.types = types;
	}
	public execute(state: SimulationState): void {
        state.unequipAll(this.types);
	}
    public equals(other: Command): boolean {
        return other instanceof CommandUnequipAll && arrayShallowEqual(this.types, other.types);
    }
}

export const parseASTCommandEquip: ParserItem<ASTCommandEquip, CommandEquip> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentSingleItemMaybeInSlot1, 
        search,
        parseASTArgumentSingleItemMaybeInSlot, 
        ([stack, slot],c)=>new CommandEquip(stack.item, slot,c), 
        codeBlocks
    );
}

export const parseASTCommandUnequip: ParserItem<ASTCommandUnequip, CommandUnequip> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentSingleItemMaybeInSlot1, 
        search,
        parseASTArgumentSingleItemMaybeInSlot, 
        ([stack, slot],c)=>new CommandUnequip(stack.item, slot,c), 
        codeBlocks
    );
}

export const parseASTCommandUnequipAll: ParserSafe<ASTCommandUnequipAll, CommandUnequipAll> = (ast) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const argItemType = ast.mLiteralMaybeAllItemType1;
    const astAll = argItemType.mLiteralMaybeAll0;
    if(isLiteralAll(astAll)){
        codeBlocks.push(codeBlockFromRange(astAll.literal0, "item.type"));
    }
    const astItemType = argItemType.mLiteralItemType1;
    return delegateParseSafe(
		astItemType,
		parseASTItemType,
		(itemTypes, c) => new CommandUnequipAll(itemTypes, c),
		codeBlocks
	);

}
