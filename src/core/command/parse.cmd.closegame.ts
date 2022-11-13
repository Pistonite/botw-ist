import { SimulationState } from "core/SimulationState";
import { ASTCommandCloseGame } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { codeBlockFromRange, ParserSafe } from "./type";

export class CommandCloseGame extends AbstractProperCommand  {
	public execute(state: SimulationState): void {
		state.closeGame();
	}
	public equals(other: Command): boolean {
        return other instanceof CommandCloseGame;
    }
}

export const parseASTCommandCloseGame: ParserSafe<ASTCommandCloseGame, CommandCloseGame> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.mLiteralClose0, "keyword.command"),
        codeBlockFromRange(ast.literal1, "keyword.command")
    ];
    return [new CommandCloseGame(codeBlocks), codeBlocks];
}