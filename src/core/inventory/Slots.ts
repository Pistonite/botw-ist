import { arrayEqual, stableSort } from "data/util";
import { getTabFromType, Item, ItemStack, ItemTab, ItemType, iterateItemTabs, MetaOption } from "data/item";
import { SlotsCore } from "./SlotsCore";
import { AmountAllType } from "core/command/ItemStackArg";
import { RemoveOption } from "./options";
import { remove } from "./remove";

/*
 * This is the data model common to GameData and VisibleInventory
 */
export class Slots {
	private core: SlotsCore;
	constructor(slots: ItemStack[]) {
		this.core = new SlotsCore(slots);
	}
	public getSlotsRef(): ItemStack[] {
		return this.core.internalSlots;
	}
	public deepClone(): Slots {
		// ItemStack is immutable so they do not need to be copied
		return new Slots([...this.core.internalSlots]);
	}

	public get length(): number {
		return this.core.internalSlots.length;
	}

	// Used to decide if game data is synced with inventory
	// Two Slots are equal if the ItemStacks equal, including metadata equality
	public equals(other: Slots): boolean {
		return this.core.equals(other.core);
	}

	// remove item(s) start from slot
	// return if removal is successful
	public remove(toRemove: ItemStack, count: number | AmountAllType, option: Partial<RemoveOption> = {}): boolean {
		return remove(this.core, toRemove, count, option);
	}

	// Add something to inventory in game
	// returns number of slots added
	public add(stack: ItemStack, reloading: boolean, mCount: number | null): number {
		const internalSlots = this.core.internalSlots;
		if(mCount === null){
			mCount = internalSlots.length;
		}

		// If item is stackable (arrow, material, spirit orbs), do 999 Cap Check
		// [confirmed] the 999 cap check always happens, even when mCount = 0
		// https://discord.com/channels/269611402854006785/269616041435332608/997404941754839060
		if(stack.item.stackable){
			let shouldCapAt999 = true;
			// [confirmed] kinak: for arrow, if there's "no arrow", 999 check is skipped
			if(stack.item.type === ItemType.Arrow){
				// [needs confirm] index will be -1 if mCount is 0, since we won't find any tabs, so arrow check is skipped regardless
				const firstArrowIndex = this.core.findFirstTabIndex(ItemType.Arrow, mCount);
				if(firstArrowIndex === -1){
					shouldCapAt999 = false;
				}
			}
			// [confirmed] 999 check (i.e. merge check) scans entire inventory
			// https://discord.com/channels/269611402854006785/269616041435332608/997764628492865572
			// [needs confirm] arrow special case works not during reload? for now, does not consider arrow case when not reloading
			// Check if there's already a slot, if so, add it to that and cap it at 999
			for(let i = 0; i<internalSlots.length;i++){
				if(internalSlots[i].equalsExcept(stack, "life")){
					if(reloading){
						if(shouldCapAt999){
							if(internalSlots[i].count + stack.count > 999){
								// [confirmed] do not add new stack during loading save, if it would exceed 999
								return 0;
							}
						}
					}else{
						// [needs confirm] if not reloading, cap the slot at 999
						const newCount = Math.min(999, internalSlots[i].count+stack.count);
						if(newCount != internalSlots[i].count){
							internalSlots[i] = internalSlots[i].modify({count: newCount});
						}

						return 0;
					}

				}
			}

		}

		// [confirmed] this check does not happen if mCount = 0 (which is covered by i!==-1 line below)
		// unrepeatable check: if a (unstackable) key item or master sword already exists in the first tab, do not add
		if(!stack.item.repeatable) {// only unstackable key items and master sword is not repeatable
			let i = this.core.findFirstTabIndex(stack.item.type, mCount);
			if(i!==-1){
				for(;i<internalSlots.length && internalSlots[i].item.type === stack.item.type;i++){
					if(internalSlots[i].item === stack.item){
						// Found the key item/master sword, do not add
						return 0;
					}
				}
				// past first (maybe empty) tab, check pass
			}
		}
		// Checks finish, do add new slot

		// Auto equip check
		if(!reloading){
			if(stack.item.type===ItemType.Weapon || stack.item.type===ItemType.Bow || stack.item.type===ItemType.Shield || stack.item.type === ItemType.Arrow){
				// [needs confirm] does auto equip check check entire inventory or only first tab? (for now, entire inventory)
				// [needs confirm] does this check happen for count = 0 ? (or just equip by force)
				// check if none of that type is equipped
				const equippedItems = internalSlots.filter(s=>
					s.item.type === stack.item.type && s.equipped
				);
				let shouldEquipNew = equippedItems.length === 0;
				if(!shouldEquipNew && stack.item.type === ItemType.Arrow){
					shouldEquipNew = equippedItems.filter(s=>
						s.count > 0
					).length === 0;
				}
				if(shouldEquipNew){
					stack = stack.modify({equipped: true});
					if(stack.item.type === ItemType.Arrow){
						// unequip other arrows
						// [needs confirm] only first tab?
						let i = this.core.findFirstTabIndex(ItemType.Arrow, mCount);
						if(i!==-1){
							for(;i<internalSlots.length && internalSlots[i].item.type === ItemType.Arrow;i++){
								internalSlots[i] = internalSlots[i].modify({equipped: false});
							}
						}
					}
				}
			}
		}

		this.core.addSlot(stack, mCount+1);
		return 1;
	}

