import { SimulationState } from "core/SimulationState";
import { arrayEqual } from "data/util";
import { getSlotsToAdd, ItemStackArg } from "./ItemStackArg";
import {
    ASTSuperCommandAddSlot,
    ASTSuperCommandSortMaterial,
    ASTSuperCommandSwap,
} from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTInteger } from "./parse.basis";
import { parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot } from "./parse.clause.with.fromslot";
import {
    codeBlockFromRange,
    CodeBlockTree,
    delegateParseItem,
    flattenCodeBlocks,
    ParserItem,
    ParserSafe,
} from "./type";

export class SuperCommandSwap extends AbstractProperCommand {
    private i: number;
    private j: number;
    constructor(i: number, j: number, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.i = i;
        this.j = j;
    }

    public execute(state: SimulationState): void {
        state.swap(this.i, this.j);
    }

    public equals(other: Command): boolean {
        return (
            other instanceof SuperCommandSwap &&
            this.i === other.i &&
            this.j === other.j
        );
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
    private slot: number;
    constructor(
        stacks: ItemStackArg[],
        slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.stacks = stacks;
        this.slot = slot - 1; //change to 0 based
    }
    public execute(state: SimulationState): void {
        state.addSlotsDirectly(getSlotsToAdd(this.stacks), this.slot);
    }
    public equals(other: Command): boolean {
        return (
            other instanceof SuperCommandAddSlot &&
            arrayEqual(this.stacks, other.stacks) &&
            this.slot === other.slot
        );
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
    constructor(codeBlocks: CodeBlockTree) {
        super(codeBlocks);
    }
    public execute(state: SimulationState): void {
        state.inaccuratelySortMaterials();
    }
    public equals(other: Command): boolean {
        return other instanceof SuperCommandSortMaterial;
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
