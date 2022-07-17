import { SimulationState } from "core/SimulationState";
import { Item, MetaOption } from "data/item";
import { CommandImpl } from "./type";

export class CommandWrite extends CommandImpl {
	private itemTarget: Item;
	private slot: number;
	private meta: MetaOption;
	constructor(item: Item, slot: number, meta: MetaOption) {
		super();
		this.itemTarget = item;
		this.slot = slot;
		this.meta = meta;
	}

	execute(state: SimulationState): void {
		return state.setMetadata(this.itemTarget, this.slot, this.meta);
	}
}
