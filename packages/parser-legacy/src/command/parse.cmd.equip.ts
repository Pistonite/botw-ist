import { type ItemStack, type ItemType, convertItem } from "./item.ts";
import {
    type ASTCommandEquip,
    type ASTCommandUnequip,
    type ASTCommandUnequipAll,
    isLiteralAll,
} from "./ast";
import { AbstractProperCommand, type Command } from "./command";
import { parseASTItemType } from "./parse.basis";
import { parseASTArgumentSingleItemMaybeInSlot } from "./parse.clause.inslot";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParseItem,
    delegateParseSafe,
    type ParserItem,
    type ParserSafe,
} from "./type";

export class CommandEquip extends AbstractProperCommand {
    private item: ItemStack;
    private slot: number;
    constructor(item: ItemStack, slot: number, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.item = item;
        this.slot = slot - 1; // change to 0-based
    }

    public convert(): string {
        let s = `equip ${convertItem(this.item)}`;
        if (this.slot !== 0) {
            s += ` in slot ${this.slot + 1}`;
        }
        s += ";";
        return s;
    }
}

export class CommandUnequip extends AbstractProperCommand {
    private item: ItemStack;
    private slot: number;
    constructor(item: ItemStack, slot: number, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.item = item;
        this.slot = slot - 1; // change to 0-based
    }

    public convert(): string {
        let s = `unequip ${convertItem(this.item)}`;
        if (this.slot !== 0) {
            s += ` in slot ${this.slot + 1}`;
        }
        s += ";";
        return s;
    }
}

export class CommandUnequipAll extends AbstractProperCommand {
    private types: ItemType[];
    constructor(types: ItemType[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.types = types;
    }
    public convert(): string {
    }
    public execute(state: SimulationState): void {
        state.unequipAll(this.types);
    }
    public equals(other: Command): boolean {
        return (
            other instanceof CommandUnequipAll &&
            arrayShallowEqual(this.types, other.types)
        );
    }
}

export const parseASTCommandEquip: ParserItem<ASTCommandEquip, CommandEquip> = (
    ast,
    search,
) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentSingleItemMaybeInSlot1,
        search,
        parseASTArgumentSingleItemMaybeInSlot,
        ([stack, slot], c) => new CommandEquip(stack, slot, c),
        codeBlocks,
    );
};

export const parseASTCommandUnequip: ParserItem<
    ASTCommandUnequip,
    CommandUnequip
> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentSingleItemMaybeInSlot1,
        search,
        parseASTArgumentSingleItemMaybeInSlot,
        ([stack, slot], c) => new CommandUnequip(stack, slot, c),
        codeBlocks,
    );
};

export const parseASTCommandUnequipAll: ParserSafe<
    ASTCommandUnequipAll,
    CommandUnequipAll
> = (ast) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const argItemType = ast.mLiteralMaybeAllItemType1;
    const astAll = argItemType.mLiteralMaybeAll0;
    if (isLiteralAll(astAll)) {
        codeBlocks.push(codeBlockFromRange(astAll.literal0, "item.type"));
    }
    const astItemType = argItemType.mLiteralItemType1;
    return delegateParseSafe(
        astItemType,
        parseASTItemType,
        (itemTypes, c) => new CommandUnequipAll(itemTypes, c),
        codeBlocks,
    );
};
