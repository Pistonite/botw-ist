import { SlotDisplayForItemStack } from "./SlotDisplayForItemStack";
import { Slots } from "./Slots";
import { VisibleInventory } from "./VisibleInventory";
import { DisplayableInventory, SlotDisplay } from "./types";
import { Ref } from "data/util";
import { ItemStack } from "data/item";

/*
 * Implementation of GameData in botw
 */
export class GameData implements DisplayableInventory {

	private slots: Slots = new Slots([]);
	constructor(slots: Slots){
		this.slots = slots;
	}
	public dump() {
		return {
			slots: this.slots.dump(),
		};
	}

	public equals(other: GameData): boolean {
		return this.slots.equals(other.slots);
	}

	public deepClone(): GameData {
		return new GameData(this.slots.deepClone());
	}

	public syncWith(pouch: VisibleInventory) {
		if(pouch.getMCount() <=0){
			// inventory nuking.
			// [confirmed] when mCount <=0, gamedata is nuked when syncing with pouch
			// https://discord.com/channels/269611402854006785/269616041435332608/998326332813480016
			this.slots = new Slots([]);
		}else{
			this.slots = pouch.getSlots().deepClone();
		}
	}

	public isSyncedWith(pouch: VisibleInventory) {
		return this.slots.equals(pouch.getSlots());
	}

	public updateLife(life: number, slot: number){
		this.slots.updateLife(life, slot);
	}

	public addAllToPouchOnReload(pouch: VisibleInventory) {
		let lastAdded: Ref<ItemStack> | undefined = undefined;
		this.slots.getView().forEach(stack=>{
			lastAdded = pouch.addWhenReload(stack, lastAdded);
		});
	}

	public getDisplayedSlots(isIconAnimated: boolean): SlotDisplay[] {
		return this.slots.getView().map(stack=>
			new SlotDisplayForItemStack(stack).init(false, isIconAnimated)
		);
	}
}