	// this is for all types of item
	public equip(item: Item, slot: number, mCount: number) {
		const internalSlots = this.core.internalSlots;
		let s = 0;
		// unequip same type in first tab
		let i = this.core.findFirstTabIndex(item.type, mCount);
		if(i!==-1){
			for(;i<internalSlots.length && internalSlots[i].item.tab === item.tab;i++){
				if(internalSlots[i].item.type === item.type){
					this.core.modifySlot(i, {equipped: false});
				}
			}
		}

		// now search for the one the player selects and equip it
		for(let i = 0; i<internalSlots.length;i++){
			if(internalSlots[i].item === item){
				if (s===slot){
					this.core.modifySlot(i, {equipped: true});
					break;
				}
				s++;
			}
		}
	}
	public unequip(item: Item, slot: number) {
		const internalSlots = this.core.internalSlots;
		let s = 0;
		const type = item.type;
		if (type===ItemType.Arrow){
			return; // cannot unequip arrow
		}
		// [needs confirm] checks entire inventory?
		for(let i = 0; i<internalSlots.length;i++){
			if(internalSlots[i].item === item){
				if(slot < 0){
					if(internalSlots[i].equipped){
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
		if(slot < 0 || slot >= this.core.internalSlots.length){
			return;
		}
		const type = this.core.internalSlots[slot].item.type;
		const stackable = this.core.internalSlots[slot].item.stackable;
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
	public shootArrow(count: number): number {
		// first find equipped arrow, search entire inventory
		const lastEquippedArrowSlot = this.findLastEquippedSlot(ItemType.Arrow);
		if(lastEquippedArrowSlot < 0){
			return -1;
		}
		const equippedArrow: Item = this.core.internalSlots[lastEquippedArrowSlot].item;
		// now find the first slot of that arrow and update
		for(let j=0;j<this.core.internalSlots.length;j++){
			if(this.core.internalSlots[j].item === equippedArrow){
				this.core.modifySlot(j, {count: Math.max(0, this.core.internalSlots[j].count-count)});
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

	public addStackDirectly(stack: ItemStack) {
		this.core.addStackDirectly(stack);
	}

	public findLastEquippedSlot(type: ItemType): number {
		return this.core.findLastEquippedSlot(type);
	}

	public setMetadata(item: Item, slot: number, meta: MetaOption) {
		this.core.setMetadata(item, slot, meta);
	}

	public clearAllButKeyItems() {
		this.core.clearAllButKeyItems();
	}

}
