import { SimulationState } from "core/SimulationState";
import { arrayEqual } from "data/util";
import { ASTCommandAdd, ASTCommandInitialize, ASTCommandPickUp } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { getSlotsToAdd, ItemStackArg } from "./ItemStackArg";
import { parseASTItems } from "./parse.item";
import { codeBlockFromRange, CodeBlockTree, delegateParseItem, ParserItem } from "./type";

export class CommandAdd extends AbstractProperCommand {
	private stacks: ItemStackArg[];
	constructor(stacks: ItemStackArg[], codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.stacks = stacks;
	}

	public execute(state: SimulationState): void {
		getSlotsToAdd(this.stacks).forEach(stack=>state.obtain(stack));
	}

	public equals(other: Command): boolean {
		return other instanceof CommandAdd && arrayEqual(this.stacks, other.stacks);
	}
}

export const parseASTCommandAdd: ParserItem<ASTCommandAdd, CommandAdd> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralAdd0, "keyword.command"));
    return delegateParseItem(ast.mOneOrMoreItems1, search, parseASTItems, (i,c)=>new CommandAdd(i,c), codeBlocks);
}

export const parseASTCommandPickup: ParserItem<ASTCommandPickUp, CommandAdd> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralPickUp0, "keyword.command"));
    return delegateParseItem(ast.mOneOrMoreItems1, search, parseASTItems, (i,c)=>new CommandAdd(i,c), codeBlocks);
}
