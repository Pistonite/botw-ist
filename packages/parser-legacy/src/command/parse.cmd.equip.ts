import { type ItemStack, ItemType, convertItem } from "./item.ts";
import {
    type ASTCommandEquip,
    type ASTCommandUnequip,
    type ASTCommandUnequipAll,
    isLiteralAll,
} from "./ast";
import { AbstractProperCommand } from "./command";
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
        this.slot = slot;
    }

    public override convert(): string {
        return `equip ${convertItem(this.item, this.slot, false)}`;
    }
}

export class CommandUnequip extends AbstractProperCommand {
    private item: ItemStack;
    private slot: number;
    constructor(item: ItemStack, slot: number, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.item = item;
        this.slot = slot;
    }

    public override convert(): string {
        return `unequip ${convertItem(this.item, this.slot, false)}`;
    }
}

export class CommandUnequipAll extends AbstractProperCommand {
    private types: ItemType[];
    constructor(types: ItemType[], codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.types = types;
    }
    public override convert(): string {
        let s = "";
        if (this.types.includes(ItemType.Weapon)) {
            s += "unequip all weapons";
        }
        if (this.types.includes(ItemType.Bow)) {
            s += "unequip all bows";
        }
        if (this.types.includes(ItemType.Shield)) {
            s += "unequip all shields";
        }
        // V3->V4: the types in V3 are named wrong
        const hasArmorHead = this.types.includes(ItemType.ArmorUpper);
        const hasArmorUpper = this.types.includes(ItemType.ArmorMiddle);
        const hasArmorLower = this.types.includes(ItemType.ArmorLower);
        if (hasArmorHead && hasArmorUpper && hasArmorLower) {
            s += "unequip all armors";
        } else {
            if (hasArmorHead) {
                s += "unequip all head-armors";
            }
            if (hasArmorUpper) {
                s += "unequip all upper-armors";
            }
            if (hasArmorLower) {
                s += "unequip all lower-armors";
            }
        }
        if (this.types.includes(ItemType.Material)) {
            s += "unequip all materials";
        }
        if (this.types.includes(ItemType.Food)) {
            s += "unequip all foods";
        }
        if (this.types.includes(ItemType.Key)) {
            s += "unequip all key-items";
        }
        return s;
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
