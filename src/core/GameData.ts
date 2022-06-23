import { DisplayableInventory, DisplayableSlot, itemStackToDisplayableSlot } from "./DisplayableInventory";
import { Item, itemToItemData } from "./Item";
import { Slots } from "./Slots";
import { VisibleInventory } from "./VisibleInventory";

/*
 * Implementation of GameData in botw
 */
export class GameData implements DisplayableInventory {
    
    private slots: Slots = new Slots([]);
    constructor(slots: Slots){
        this.slots = slots;
    }

    public deepClone(): GameData {
        return new GameData(this.slots.deepClone());
    }
    
    public syncWith(pouch: VisibleInventory) {
        this.slots = pouch.getSlots().deepClone();
    }

    public updateDurability(durability: number, slot: number){
        this.slots.corrupt(durability, slot);
    }

    public addAllToPouchOnReload(pouch: VisibleInventory) {
        this.slots.getSlotsRef().forEach(stack=>pouch.addWhenReload(stack.item, stack.count, stack.equipped));
    }

    public getDisplayedSlots(): DisplayableSlot[] {
        return this.slots.getSlotsRef().map(stack=>itemStackToDisplayableSlot(stack, false));
    }
}
