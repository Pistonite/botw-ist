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
    private verb: string = "destroy";
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
    public setVerb(verb: string): CommandRemove {
        this.verb = verb;
        return this;
    }
    public override convert(): string {
        let s = `${this.verb} ${this.stacks.map((s) => s.convert()).join(" ")}`;
        if (this.slot) {
            s += ` from slot ${this.slot + 1}`;
        }
        s += ";";
        return s;
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
        this.slot = slot - 1; //change to 0 based
    }
    public override convert(): string {
        let s = `eat ${this.stacks.map((s) => s.convert()).join(" ")}`;
        if (this.slot) {
            s += ` from slot ${this.slot + 1}`;
        }
        s += ";";
        return s;
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
            s += "destroy all weapons;";
        }
        if (this.types.includes(ItemType.Bow)) {
            s += "destroy all bows;";
        }
        if (this.types.includes(ItemType.Shield)) {
            s += "destroy all shields;";
        }
        // V3->V4: the types in V3 are named wrong
        const hasArmorHead = this.types.includes(ItemType.ArmorUpper);
        const hasArmorUpper = this.types.includes(ItemType.ArmorMiddle);
        const hasArmorLower = this.types.includes(ItemType.ArmorLower);
        if (hasArmorHead && hasArmorUpper && hasArmorLower) {
            s += "destroy all armors;";
        } else {
            if (hasArmorHead) {
                s += "destroy all head-armors;";
            }
            if (hasArmorUpper) {
                s += "destroy all upper-armors;";
            }
            if (hasArmorLower) {
                s += "destroy all lower-armors;";
            }
        }
        if (this.types.includes(ItemType.Material)) {
            s += "destroy all materials;";
        }
        if (this.types.includes(ItemType.Food)) {
            s += "destroy all foods;";
        }
        if (this.types.includes(ItemType.Key)) {
            s += "destroy all key-items;";
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
        this.slot = slot - 1; //change to 0 based
    }
    public override convert(): string {
        let s = `dnp ${this.stacks.map((s) => s.convert()).join(" ")}`;
        if (this.slot) {
            s += ` from slot ${this.slot + 1}`;
        }
        s += ";";
        return s;
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
        (i, c) => new CommandRemove(...i, c).setVerb("drop"),
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
