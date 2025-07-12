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

    public override convert(): string {
        return `### !swap by index is no longer supported. Please use !swap ITEM1 and ITEM2\n# !swap ${this.i} ${this.j}`;
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
    // private _slot: number;
    constructor(
        stacks: ItemStackArg[],
        _slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.stacks = stacks;
        // this._slot = slot - 1; //change to 0 based
    }
    public override convert() {
        return `!add-slot ${this.stacks.map((s) => s.convert(0, false)).join(" ")}`;
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
    public override convert(): string {
        return `sort materials`;
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
