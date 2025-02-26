import type { ASTCommandSyncGameData } from "./ast";
import { AbstractProperCommand } from "./command";
import { codeBlockFromRange, type Parser } from "./type";

export class CommandSync extends AbstractProperCommand {
    public convert(): string {
        return "sync;";
    }
}

export const parseASTCommandSyncGameData: Parser<
    ASTCommandSyncGameData,
    CommandSync
> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.literal0, "keyword.command"),
        codeBlockFromRange(ast.literal1, "keyword.command"),
    ];
    return [new CommandSync(codeBlocks), codeBlocks, ""];
};
