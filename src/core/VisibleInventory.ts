import { DisplayableInventory, DisplayableSlot, itemStackToDisplayableSlot } from "./DisplayableInventory";
import { Item, ItemStack } from "./Item";
import { Slots } from "./Slots";

/*
 * Implementation of Visible Inventory (PauseMenuDataMgr) in botw
 */
export class VisibleInventory implements DisplayableInventory{
    private slots: Slots = new Slots([]);
    /* Implementation of mCount in botw */
    private count: number = 0;
    constructor(slots: Slots, count: number){
        this.slots = slots;
        this.count = count;
    }

    public deepClone(): VisibleInventory {
        return new VisibleInventory(this.slots.deepClone(), this.count);
    }

    public getDisplayedSlots(): DisplayableSlot[] {
        return this.slots.getSlotsRef().map((stack, i)=>itemStackToDisplayableSlot(stack, i>=this.count));
    }

    public getSlots(): Slots {
        return this.slots;
    }

    public addDirectly(stack: ItemStack){
        this.count+=this.slots.addStackDirectly(stack);
    }

    public addWhenReload(item: Item, count: number, equippedDuringReload: boolean) {
        const slotsAdded = this.slots.add(item, count, equippedDuringReload, true, this.count);
        this.count+=slotsAdded;
    }

    //public addInGame

    // Only clears first this.count
    public clearForReload() {
        if(this.count > 0){
            this.slots.clearFirst(this.count);
            this.count = 0;
        }
    }

    public getCount(): number {
        return this.count;
    }

    public modifyCount(delta: number): void {
        this.count+=delta;
    }

    public resetCount(): void {
        this.count = this.slots.length;
    }
}
