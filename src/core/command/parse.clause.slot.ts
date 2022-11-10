import { ASTClauseFromSlot } from "./ast";
import { codeBlockFromRange, ParserSafe } from "./type";

export const parseASTClauseSlot: ParserSafe<ASTClauseFromSlot, number> = (ast) => {
    return [
        ast.mInteger2.value,
        [
            codeBlockFromRange(ast.literal0, "keyword.other"),
            codeBlockFromRange(ast.mLiteralSlot1, "keyword.other"),
            codeBlockFromRange(ast.mInteger2, "slot.number")
        ]
    ];
}
