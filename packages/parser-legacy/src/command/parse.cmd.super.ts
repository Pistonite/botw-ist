import type { ItemStackArg } from "./ItemStackArg";
import type {
    ASTSuperCommandAddSlot,
    ASTSuperCommandSortMaterial,
    ASTSuperCommandSwap,
} from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTInteger } from "./parse.basis";
import { parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot } from "./parse.clause.with.fromslot";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParseItem,
    flattenCodeBlocks,
    type ParserItem,
    type ParserSafe,
} from "./type";

export class SuperCommandSwap extends AbstractProperCommand {
    private i: number;
    private j: number;
    constructor(i: number, j: number, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.i = i;
        this.j = j;
    }

    public convert(): string {
        return `!swap ${this.i} ${this.j};`;
    }
}

export const parseASTSuperCommandSwap: ParserSafe<
    ASTSuperCommandSwap,
    SuperCommandSwap
> = (ast) => {
    const codeBlocks: CodeBlockTree = [
        codeBlockFromRange(ast.literal0, "keyword.super"),
        codeBlockFromRange(ast.literal1, "keyword.super"),
    ];
    const [i, iBlock] = parseASTInteger(ast.mInteger2);
    const [j, jBlock] = parseASTInteger(ast.mInteger3);

    codeBlocks.push(flattenCodeBlocks([], [iBlock, jBlock], "slot.number"));

    return [new SuperCommandSwap(i, j, codeBlocks), codeBlocks];
};

export class SuperCommandAddSlot extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    private _slot: number;
    constructor(
        stacks: ItemStackArg[],
        slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.stacks = stacks;
        this._slot = slot - 1; //change to 0 based
    }
    public convert() {
        return `### "!add-slot" is no longer supported!!!\n### !add-slot ${this.stacks.map((s) => s.convert()).join(" ")};`;
    }
}

export const parseASTSuperCommandAddSlot: ParserItem<
    ASTSuperCommandAddSlot,
    SuperCommandAddSlot
> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [
        codeBlockFromRange(ast.literal0, "keyword.super"),
        codeBlockFromRange(ast.literal1, "keyword.super"),
        codeBlockFromRange(ast.mLiteralSlot2, "keyword.super"),
    ];

    return delegateParseItem(
        ast.mArgumentOneOrMoreItemsMaybeFromSlot3,
        search,
        parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
        ([stacks, slot], codeBlocks) =>
            new SuperCommandAddSlot(stacks, slot, codeBlocks),
        codeBlocks,
    );
};

export class SuperCommandSortMaterial extends AbstractProperCommand {
    public convert(): string {
        return `sort materials;`;
    }
}

export const parseASTSuperCommandSortMaterial: ParserSafe<
    ASTSuperCommandSortMaterial,
    SuperCommandSortMaterial
> = (ast) => {
    const codeBlocks: CodeBlockTree = [
        codeBlockFromRange(ast.literal0, "keyword.super"),
        codeBlockFromRange(ast.literal1, "keyword.super"),
    ];

    return [new SuperCommandSortMaterial(codeBlocks), codeBlocks];
};
