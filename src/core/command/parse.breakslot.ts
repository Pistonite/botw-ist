import { SimulationState } from "core/SimulationState";
import { arrayEqual } from "data/util/util";
import { ASTCommandBreakSlots } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { ItemStackArg } from "./ItemStackArg";
import { parseASTInteger } from "./parse.basis";
import { parseASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot } from "./parse.clause.with.fromslot";
import { CodeBlock, codeBlockFromRange, codeBlockFromRangeObj, CodeBlockTree, delegateParseItem, flattenCodeBlocks, Parser, ParserItem, ParserSafe } from "./type";

export class CommandBreakSlots extends AbstractProperCommand {
    private slot: number;
	private numToBreak: number;
    private stacks: ItemStackArg[];
	constructor(numToBreak: number, stacks: ItemStackArg[], slot: number, codeBlocks: CodeBlockTree){
		super(codeBlocks);
        this.slot = slot-1;// change to 0 based
		this.numToBreak = numToBreak;
        this.stacks = stacks;
	}

	public execute(state: SimulationState): void {
        this.stacks.forEach(stackArg=>state.remove(stackArg.stack, stackArg.number, this.slot));

		state.breakSlots(this.numToBreak);
	}

    public equals(other: Command): boolean {
        return other instanceof CommandBreakSlots && this.numToBreak === other.numToBreak && arrayEqual(this.stacks, other.stacks) && this.slot === other.slot;
    }
}

export const parseASTCommandBreakSlots: ParserItem<ASTCommandBreakSlots, CommandBreakSlots> = (ast, search) => {
    const codeBlocks: CodeBlock[] = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const [numberToBreak, numberBlocks] = parseASTInteger(ast.mInteger1);
    flattenCodeBlocks(codeBlocks, numberBlocks, "slot.number");
	codeBlocks.push(codeBlockFromRangeObj(ast.mLiteralSlot2, "keyword.command"));
    return delegateParseItem(
        ast.mMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot3, 
        search, 
        parseASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot,
        ([stacks, slot],codeBlocks)=>new CommandBreakSlots(numberToBreak, stacks, slot ,codeBlocks), 
        codeBlocks
    );

}
