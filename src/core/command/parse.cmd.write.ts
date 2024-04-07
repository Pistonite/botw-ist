import { SimulationState } from "core/SimulationState";
import { Item, MetaModifyOption } from "data/item";
import { ASTCommandWriteMetadata } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTArgumentSingleItemMaybeInSlot } from "./parse.clause.inslot";
import { parseASTMetadata } from "./parse.metadata";
import { codeBlockFromRange, CodeBlockTree, ParserItem } from "./type";

export class CommandWrite extends AbstractProperCommand {
    private itemTarget: Item;
    private slot: number;
    private meta: MetaModifyOption;
    constructor(
        item: Item,
        slot: number,
        meta: MetaModifyOption,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.itemTarget = item;
        this.slot = slot - 1; //change to 0 based
        this.meta = meta;
    }

    public execute(state: SimulationState): void {
        return state.setMetadata(this.itemTarget, this.slot, this.meta);
    }

    public equals(other: Command): boolean {
        return (
            other instanceof CommandWrite &&
            this.itemTarget === other.itemTarget &&
            this.slot === other.slot &&
            JSON.stringify(this.meta) === JSON.stringify(other.meta)
        );
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
        new CommandWrite(stack.item, slot, meta, codeBlocks),
        codeBlocks,
        "",
    ];
};
