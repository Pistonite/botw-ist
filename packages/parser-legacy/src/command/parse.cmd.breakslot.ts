import type { ItemStackArg } from "./ItemStackArg";
import type { ASTCommandBreakSlots } from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTInteger } from "./parse.basis";
import { parseASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot } from "./parse.clause.with.fromslot";
import {
    type CodeBlock,
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParseItem,
    flattenCodeBlocks,
    type ParserItem,
} from "./type";

export class CommandBreakSlots extends AbstractProperCommand {
    private slot: number;
    private numToBreak: number;
    private stacks: ItemStackArg[];
    constructor(
        numToBreak: number,
        stacks: ItemStackArg[],
        slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.slot = slot - 1; // change to 0 based
        this.numToBreak = numToBreak;
        this.stacks = stacks;
    }

    public convert(): string {
        let s = "";
        if (this.stacks.length > 0) {
            s += `destroy ${this.stacks.map((s) => s.convert()).join(" ")}`;
            if (this.slot) {
                s += ` from slot ${this.slot + 1}`;
            }
            s += "; ";
        }
        const slotWord = this.numToBreak === 1 ? "slot" : "slots";
        s += `break ${this.numToBreak} ${slotWord};`;
        return s;
    }
}

export const parseASTCommandBreakSlots: ParserItem<
    ASTCommandBreakSlots,
    CommandBreakSlots
> = (ast, search) => {
    const codeBlocks: CodeBlock[] = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const [numberToBreak, numberBlocks] = parseASTInteger(ast.mInteger1);
    flattenCodeBlocks(codeBlocks, numberBlocks, "slot.number");
    codeBlocks.push(codeBlockFromRange(ast.mLiteralSlot2, "keyword.command"));
    return delegateParseItem(
        ast.mMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot3,
        search,
        parseASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot,
        ([stacks, slot], codeBlocks) =>
            new CommandBreakSlots(numberToBreak, stacks, slot, codeBlocks),
        codeBlocks,
    );
};
