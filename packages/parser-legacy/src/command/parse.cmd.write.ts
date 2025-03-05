import { convertItemMeta, type MetaModifyOption } from "./item.ts";
import type { ASTCommandWriteMetadata } from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTArgumentSingleItemMaybeInSlot } from "./parse.clause.inslot";
import { parseASTMetadata } from "./parse.metadata";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    type ParserItem,
} from "./type";

export class CommandWrite extends AbstractProperCommand {
    private itemTarget: string;
    private slot: number;
    private meta: MetaModifyOption;
    constructor(
        item: string,
        slot: number,
        meta: MetaModifyOption,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.itemTarget = item;
        this.slot = slot - 1; //change to 0 based
        this.meta = meta;
    }

    public override convert() {
        const meta = convertItemMeta(this.meta);
        let slotClause = "";
        if (this.slot) {
            slotClause = ` in slot ${this.slot + 1}`;
        }
        return `write ${meta} to ${this.itemTarget}${slotClause};`;
    }
}

export const parseASTCommandWriteMetadata: ParserItem<
    ASTCommandWriteMetadata,
    CommandWrite
> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    const [meta, metaBlocks, metaError] = parseASTMetadata(ast.mMetadata1);
    codeBlocks.push(metaBlocks);
    codeBlocks.push(codeBlockFromRange(ast.literal2, "keyword.command"));
    const [result, itemAndSlotBlocks, resultError] =
        parseASTArgumentSingleItemMaybeInSlot(
            ast.mArgumentSingleItemMaybeInSlot3,
            search,
        );
    codeBlocks.push(itemAndSlotBlocks);
    if (!meta) {
        return [undefined, codeBlocks, metaError];
    }
    if (!result) {
        return [undefined, codeBlocks, resultError];
    }
    const [stack, slot] = result;
    return [
        new CommandWrite(stack.ident, slot, meta, codeBlocks),
        codeBlocks,
        "",
    ];
};
