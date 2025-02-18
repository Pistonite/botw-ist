import { ASTClauseFromSlot, ASTClauseInSlot } from "./ast";
import { codeBlockFromRange, Parser } from "./type";

export const parseASTClauseSlot: Parser<
    ASTClauseFromSlot | ASTClauseInSlot,
    number
> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.literal0, "keyword.other"),
        codeBlockFromRange(ast.mLiteralSlot1, "keyword.other"),
        codeBlockFromRange(ast.mInteger2, "slot.number"),
    ];
    if (ast.mInteger2.value < 1) {
        return [undefined, codeBlocks, "Slot number must be > 1"];
    }
    return [ast.mInteger2.value, codeBlocks, ""];
};
