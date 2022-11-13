import { SimulationState } from "core/SimulationState";
import { ItemType } from "data/item";
import { arrayEqual, arrayShallowEqual } from "data/util";
import { getSlotsToAdd, ItemStackArg } from "./ItemStackArg";
import { ASTCommandDnp, ASTCommandDrop, ASTCommandEat, ASTCommandRemove, ASTCommandRemoveAll } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTItemType } from "./parse.basis";
import { parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot } from "./parse.clause.with.fromslot";
import { codeBlockFromRange, CodeBlockTree, delegateParseItem, delegateParseSafe, ParserItem, ParserSafe } from "./type";

// Remove, Sell, With, Drop
export class CommandRemove extends AbstractProperCommand  {
	private stacks: ItemStackArg[];
	private slot: number;
	constructor(stacks: ItemStackArg[], slot: number, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.stacks = stacks;
		this.slot = slot-1;//change to 0 based
	}
	public execute(state: SimulationState): void {
        this.stacks.forEach(stackArg=>state.remove(stackArg.stack, stackArg.number, this.slot));
	}
    public equals(other: Command): boolean {
        return other instanceof CommandRemove && arrayEqual(this.stacks, other.stacks) && this.slot === other.slot;
    }
}

// Eat (deletes arrow slots)
export class CommandEat extends AbstractProperCommand  {
	private stacks: ItemStackArg[];
	private slot: number;
	constructor(stacks: ItemStackArg[], slot: number, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.stacks = stacks;
		this.slot = slot-1;//change to 0 based
	}
	public execute(state: SimulationState): void {
        this.stacks.forEach(stackArg=>state.eat(stackArg.stack, stackArg.number, this.slot));
	}
    public equals(other: Command): boolean {
        return other instanceof CommandEat && arrayEqual(this.stacks, other.stacks) && this.slot === other.slot;
    }
}

// Remove all type
export class CommandRemoveAll extends AbstractProperCommand  {
	private types: ItemType[];
	constructor(types: ItemType[], codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.types = types;
	}
	public execute(state: SimulationState): void {
        state.removeAll(this.types);
	}
    public equals(other: Command): boolean {
        return other instanceof CommandRemoveAll && arrayShallowEqual(this.types, other.types);
    }
}

export class CommandDnp extends AbstractProperCommand  {
	private stacks: ItemStackArg[];
	private slot: number;
	constructor(stacks: ItemStackArg[], slot: number, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.stacks = stacks;
		this.slot = slot-1;//change to 0 based
	}
	public execute(state: SimulationState): void {
		this.stacks.forEach(stackArg =>{
			const removedCount = state.remove(stackArg.stack, stackArg.number, this.slot);
			if(removedCount > 0){
				getSlotsToAdd([new ItemStackArg(stackArg.stack, removedCount)]).forEach(stack=>{
					state.obtain(stack);
				});
			}
		})
	}
    public equals(other: Command): boolean {
        return other instanceof CommandDnp && arrayEqual(this.stacks, other.stacks) && this.slot === other.slot;
    }
}

export const parseASTCommandRemove: ParserItem<ASTCommandRemove, CommandRemove> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralRemove0, "keyword.command"));
    return delegateParseItem(
		ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
		search,
		parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
		(i,c)=>new CommandRemove(...i,c),
		codeBlocks
	);
}
// TODO: drop to ground
export const parseASTCommandDrop: ParserItem<ASTCommandDrop, CommandRemove> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
		ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
		search,
		parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
		(i,c)=>new CommandRemove(...i,c),
		codeBlocks
	);
}

export const parseASTCommandEat: ParserItem<ASTCommandEat, CommandEat> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
		ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
		search,
		parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
		(i,c)=>new CommandEat(...i,c),
		codeBlocks
	);
}

export const parseASTCommandRemoveAll: ParserSafe<ASTCommandRemoveAll, CommandRemoveAll> = (ast) => {
	const codeBlocks: CodeBlockTree = [
		codeBlockFromRange(ast.literal0, "keyword.command"),
		codeBlockFromRange(ast.literal1, "item.type")
	];
	return delegateParseSafe(
		ast.mLiteralItemType2,
		parseASTItemType,
		(itemTypes, c) => new CommandRemoveAll(itemTypes, c),
		codeBlocks
	);
}

export const parseASTCommandDnp: ParserItem<ASTCommandDnp, CommandDnp> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralDnp0, "keyword.command"));
    return delegateParseItem(
		ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
		search,
		parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
		(i,c)=>new CommandDnp(...i,c),
		codeBlocks
	);
}
