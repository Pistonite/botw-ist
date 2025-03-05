import type { ASTCommandCloseGame } from "./ast";
import { AbstractProperCommand } from "./command";
import { codeBlockFromRange, type ParserSafe } from "./type";

export class CommandCloseGame extends AbstractProperCommand {
    public override convert(): string {
        return "close-game;";
    }
}

export const parseASTCommandCloseGame: ParserSafe<
    ASTCommandCloseGame,
    CommandCloseGame
> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.mLiteralClose0, "keyword.command"),
        codeBlockFromRange(ast.literal1, "keyword.command"),
    ];
    return [new CommandCloseGame(codeBlocks), codeBlocks];
};
