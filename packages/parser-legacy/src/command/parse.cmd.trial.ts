import type { ASTCommandEnterTrial, ASTCommandExitTrial } from "./ast";
import { AbstractProperCommand } from "./command";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    type ParserSafe,
} from "./type";

export class CommandTrial extends AbstractProperCommand {
    private enter: boolean;
    constructor(enter: boolean, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.enter = enter;
    }

    public override convert(): string {
        if (this.enter) {
            return "leave eventide";
        }
        return "enter eventide";
    }
}

export const parseASTCommandEnterTrial: ParserSafe<
    ASTCommandEnterTrial,
    CommandTrial
> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.literal0, "keyword.command"),
        codeBlockFromRange(ast.mLiteralTrial1, "keyword.command"),
    ];
    return [new CommandTrial(true, codeBlocks), codeBlocks];
};

export const parseASTCommandExitTrial: ParserSafe<
    ASTCommandExitTrial,
    CommandTrial
> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.mLiteralLeave0, "keyword.command"),
        codeBlockFromRange(ast.mLiteralTrial1, "keyword.command"),
    ];
    return [new CommandTrial(false, codeBlocks), codeBlocks];
};
