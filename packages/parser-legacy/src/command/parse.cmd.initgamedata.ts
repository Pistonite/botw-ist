import type { ItemStackArg } from "./ItemStackArg";
import type { ASTCommandInitGameData } from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTItems } from "./parse.item";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParseItem,
    type ParserItem,
} from "./type";

export class CommandInitGameData extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    constructor(stacks: ItemStackArg[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.stacks = stacks;
    }

    public override convert(): string {
        return `!set-gamedata ${this.stacks.map((s) => s.convert(0)).join(" ")};`;
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
