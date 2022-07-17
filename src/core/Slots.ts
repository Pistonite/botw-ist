import { stableSort } from "data/stableSort";
import { getTabFromType, Item, ItemStack, ItemTab, ItemType, iterateItemTabs, MetaOption } from "data/item";

/*
 * This is the data model common to GameData and VisibleInventory
 * All branches in public interface should have comment with [confirmed] or [need confirm] indicating whether something is confirmed to be the same in game
 * cases tagged with [confirmed] must also have unit tests covering them
 * make sure to add unit tests when changing from [needs confirm] to [confirmed]
 */
export class Slots {
	private internalSlots: ItemStack[] = [];
	constructor(slots: ItemStack[]) {
		this.internalSlots = slots;
	}
	public getSlotsRef(): ItemStack[] {
		return this.internalSlots;
	}
	public deepClone(): Slots {
		// ItemStack is immutable so they do not need to be copied
		return new Slots([...this.internalSlots]);
	}

	public get length(): number {
		return this.internalSlots.length;
	}

	// Sort the item types as they appear in game. Arrows are also sorted amongst each other
	// Individual tabs are not sorted
	// input mCount = null will skip the optimization. Otherwise if mCount <= 1, do nothing
	public sortItemByTab(mCount: number | null) {
		if(mCount === null){
			mCount = this.internalSlots.length;
		}
		if(mCount <= 1){
			return;
		}
		stableSort(this.internalSlots, (a,b)=>{
			//const aData = itemToItemData(a.item);
			//const bData = itemToItemData(b.item);
			if(a.item.type === ItemType.Arrow && b.item.type === ItemType.Arrow){
				return a.item.sortOrder - b.item.sortOrder;
			}
			if(a.item.tab === b.item.tab && a.item.tab === ItemTab.Bow){
				// arrows are always after bow
				return a.item.type - b.item.type;
			}
			// otherwise sort by tab
			return a.item.tab - b.item.tab;
		});
	}

	public clearFirst(count: number) {
		this.internalSlots.splice(0, count);
	}

	public addStackDirectly(stack: ItemStack): number {
		this.internalSlots.push(stack);
		return 1;
	}
	public addSlot(stack: ItemStack, mCount: number | null) {
		this.internalSlots.push(stack);
		this.sortItemByTab(mCount);
	}

	// remove item(s) start from slot
	// return number of slots removed
	// if item stack can't be matched exactly, will try to match without metadata
	// pass negative as count to remove all
	public remove(toRemove: ItemStack, slot: number): number {
		const oldLength = this.internalSlots.length;
		let count = toRemove.count;
		let s = 0;
		let found = false;
		for(let i = 0; i<this.internalSlots.length && count !== 0;i++){
			const stack = this.internalSlots[i];
			if(stack.canStack(toRemove)){
				found = true;
				if(s<slot){
					// find the right slot
					s++;
				}else{
					if(count<0 || stack.count<count){
						// this stack not enough to remove all
						count-=stack.count;
						this.internalSlots[i] = stack.modify({count:0});
                        
					}else{
						this.internalSlots[i] = stack.modify({count:stack.count-count});
						break;
					}
				}
			}
		}
		if(!found){
			for(let i = 0; i<this.internalSlots.length && count > 0;i++){
				const stack = this.internalSlots[i];
				if(stack.item === toRemove.item){
					found = true;
					if(s<slot){
						// find the right slot
						s++;
					}else{
						if(stack.count<count){
							// this stack not enough to remove all
							count-=stack.count;
							this.internalSlots[i] = stack.modify({count:0});
                            
						}else{
							this.internalSlots[i] = stack.modify({count:stack.count-count});
							break;
						}
					}
				}
			}
		}
		this.removeZeroStackExceptArrows();
		return oldLength-this.internalSlots.length;
	}

	removeZeroStackExceptArrows(): void {
		this.internalSlots = this.internalSlots.filter(({item, count})=>{
			return item.type === ItemType.Arrow || count > 0;
		});
	}

