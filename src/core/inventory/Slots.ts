import { AmountAll, AmountAllType } from "core/command";
import { dumpItemStack, Item, ItemMaxes, ItemStack, ItemType, MetaModifyOption } from "data/item";
import { SlotsCore } from "./SlotsCore";
import { RemoveOption } from "./options";
import { remove } from "./remove";
import { Ref } from "data/util";
import { off } from "process";

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
	// returns the added slot ref, or undefined if no new slot is added
	public add(stack: ItemStack, reloading: boolean, mCount: number | null): Ref<ItemStack> | undefined {
		if(mCount === null){
			mCount = this.core.length;
		}

		// If item is stackable (arrow, material, spirit orbs), do 999 Cap Check
		// [confirmed] the 999 cap check always happens, even when mCount = 0
		// https://discord.com/channels/269611402854006785/269616041435332608/997404941754839060
		if(stack.item.stackable){
			let shouldCapAt999 = true;
			// [confirmed] kinak: for arrow, if there's "no arrow", 999 check is skipped
			if(stack.item.type === ItemType.Arrow){
				// [needs confirm] index will be -1 if mCount is 0, since we won't find any tabs, so arrow check is skipped regardless
				const [firstArrowItem] = this.core.findFirstTab(ItemType.Arrow, mCount);
				if(!firstArrowItem){
					shouldCapAt999 = false;
				}
			}
			// [confirmed] 999 check (i.e. merge check) scans entire inventory
			// https://discord.com/channels/269611402854006785/269616041435332608/997764628492865572
			// [needs confirm] arrow special case works not during reload? for now, does not consider arrow case when not reloading
			// Check if there's already a slot, if so, add it to that and cap it at 999
			for(let i = 0; i<this.core.length;i++){
				const ithItem = this.core.get(i);
				if(ithItem.equalsExcept(stack, "count")){
					if(reloading){
						if(shouldCapAt999){
							if(ithItem.count + stack.count > 999){
								// [confirmed] do not add new stack during loading save, if it would exceed 999
								return undefined;
							}
						}
					}else{
						// [needs confirm] if not reloading, cap the slot at 999
						const newCount = Math.min(999, ithItem.count+stack.count);
						if(newCount != ithItem.count){
							this.core.modifySlot(i, {count: newCount});
						}

						return undefined;
					}

				}
			}

		}

		// [confirmed] this check does not happen if mCount = 0 (which is covered by i!==-1 line below)
		// unrepeatable check: if a (unstackable) key item or master sword already exists in the first tab, do not add
		if(!stack.item.repeatable) {// only unstackable key items and master sword is not repeatable
			const [firstTabItem, firstTabIndex] = this.core.findFirstTab(stack.item.type, mCount);
			if(firstTabItem){
				for(let i=firstTabIndex;i<this.core.length && this.core.get(i).item.type === stack.item.type;i++){
					if(this.core.get(i).item === stack.item){
						// Found the key item/master sword, do not add
						return undefined;
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
				const equippedItems = this.getView().filter(s=>
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
						const [firstTabItem, firstTabIndex] = this.core.findFirstTab(ItemType.Arrow, mCount);
						if(firstTabItem){
							for(let i = firstTabIndex;i<this.core.length && this.core.get(i).item.type === ItemType.Arrow;i++){
								this.core.modifySlot(i, {equipped: false});
							}
						}
					}
				}
			}
		}
		
		// [no test coverage] limit check - detail too complicated, only basic case for wmc for now
		if(reloading){
			if(!stack.item.stackable && stack.item.type === ItemType.Food){
				const max = ItemMaxes[ItemType.Food];
				const current = this.core.getView().filter(stack=>stack.item.type===ItemType.Food).length;
				if(current >= max){
					return undefined;
				}
			}
		}

		return this.core.addSlot(stack, mCount+1);
	}

	// this is for all types of item
	public equip(item: Item, slot: number, mCount: number) {
		let s = 0;
		// unequip same type in first tab
		const [firstTabItem, firstTabIndex] = this.core.findFirstTab(item.type, mCount);
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
		const type = item.type;
		if (type===ItemType.Arrow){
			return; // cannot unequip arrow
		}
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

	public addStackDirectly(stack: ItemStack) {
		this.core.addStackDirectly(stack);
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

}
