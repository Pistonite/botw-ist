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
import { getParsingCommand } from "./parsev2.ts";

// Remove, Sell, With, Drop
export class CommandRemove extends AbstractProperCommand {
    private stacks: ItemStackArg[];
    private verb: string;
    private slot: number;
    constructor(
        stacks: ItemStackArg[],
        slot: number,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.stacks = stacks;
        this.slot = slot;
        this.verb = "!remove";
    }
    public setVerb(x: string): CommandRemove {
        this.verb = x;
        return this;
    }
    public override convert(): string {
        return `${this.verb} ${this.stacks.map((s) => s.convert(this.slot)).join(" ")};`;
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
    private verb: string;
    constructor(types: ItemType[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.types = types;
        this.verb = "!remove";
    }
    public setVerb(x: string): CommandRemoveAll {
        this.verb = x;
        return this;
    }
    public override convert(): string {
        let s = "";
        if (this.types.includes(ItemType.Weapon)) {
            s += `${this.verb} all weapons;`;
        }
        if (this.types.includes(ItemType.Bow)) {
            s += `${this.verb} all bows;`;
        }
        if (this.types.includes(ItemType.Shield)) {
            s += `${this.verb} all shields;`;
        }
        // V3->V4: the types in V3 are named wrong
        const hasArmorHead = this.types.includes(ItemType.ArmorUpper);
        const hasArmorUpper = this.types.includes(ItemType.ArmorMiddle);
        const hasArmorLower = this.types.includes(ItemType.ArmorLower);
        if (hasArmorHead && hasArmorUpper && hasArmorLower) {
            s += `${this.verb} all armors;`;
        } else {
            if (hasArmorHead) {
                s += `${this.verb} all head-armors;`;
            }
            if (hasArmorUpper) {
                s += `${this.verb} all upper-armors;`;
            }
            if (hasArmorLower) {
                s += `${this.verb} all lower-armors;`;
            }
        }
        if (this.types.includes(ItemType.Material)) {
            s += `${this.verb} all materials;`;
        }
        if (this.types.includes(ItemType.Food)) {
            s += `${this.verb} all foods;`;
        }
        if (this.types.includes(ItemType.Key)) {
            s += `${this.verb} all key-items;`;
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
    const range = ast.mLiteralRemove0.range;
    const script = getParsingCommand().substring(range[0], range[1]);
    const isSell = script.toLowerCase() === "sell";
    codeBlocks.push(codeBlockFromRange(ast.mLiteralRemove0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
        search,
        parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
        (i, c) => {
            const x = new CommandRemove(...i, c);
            if (isSell) {
                return x.setVerb("sell");
            }
            return x;
        },
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
    const isDrop = isLiteralDrop(ast.mLiteralRemoveOrDrop0);
    const literal0 = isDrop
        ? ast.mLiteralRemoveOrDrop0.literal0
        : ast.mLiteralRemoveOrDrop0;
    const literal0Range = isDrop
        ? ast.mLiteralRemoveOrDrop0.literal0
        : ast.mLiteralRemoveOrDrop0.range;
    const script = getParsingCommand();
    const isSell =
        script.substring(literal0Range[0], literal0Range[1]).toLowerCase() ===
        "sell";
    const codeBlocks: CodeBlockTree = [
        codeBlockFromRange(literal0, "keyword.command"),
        codeBlockFromRange(ast.literal1, "item.type"),
    ];
    return delegateParseSafe(
        ast.mLiteralItemType2,
        parseASTItemType,
        (itemTypes, c) => {
            const x = new CommandRemoveAll(itemTypes, c);
            if (isSell) {
                return x.setVerb("sell");
            }
            if (isDrop) {
                return x.setVerb("drop");
            }
            return x;
        },
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
