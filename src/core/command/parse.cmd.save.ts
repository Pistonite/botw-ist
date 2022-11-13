import { SimulationState } from "core/SimulationState";
import { ASTCommandSave, ASTMaybeClauseSaveTarget, isEpsilon } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTOneOrMoreIdentifiers } from "./parse.basis";
import { codeBlockFromRange, CodeBlockTree, delegateParse, flattenCodeBlocks, Parser } from "./type";

export class CommandSave extends AbstractProperCommand {
	private name: string | undefined;
	constructor(name: string | undefined, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		if(!name){
			this.name = undefined;
		}else{
			this.name = name;
		}

	}
	public execute(state: SimulationState): void {
		state.save(this.name);
	}
	public equals(other: Command): boolean {
		return other instanceof CommandSave && this.name === other.name;
	}
}

export const parseASTCommandSave: Parser<ASTCommandSave, CommandSave> = (ast) => {
	const codeBlocks: CodeBlockTree = [];
	codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
	return delegateParse(ast.mMaybeClauseSaveTarget1, parseASTSaveTarget, (target,c)=>new CommandSave(target,c), codeBlocks);
};

const parseASTSaveTarget: Parser<ASTMaybeClauseSaveTarget, string> = (ast) => {
	if(isEpsilon(ast)){
		return ["", [], ""];
	}
	const codeBlocks: CodeBlockTree = [
		codeBlockFromRange(ast.literal0, "keyword.command"),
	];
	const [ids, idCodeBlocks, idError] = parseASTOneOrMoreIdentifiers(ast.mOneOrMoreIdentifiers1);
	codeBlocks.push(flattenCodeBlocks([], idCodeBlocks, "identifier.other"));
	if(!ids){
		return [undefined, codeBlocks, idError];
	}
	const saveTarget = ids.join(" ");
	return [saveTarget, codeBlocks, ""];

};
