import { ItemType } from "./item.ts";
import type { ItemStackArg } from "./ItemStackArg";
import {
    type ASTCommandDnp,
    type ASTCommandDrop,
    type ASTCommandEat,
    type ASTCommandRemove,
    type ASTCommandRemoveAll,
    isLiteralDrop,
} from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTItemType } from "./parse.basis";
import { parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot } from "./parse.clause.with.fromslot";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParseItem,
    delegateParseSafe,
    type ParserItem,
    type ParserSafe,
} from "./type";

// Remove, Sell, With, Drop
export class CommandRemove extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    private slot: number;
    constructor(
        stacks: ItemStackArg[],
        slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.stacks = stacks;
        this.slot = slot;
    }
    public override convert(): string {
        return `!remove ${this.stacks.map((s) => s.convert(this.slot)).join(" ")};`;
    }
}

// Eat (deletes arrow slots)
export class CommandEat extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    private slot: number;
    constructor(
        stacks: ItemStackArg[],
        slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.stacks = stacks;
        this.slot = slot;
    }
    public override convert(): string {
        return `eat ${this.stacks.map((s) => s.convert(this.slot)).join(" ")};`;
    }
}

// Remove all type
export class CommandRemoveAll extends AbstractProperCommand {
    private types: ItemType[];
    constructor(types: ItemType[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.types = types;
    }
    public override convert(): string {
        let s = "";
        if (this.types.includes(ItemType.Weapon)) {
            s += "!remove all weapons;";
        }
        if (this.types.includes(ItemType.Bow)) {
            s += "!remove all bows;";
        }
        if (this.types.includes(ItemType.Shield)) {
            s += "!remove all shields;";
        }
        // V3->V4: the types in V3 are named wrong
        const hasArmorHead = this.types.includes(ItemType.ArmorUpper);
        const hasArmorUpper = this.types.includes(ItemType.ArmorMiddle);
        const hasArmorLower = this.types.includes(ItemType.ArmorLower);
        if (hasArmorHead && hasArmorUpper && hasArmorLower) {
            s += "!remove all armors;";
        } else {
            if (hasArmorHead) {
                s += "!remove all head-armors;";
            }
            if (hasArmorUpper) {
                s += "!remove all upper-armors;";
            }
            if (hasArmorLower) {
                s += "!remove all lower-armors;";
            }
        }
        if (this.types.includes(ItemType.Material)) {
            s += "!remove all materials;";
        }
        if (this.types.includes(ItemType.Food)) {
            s += "!remove all foods;";
        }
        if (this.types.includes(ItemType.Key)) {
            s += "!remove all key-items;";
        }
        return s;
    }
}

export class CommandDnp extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    private slot: number;
    constructor(
        stacks: ItemStackArg[],
        slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.stacks = stacks;
        this.slot = slot;
    }
    public override convert(): string {
        return `dnp ${this.stacks.map((s) => s.convert(this.slot)).join(" ")};`;
    }
}

export const parseASTCommandRemove: ParserItem<
    ASTCommandRemove,
    CommandRemove
> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralRemove0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
        search,
        parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
        (i, c) => new CommandRemove(...i, c),
        codeBlocks,
    );
};

export const parseASTCommandDrop: ParserItem<ASTCommandDrop, CommandRemove> = (
    ast,
    search,
) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
        search,
        parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
        (i, c) => new CommandRemove(...i, c),
        codeBlocks,
    );
};

export const parseASTCommandEat: ParserItem<ASTCommandEat, CommandEat> = (
    ast,
    search,
) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
        search,
        parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
        (i, c) => new CommandEat(...i, c),
        codeBlocks,
    );
};

export const parseASTCommandRemoveAll: ParserSafe<
    ASTCommandRemoveAll,
    CommandRemoveAll
> = (ast) => {
    const literal0 = isLiteralDrop(ast.mLiteralRemoveOrDrop0)
        ? ast.mLiteralRemoveOrDrop0.literal0
        : ast.mLiteralRemoveOrDrop0;
    const codeBlocks: CodeBlockTree = [
        codeBlockFromRange(literal0, "keyword.command"),
        codeBlockFromRange(ast.literal1, "item.type"),
    ];
    return delegateParseSafe(
        ast.mLiteralItemType2,
        parseASTItemType,
        (itemTypes, c) => new CommandRemoveAll(itemTypes, c),
        codeBlocks,
    );
};

export const parseASTCommandDnp: ParserItem<ASTCommandDnp, CommandDnp> = (
    ast,
    search,
) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.mLiteralDnp0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
        search,
        parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
        (i, c) => new CommandDnp(...i, c),
        codeBlocks,
    );
};
