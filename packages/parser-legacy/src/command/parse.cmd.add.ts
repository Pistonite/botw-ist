import type { ItemStackArg } from "./ItemStackArg";
import type { ASTCommandAdd, ASTCommandPickUp } from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTItems } from "./parse.item";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParseItem,
    type ParserItem,
} from "./type";

export class CommandAdd extends AbstractProperCommand {
    private verb: string = "get";
    private stacks: ItemStackArg[];
    constructor(stacks: ItemStackArg[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.stacks = stacks;
    }

    public setVerb(verb: string): CommandAdd {
        this.verb = verb;
        return this;
    }

    public override convert(): string {
        return `${this.verb} ${this.stacks.map((s) => s.convert(0)).join(" ")};`;
    }
}

export const parseASTCommandAdd: ParserItem<ASTCommandAdd, CommandAdd> = (
    ast,
    search,
) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralAdd0, "keyword.command"));
    return delegateParseItem(
        ast.mOneOrMoreItems1,
        search,
        parseASTItems,
        (i, c) => new CommandAdd(i, c),
        codeBlocks,
    );
};

export const parseASTCommandPickup: ParserItem<ASTCommandPickUp, CommandAdd> = (
    ast,
    search,
) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralPickUp0, "keyword.command"));
    return delegateParseItem(
        ast.mOneOrMoreItems1,
        search,
        parseASTItems,
        (i, c) => new CommandAdd(i, c).setVerb("pick-up"),
        codeBlocks,
    );
};