	// Add something to inventory in game
	// returns number of slots added
	public add(stack: ItemStack, reloading: boolean, mCount: number | null): number {
		if(mCount === null){
			mCount = this.internalSlots.length;
		}

		// If item is stackable (arrow, material, spirit orbs), do 999 Cap Check
		// [confirmed] the 999 cap check always happens, even when mCount = 0
		// https://discord.com/channels/269611402854006785/269616041435332608/997404941754839060
		if(stack.item.stackable){
			let shouldCapAt999 = true;
			// [confirmed] kinak: for arrow, if there's "no arrow", 999 check is skipped
			if(stack.item.type === ItemType.Arrow){
				// [needs confirm] index will be -1 if mCount is 0, since we won't find any tabs, so arrow check is skipped regardless
				const firstArrowIndex = this.findFirstTabIndex(ItemType.Arrow, mCount);
				if(firstArrowIndex === -1){
					shouldCapAt999 = false;
				}
			}
			// [confirmed] 999 check (i.e. merge check) scans entire inventory
			// https://discord.com/channels/269611402854006785/269616041435332608/997764628492865572
			// [needs confirm] arrow special case works not during reload? for now, does not consider arrow case when not reloading
			// Check if there's already a slot, if so, add it to that and cap it at 999
			for(let i = 0; i<this.internalSlots.length;i++){
				if(this.internalSlots[i].canStack(stack)){
					if(reloading){
						if(shouldCapAt999){
							if(this.internalSlots[i].count + stack.count > 999){
								// [confirmed] do not add new stack during loading save, if it would exceed 999
								return 0;
							}
						}
					}else{
						// [needs confirm] if not reloading, cap the slot at 999
						const newCount = Math.min(999, this.internalSlots[i].count+stack.count);
						if(newCount != this.internalSlots[i].count){
							this.internalSlots[i] = this.internalSlots[i].modify({count: newCount});
						}
                        
						return 0;
					}
                    
				}
			}
            
		}

		// [confirmed] this check does not happen if mCount = 0 (which is covered by i!==-1 line below)
		// unrepeatable check: if a (unstackable) key item or master sword already exists in the first tab, do not add
		if(!stack.item.repeatable) {// only unstackable key items and master sword is not repeatable
			let i = this.findFirstTabIndex(stack.item.type, mCount);
			if(i!==-1){
				for(;i<this.internalSlots.length && this.internalSlots[i].item.type === stack.item.type;i++){
					if(this.internalSlots[i].item === stack.item){
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
				const equippedItems = this.internalSlots.filter(s=>
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
						let i = this.findFirstTabIndex(ItemType.Arrow, mCount);
						if(i!==-1){
							for(;i<this.internalSlots.length && this.internalSlots[i].item.type === ItemType.Arrow;i++){
								this.internalSlots[i] = this.internalSlots[i].modify({equipped: false});
							}
						}
					}
				}
			}
		}
        
		this.addSlot(stack, mCount+1);
		return 1;
	}

	// this is for all types of item
	public equip(item: Item, slot: number, mCount: number) {
		let s = 0;
		// unequip same type in first tab
		let i = this.findFirstTabIndex(item.type, mCount);
		if(i!==-1){
			for(;i<this.internalSlots.length && this.internalSlots[i].item.tab === item.tab;i++){
				if(this.internalSlots[i].item.type === item.type){
					this.modifySlot(i, {equipped: false});
				}
			}
		}
        
		// now search for the one the player selects and equip it
		for(let i = 0; i<this.internalSlots.length;i++){
			if(this.internalSlots[i].item === item){
				if (s===slot){
					this.modifySlot(i, {equipped: true});
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
		for(let i = 0; i<this.internalSlots.length;i++){
			if(this.internalSlots[i].item === item){
				if(slot < 0){
					if(this.internalSlots[i].equipped){
						this.modifySlot(i, {equipped: false});
						break;
					}
				}else{
					if(s<slot){
						s++;
					}else{
						this.modifySlot(i, {equipped: false});
						break;
					}
				}
			}
		}
	}

	public updateLife(life: number, slot: number) {
		if(slot < 0 || slot >= this.internalSlots.length){
			return;
		}
		if(this.internalSlots[slot].item.stackable && this.internalSlots[slot].item.type !== ItemType.Arrow){
			life = Math.min(999, life);
		}
		//const thisData = itemToItemData(this.internalSlots[slot].item);
		// Currently only supports corrupting arrows, material, food and key items as durability values are not simulated on equipments
		//if(this.internalSlots[slot].item.type >= ItemType.Material || this.internalSlots[slot].item.stackable){
		//const newLife = Math.min(999, life);
		this.modifySlot(slot, {count: life});
		//}
	}

	// shoot count arrows. return the slot that was updated, or -1
	public shootArrow(count: number): number {
		// first find equipped arrow, search entire inventory
		// this is the last equipped arrow before armor
		let i=0;
		let equippedArrow: Item | undefined = undefined;
		// [needs confirm] does this check entire inventory?
		for(;i<this.internalSlots.length;i++){
			if(this.internalSlots[i].item.type > ItemType.Shield){
				break;
			}
			if(this.internalSlots[i].equipped && this.internalSlots[i].item.type === ItemType.Arrow){
				equippedArrow = this.internalSlots[i].item;
			}
		}
		if(i>=this.internalSlots.length){
			//can't find equipped arrow
			return -1;
		}
		// now find the first slot of that arrow and update
		for(let j=0;j<this.internalSlots.length;j++){
			if(this.internalSlots[j].item === equippedArrow){
				this.modifySlot(j, {count: Math.max(0, this.internalSlots[j].count-count)});
				return j;
			}
		}
		//for some reason cannot find that arrow now?
		return -1;

	}

	// set item metadata
	public setMetadata(item: Item, slot: number, meta: MetaOption) {
		let s = 0;
		for(let i = 0; i<this.internalSlots.length;i++){
			const stack = this.internalSlots[i];
			if(stack.item === item){
				if(s<slot){
					// find the right slot
					s++;
				}else{
					this.internalSlots[i] = stack.modifyMeta(meta);
					break;
				}
			}
		}
	}

	// return how many slots are removed
	public clearAllButKeyItems(): number {
		const newslots = this.internalSlots.filter(stack=>stack.item.type === ItemType.Key);
		const removedCount = this.internalSlots.length - newslots.length;
		this.internalSlots = newslots;
		return removedCount;
	}

	public findFirstTabIndex(type: ItemType, mCount: number): number {
		// figure out the tabs first
		const tabArray: [ItemTab, number][] = [];
		const tabAdded = new Set();
		if(mCount !== 0){
			// scan inventory array for tabs
			let lastTab = ItemTab.None;
			for(let i =0;i<this.internalSlots.length;i++){
				const currentItemTab = this.internalSlots[i].item.tab;
				if(currentItemTab != lastTab){
					// add missing empty tabs if already discovered
					iterateItemTabs().filter(t=>t<currentItemTab).forEach(t=>{
						if(!tabAdded.has(t) && this.isTabDiscovered(t)){
							tabArray.push([t, -1]);
							tabAdded.add(t);
						}
					});
					// add new tab
					tabArray.push([currentItemTab, i]);
					tabAdded.add(currentItemTab);
					lastTab = currentItemTab;
				}
			}
		}else{
			// just add discovered tabs
			iterateItemTabs().forEach(t=>{
				if(this.isTabDiscovered(t)){
					tabArray.push([t, -1]);
				}
			});
		}
        
		// first first tab of that type
		// if type is arrow, find the first arrow in that tab
		const tabToFind = getTabFromType(type);
		let foundTabItemIndex = -1;
		for(let i =0;i<tabArray.length;i++){
			const [tab, itemIndex] = tabArray[i];
			if(tab === tabToFind){
				foundTabItemIndex = itemIndex;
				break;
			}
		}
		if(type === ItemType.Arrow && foundTabItemIndex !== -1){
			// [confirmed] even if there are weapons in between bows, the check will continue to find the arrow
			// https://github.com/zeldaret/botw/blob/9d3bc8cfe1c4ddd74c3b072bbe6418665aa06de1/src/Game/UI/uiPauseMenuDataMgr.cpp#L1215
			for(;foundTabItemIndex<this.internalSlots.length; foundTabItemIndex++){
				if(this.internalSlots[foundTabItemIndex].item.type > ItemType.Arrow){
					// arrow not found
					return -1;
				}
				if(this.internalSlots[foundTabItemIndex].item.type === ItemType.Arrow){
					return foundTabItemIndex;
				}
			}
		}
		return foundTabItemIndex;
	}

	isTabDiscovered(_tab: ItemTab): boolean {
		// [confirmed] kinak: tabs are different when a tab is discovered vs not
		// https://discord.com/channels/269611402854006785/269616041435332608/997547358206300231
		// for now we assume tabs are always already discovered
		return true;
	}

	modifySlot(i: number, option: Partial<ItemStack>) {
		this.internalSlots[i] = this.internalSlots[i].modify(option);
	}

}
