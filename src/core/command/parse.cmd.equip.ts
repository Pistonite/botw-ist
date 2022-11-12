import { SimulationState } from "core/SimulationState";
import { Item } from "data/item";
import { ASTCommandEquip } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTArgumentSingleItemMaybeInSlot } from "./parse.clause.inslot";
import { codeBlockFromRange, CodeBlockTree, delegateParseItem, ParserItem } from "./type";

export class CommandEquip extends AbstractProperCommand  {
	private item: Item;
	private slot: number;
	constructor(item: Item, slot: number, codeBlocks: CodeBlockTree){
		super(codeBlocks);
		this.item = item;
		this.slot = slot-1;// change to 0-based
	}

	public execute(state: SimulationState): void {
		state.equip(this.item, this.slot);
	}
    public equals(other: Command): boolean {
        return other instanceof CommandEquip && other.item === this.item && this.slot === other.slot;
    }
}

// export class CommandUnequip extends CommandImpl {
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(item: Item, slot: number, noSlot: boolean){
// 		super();
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}

// 	public execute(state: SimulationState): void {
// 		state.unequip(this.item, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
// 		return `Unequip ${this.item}${slotString}`;
// 	}
// } 

export const parseASTCommandEquip: ParserItem<ASTCommandEquip, CommandEquip> = (ast, search) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParseItem(
        ast.mArgumentSingleItemMaybeInSlot1, 
        search,
        parseASTArgumentSingleItemMaybeInSlot, 
        ([stack, slot],c)=>new CommandEquip(stack.item, slot,c), 
        codeBlocks
    );
}
