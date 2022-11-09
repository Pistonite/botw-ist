import { SimulationState } from "core/SimulationState";
import { arrayEqual } from "data/util/util";
import { ASTCommandInitGameData } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { processWrappers } from "./helper";
import { ItemStackArg } from "./ItemStackArg";
import { parseASTItems } from "./parse.item";
import { codeBlockFromRange, codeBlockFromRangeObj, CodeBlockTree, delegateParseItem, ParserItem } from "./type";


export class CommandInitGameData extends AbstractProperCommand {
	private stacks: ItemStackArg[];
	constructor(stacks: ItemStackArg[], codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.stacks = stacks;
	}

	public execute(state: SimulationState): void {
		state.setGameData(processWrappers(this.stacks));
	}

	public equals(other: Command): boolean {
		return other instanceof CommandInitGameData && arrayEqual(this.stacks, other.stacks);
	}

}

export const parseASTCommandInitGamedata: ParserItem<ASTCommandInitGameData, CommandInitGameData> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRangeObj(ast.mLiteralInitialize0, "keyword.command"));
	codeBlocks.push(codeBlockFromRange(ast.literal1, "keyword.command"));
    return delegateParseItem(ast.mZeroOrMoreItems2, search, parseASTItems, (i,c)=>new CommandInitGameData(i,c), codeBlocks);
}
