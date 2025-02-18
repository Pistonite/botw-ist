import { SimulationState } from "core/SimulationState";
import { arrayEqual } from "data/util";
import { getSlotsToAdd, ItemStackArg } from "./ItemStackArg";
import { ASTCommandInitGameData } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTItems } from "./parse.item";
import {
    codeBlockFromRange,
    CodeBlockTree,
    delegateParseItem,
    ParserItem,
} from "./type";

export class CommandInitGameData extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    constructor(stacks: ItemStackArg[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.stacks = stacks;
    }

    public execute(state: SimulationState): void {
        state.setGameData(getSlotsToAdd(this.stacks));
    }

    public equals(other: Command): boolean {
        return (
            other instanceof CommandInitGameData &&
            arrayEqual(this.stacks, other.stacks)
        );
    }
}

export const parseASTCommandInitGamedata: ParserItem<
    ASTCommandInitGameData,
    CommandInitGameData
> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(
        codeBlockFromRange(ast.mLiteralInitialize0, "keyword.command"),
    );
    codeBlocks.push(codeBlockFromRange(ast.literal1, "keyword.command"));
    return delegateParseItem(
        ast.mZeroOrMoreItems2,
        search,
        parseASTItems,
        (i, c) => new CommandInitGameData(i, c),
        codeBlocks,
    );
};
