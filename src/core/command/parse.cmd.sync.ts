import { SimulationState } from "core/SimulationState";
import { ASTCommandSyncGameData } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { codeBlockFromRange, Parser } from "./type";

export class CommandSync extends AbstractProperCommand  {

	public execute(state: SimulationState): void {
		state.syncGameDataWithPouch();
	}

	public equals(other: Command): boolean {
		return other instanceof CommandSync;
	}

}

export const parseASTCommandSyncGameData: Parser<ASTCommandSyncGameData, CommandSync> = (ast)=>{
	const codeBlocks = [
		codeBlockFromRange(ast.literal0, "keyword.command"),
		codeBlockFromRange(ast.literal1, "keyword.command")
	];
	return [
		new CommandSync(codeBlocks),
		codeBlocks,
		""
	];
};
