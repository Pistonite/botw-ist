import { ASTClauseFromSlot } from "./ast";
import { codeBlockFromRange, codeBlockFromRangeObj, Parser, ParserSafe } from "./type";

export const parseASTClauseSlot: ParserSafe<ASTClauseFromSlot, number> = (ast) => {
    return [
        ast.mInteger2.value,
        [
            codeBlockFromRange(ast.literal0, "keyword.other"),
            codeBlockFromRangeObj(ast.mLiteralSlot1, "keyword.other"),
            codeBlockFromRangeObj(ast.mInteger2, "slot.number")
        ]
    ];
}
