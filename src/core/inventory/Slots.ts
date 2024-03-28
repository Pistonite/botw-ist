import { AmountAll, AmountAllType } from "core/command";
import { dumpItemStack, Item, ItemStack, ItemType, MetaModifyOption } from "data/item";
import { Ref } from "data/util";
import { SlotsCore } from "./SlotsCore";
import { add } from "./add";
import { RemoveOption } from "./options";
import { remove } from "./remove";
import { GameFlags } from "./types";

/*
 * This is the data model common to GameData and VisibleInventory
 */
export class Slots {
	private core: SlotsCore;
	constructor(slots: ItemStack[]) {
		this.core = new SlotsCore(slots);
	}
	public getView(): ItemStack[] {
		return this.core.getView();
	}
	public dump() {
		return this.core.getView().map(stack=>dumpItemStack(stack));
	}
	public deepClone(): Slots {
		// ItemStack is immutable so they do not need to be copied
		return new Slots([...this.core.getView()]);
	}

	public get length(): number {
		return this.core.length;
	}

	// Used to decide if game data is synced with inventory
	// Two Slots are equal if the ItemStacks equal, including metadata equality
	public equals(other: Slots): boolean {
		return this.core.equals(other.core);
	}

	// remove item(s) start from slot
	// return number of items removed
	public remove(toRemove: ItemStack, count: number | AmountAllType, option: Partial<RemoveOption> = {}): number {
		return remove(this.core, toRemove, count, option);
	}

	// Add something to inventory in game
	// See add.ts
	public add(stack: ItemStack, reloading: boolean, mCount: number | null, flags: GameFlags, lastAdded: Ref<ItemStack> | undefined, listHeadsInit?: boolean): Ref<ItemStack> | undefined {
		return add(this.core, stack, reloading, mCount, flags, lastAdded, listHeadsInit);
	}

	// this is for all types of item
	public equip(item: Item, slot: number) {
		let s = 0;
		// unequip same type in first tab
		// PF: all methods of handling equipping in-game seem to be unaffected by mListHeads
		const [firstTabItem, firstTabIndex] = this.core.findFirstTab(item.type, true);
		if(firstTabItem){
			for(let i = firstTabIndex;i<this.core.length && this.core.get(i).item.tab === item.tab;i++){
				if( this.core.get(i).item.type === item.type){
					this.core.modifySlot(i, {equipped: false});
				}
			}
		}

		// now search for the one the player selects and equip it
		for(let i = 0; i<this.core.length;i++){
			if( this.core.get(i).item === item){
				if (s===slot){
					this.core.modifySlot(i, {equipped: true});
					break;
				}
				s++;
			}
		}
	}
	public unequip(item: Item, slot: number) {
		let s = 0;

		// [needs confirm] checks entire inventory?
		for(let i = 0; i<this.core.length;i++){
			if(this.core.get(i).item === item){
				if(slot <= 0){
					if(this.core.get(i).equipped){
						this.core.modifySlot(i, {equipped: false});
						break;
					}
				}else{
					if(s<slot){
						s++;
					}else{
						this.core.modifySlot(i, {equipped: false});
						break;
					}
				}
			}
		}
	}

	public updateLife(life: number, slot: number) {

		if(slot < 0 || slot >= this.core.length){
			return;
		}
		const stack = this.core.get(slot);

		const type = stack.item.type;
		const stackable = stack.item.stackable;
		// [confirmed] material and meals are capped at 999
		// meals: https://discord.com/channels/269611402854006785/269616041435332608/1000253331668742265
		const isMaterialOrMeal = type === ItemType.Material || type === ItemType.Food;
		// [confirmed] arrows are not capped at 999
		const isArrow = type === ItemType.Arrow;
		// [confirmed] stackble key items are capped
		// https://discord.com/channels/269611402854006785/269616041435332608/1003165317125656586
		const isStackableKey = type === ItemType.Key && stackable;

		const shouldCapAt999 = !isArrow && (isMaterialOrMeal || isStackableKey);
		if(shouldCapAt999){
			life = Math.min(999, life);
		}

		this.core.modifySlot(slot, {count: life});

	}

	// shoot count arrows. return the slot that was updated, or -1
	public shootArrow(countToShoot: number | AmountAllType): number {
		// first find equipped arrow, search entire inventory
		const lastEquippedArrow = this.findLastEquipped(ItemType.Arrow);
		if(!lastEquippedArrow){
			return -1;
		}
		const equippedArrow: Item = lastEquippedArrow.get().item;
		// now find the first slot of that arrow and update
		for(let j=0;j<this.core.length;j++){
			const {item, count} = this.core.get(j);
			if(item === equippedArrow){
				if(countToShoot === AmountAll){
					this.core.modifySlot(j, {count: 0});
				}else{
					this.core.modifySlot(j, {count: Math.max(0, count-countToShoot)});
				}

				return j;
			}
		}
		//for some reason cannot find that arrow now?
		return -1;
	}

	// Other core delegates
	public clearFirst(count: number) {
		this.core.clearFirst(count);
	}

	public addStackDirectly(stack: ItemStack, index?: number): Ref<ItemStack> {
		return this.core.addStackDirectly(stack, index);
	}

	public findLastEquipped(type: ItemType): Ref<ItemStack> | undefined{
		return this.core.findLastEquipped(type);
	}

	public setMetadata(item: Item, slot: number, meta: MetaModifyOption) {
		this.core.setMetadata(item, slot, meta);
	}

	public removeAll(types: ItemType[]) {
		this.core.removeAll(types);
	}

	public unequipAll(types: ItemType[]) {
		this.core.unequipAll(types);
	}

	public swap(i: number, j: number){
		this.core.swap(i, j);
	}

}
