import { SimulationState } from "core/SimulationState";
import { ASTCommandShoot } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTAmountOrAll } from "./parse.item";
import { codeBlockFromRange, CodeBlockTree, Parser, AmountAllType } from "./type";

export class CommandShootArrow extends AbstractProperCommand  {
	private count: number | AmountAllType;
	constructor(count: number | AmountAllType, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.count = count;
	}

	public execute(state: SimulationState): void {
		state.shootArrow(this.count);
	}
	public equals(other: Command): boolean {
        return other instanceof CommandShootArrow && this.count === other.count;
    }
}

export const parseASTCommandShoot: Parser<ASTCommandShoot, CommandShootArrow> = (ast) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const [amount, amountBlocks] = parseASTAmountOrAll(ast.mAmountOrAll1);
    codeBlocks.push(amountBlocks);
    codeBlocks.push(codeBlockFromRange(ast.mLiteralArrow2, "keyword.command"));
    if(amount <= 0){
        return [undefined, codeBlocks, "Must shoot at least 1 arrow"];
    }
    return [new CommandShootArrow(amount, codeBlocks), codeBlocks, ""];
}
