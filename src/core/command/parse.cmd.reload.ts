import { SimulationState } from "core/SimulationState";
import { ASTCommandReload } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTIdentifierPrime } from "./parse.basis";
import { codeBlockFromRange, CodeBlockTree, flattenCodeBlocks, Parser } from "./type";

export class CommandReload extends AbstractProperCommand {
	private name: string | undefined;
	constructor(name: string | undefined, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.name = name;
	}
	public execute(state: SimulationState): void {
		state.reload(this.name);
	}
    public equals(other: Command): boolean {
        return other instanceof CommandReload && this.name === other.name;
    }
}

export const parseASTCommandReload: Parser<ASTCommandReload, CommandReload> = (ast) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const [ids, idBlocks, idError] = parseASTIdentifierPrime(ast.mIdentifierPrime1);
    codeBlocks.push(flattenCodeBlocks([], idBlocks, "identifier.other"));
    if(!ids){
        return [undefined, codeBlocks, idError];
    }
    let fileName: string | undefined = ids.join(" ");
    if(!fileName){
        fileName = undefined;
    }
    return [ new CommandReload(fileName, codeBlocks), codeBlocks, ""];
}
