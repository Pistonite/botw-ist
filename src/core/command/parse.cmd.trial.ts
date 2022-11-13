import { SimulationState } from "core/SimulationState";
import { ASTCommandEnterTrial, ASTCommandExitTrial } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { codeBlockFromRange, CodeBlockTree, ParserSafe } from "./type";

export class CommandTrial extends AbstractProperCommand  {
	private enter: boolean;
	constructor(enter: boolean, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.enter = enter;
	}

	public execute(state: SimulationState): void {
		state.setEventide(this.enter);
	}
    public equals(other: Command): boolean {
        return other instanceof CommandTrial && this.enter === other.enter;
    }
}

export const parseASTCommandEnterTrial: ParserSafe<ASTCommandEnterTrial, CommandTrial> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.literal0, "keyword.command"),
        codeBlockFromRange(ast.mLiteralTrial1, "keyword.command")
    ];
    return [new CommandTrial(true, codeBlocks), codeBlocks];
}

export const parseASTCommandExitTrial: ParserSafe<ASTCommandExitTrial, CommandTrial> = (ast) => {
    const codeBlocks = [
        codeBlockFromRange(ast.mLiteralLeave0, "keyword.command"),
        codeBlockFromRange(ast.mLiteralTrial1, "keyword.command")
    ];
    return [new CommandTrial(false, codeBlocks), codeBlocks];
}
