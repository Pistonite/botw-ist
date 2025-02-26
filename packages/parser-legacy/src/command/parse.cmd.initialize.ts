import type { ItemStackArg } from "./ItemStackArg";
import type { ASTCommandInitialize } from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTItems } from "./parse.item";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParseItem,
    type ParserItem,
} from "./type";

export class CommandInitialize extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    constructor(stacks: ItemStackArg[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.stacks = stacks;
    }

    public convert(): string {
        return `!set-inventory ${this.stacks.map((s) => s.convert()).join(" ")};`;
    }
}

export const parseASTCommandInitialize: ParserItem<
    ASTCommandInitialize,
    CommandInitialize
> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(
        codeBlockFromRange(ast.mLiteralInitialize0, "keyword.command"),
    );
    return delegateParseItem(
        ast.mZeroOrMoreItems1,
        search,
        parseASTItems,
        (i, c) => new CommandInitialize(i, c),
        codeBlocks,
    );
};
