import type { ASTCommandShoot } from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTAmountOrAll } from "./parse.item";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    type Parser,
    type AmountAllType,
} from "./type";

export class CommandShootArrow extends AbstractProperCommand {
    private count: number | AmountAllType;
    constructor(count: number | AmountAllType, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.count = count;
    }

    public override convert(): string {
        if (this.count === "All") {
            return "### `Shoot All` is no longer supported. Please specify number of times!\n### shoot X times;";
        }
        const timeWord = this.count === 1 ? "time" : "times";
        return `shoot ${this.count} ${timeWord};`;
    }
}

export const parseASTCommandShoot: Parser<
    ASTCommandShoot,
    CommandShootArrow
> = (ast) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const [amount, amountBlocks] = parseASTAmountOrAll(ast.mAmountOrAll1);
    codeBlocks.push(amountBlocks);
    codeBlocks.push(codeBlockFromRange(ast.mLiteralArrow2, "keyword.command"));
    if (typeof amount != "string" && amount <= 0) {
        return [undefined, codeBlocks, "Must shoot at least 1 arrow"];
    }
    return [new CommandShootArrow(amount, codeBlocks), codeBlocks, ""];
};
