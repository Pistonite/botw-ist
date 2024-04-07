import { ItemStack, ItemType } from "data/item";
import { Ref } from "data/util";
import { SlotDisplayForItemStack } from "./SlotDisplayForItemStack";
import { Slots } from "./Slots";
import { VisibleInventory } from "./VisibleInventory";
import { DisplayableInventory, GameFlags, SlotDisplay } from "./types";

/*
 * Implementation of GameData in botw
 */
export class GameData implements DisplayableInventory {
    private flags: GameFlags;
    private slots: Slots;
    constructor(slots: Slots, flags: Partial<GameFlags>) {
        this.slots = slots;
        this.flags = {
            weaponSlots: 8,
            bowSlots: 5,
            shieldSlots: 4,
            ...flags,
        };
    }

    public getFlags(): GameFlags {
        return this.flags;
    }

    public getFlag<T extends keyof GameFlags>(key: T): GameFlags[T] {
        return this.flags[key];
    }

    public setFlag(key: keyof GameFlags, value: string | number | boolean) {
        this.flags[key] = value as unknown as number;
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
        return new GameData(this.slots.deepClone(), this.flags);
    }

    public syncWith(pouch: VisibleInventory) {
        if (pouch.getMCount() <= 0) {
            // inventory nuking.
            // [confirmed] when mCount <=0, gamedata is nuked when syncing with pouch
            // https://discord.com/channels/269611402854006785/269616041435332608/998326332813480016
            this.slots = new Slots([]);
        } else {
            this.slots = pouch.getSlots().deepClone();
        }
    }

    public isSyncedWith(pouch: VisibleInventory) {
        return this.slots.equals(pouch.getSlots());
    }

    public updateLife(life: number, slot: number) {
        this.slots.updateLife(life, slot);
    }

    public addAllToPouchOnReload(pouch: VisibleInventory) {
        let lastAdded: Ref<ItemStack> | undefined = undefined;

        const allItems = this.slots.getView();
        // This is needed to simulate a case where the game load a food, but does not increment
        // the food counter, causing food data to be offset
        const allFood = allItems.filter(
            (stack) => stack.item.type === ItemType.Food,
        );
        let nextFood = 0;

        allItems.forEach((stack) => {
            const isFood = stack.item.type === ItemType.Food;
            const cookDataSource = isFood ? allFood[nextFood] : stack;
            lastAdded = pouch.addWhenReload(
                stack,
                cookDataSource,
                lastAdded,
                this.flags,
            );
            if (lastAdded && isFood) {
                nextFood++;
            }
        });
    }

    public getDisplayedSlots(isIconAnimated: boolean): SlotDisplay[] {
        return this.slots
            .getView()
            .map((stack) =>
                new SlotDisplayForItemStack(stack).init(false, isIconAnimated),
            );
    }
}
