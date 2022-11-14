import { SimulationState } from "core/SimulationState";
import { ASTSuperCommandSwap } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTInteger } from "./parse.basis";
import { codeBlockFromRange, CodeBlockTree, flattenCodeBlocks, ParserSafe } from "./type";

export class SuperCommandSwap extends AbstractProperCommand {
    private i: number;
    private j: number;
    constructor(i: number, j: number, codeBlocks: CodeBlockTree){
        super(codeBlocks);
        this.i = i;
        this.j = j;
    }

    public execute(state: SimulationState): void {
        state.swap(this.i, this.j);
    }

    public equals(other: Command): boolean {
        return other instanceof SuperCommandSwap && this.i === other.i && this.j === other.j;
    }
}

export const parseASTSuperCommandSwap: ParserSafe<ASTSuperCommandSwap, SuperCommandSwap> = (ast) => {
    const codeBlocks: CodeBlockTree = [
        codeBlockFromRange(ast.literal0, "keyword.super"),
        codeBlockFromRange(ast.literal1, "keyword.super"),
    ];
    const [i, iBlock] = parseASTInteger(ast.mInteger2);
    const [j, jBlock] = parseASTInteger(ast.mInteger3);

    codeBlocks.push(flattenCodeBlocks([], [iBlock, jBlock], "slot.number"));

    return [new SuperCommandSwap(i, j, codeBlocks), codeBlocks];
}
