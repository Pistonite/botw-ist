import { getTabFromType, Item, ItemStack, ItemTab, ItemType, iterateItemTabs, MetaModifyOption } from "data/item";
import { arrayEqual, stableSort, inPlaceFilter, Ref, newRef } from "data/util";

// This is the "core" of Slots with basic getter and manipulation methods
export class SlotsCore {
	// Internal array. Must guarantee that the reference doesn't change
	private internalSlots: Ref<ItemStack>[] = [];
	constructor(slots: ItemStack[]) {
		this.internalSlots = slots.map(newRef);
	}

	public getView(): ItemStack[] {
		return this.internalSlots.map(ref=>ref.get());
	}

	public get length(): number {
		return this.internalSlots.length;
	}

	public get(index: number): ItemStack {
		return this.internalSlots[index].get();
	}

	public getMatchingRefs(matchers: ((stack: ItemStack)=>boolean)[]): Ref<ItemStack>[][] {
		return matchers.map(match=>{
			return this.internalSlots.filter(ref=>match(ref.get()));
		});
	}

	public removeRefs(refs: Ref<ItemStack>[]) {
		inPlaceFilter(this.internalSlots, ref=>!refs.includes(ref));
	}

	public swap(i: number, j: number) {
		if(i < 0 || j < 0 || i >= this.internalSlots.length || j >= this.internalSlots.length){
			return;
		}

		const temp = this.internalSlots[i];
		this.internalSlots[i] = this.internalSlots[j];
		this.internalSlots[j] = temp;
	}

	// Used to decide if game data is synced with inventory
	// Two Slots are equal if the ItemStacks equal, including metadata equality
	public equals(other: SlotsCore): boolean {
		return arrayEqual(this.getView(), other.getView());
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
			const itemA = a.get().item;
			const itemB = b.get().item;
			if(itemA.type === ItemType.Arrow && itemB.type === ItemType.Arrow){
				return itemA.sortOrder - itemB.sortOrder;
			}
			if(itemA.tab === itemB.tab && itemA.tab === ItemTab.Bow){
				// arrows are always after bow
				return itemA.type - itemB.type;
			}
			// otherwise sort by tab
			return itemA.tab - itemB.tab;
		});
	}

	public clearFirst(count: number) {
		this.internalSlots.splice(0, count);
	}

	public addStackDirectly(stack: ItemStack, index?: number): Ref<ItemStack> {
		if(!index || index < 0 || index > this.internalSlots.length){
			index = this.internalSlots.length;
		}
		const newStackRef = newRef(stack);
		if(index === this.internalSlots.length){
			this.internalSlots.push(newStackRef);
		}else{
			this.internalSlots.splice(index, 0, newStackRef);
		}

		return newStackRef;
	}

	public addSlot(stack: ItemStack, mCount: number | null) {
		const newStackRef = this.addStackDirectly(stack);
		this.sortItemByTab(mCount);
		return newStackRef;
	}

	public removeZeroStackExceptArrowsAndMasterSword(): void {
		inPlaceFilter(this.internalSlots, (ref)=>{
			const {item, count} = ref.get();
			return item.type === ItemType.Arrow || item.id === "MasterSword" || count > 0;
		});
	}

	public modifySlot(i: number, option: Partial<ItemStack>) {
		this.internalSlots[i].set(this.internalSlots[i].get().modify(option));
	}

	public findLastEquipped(type: ItemType): Ref<ItemStack> | undefined{
		let i = 0;
		let result = undefined;
		// [needs confirm] does this check entire inventory?
		for(;i<this.internalSlots.length;i++){
			const {item, equipped} = this.internalSlots[i].get();
			// [needs confirm] does this break when == type+1?
			// [needs confirm] does this matter when tabs are undiscovered?
			if(item.type > type+1){
				break;
			}
			if(equipped && item.type === type){
				// constantly update result as long as a new slot is found
				// In the end, this will be the last equipped slots of that type
				result = this.internalSlots[i];
			}
		}
		// will be undefined if never found
		return result;
	}

	// set item metadata
	public setMetadata(item: Item, slot: number, meta: MetaModifyOption) {
		let s = 0;
		for(let i = 0; i<this.internalSlots.length;i++){
			const stack = this.internalSlots[i].get();
			if(stack.item === item){
				if(s<slot){
					// find the right slot
					s++;
				}else{
					this.internalSlots[i].set(stack.modifyMeta(meta));
					break;
				}
			}
		}
	}

	public removeAll(types: ItemType[]) {
		inPlaceFilter(this.internalSlots, ref=>!types.includes(ref.get().item.type));
		this.removeZeroStackExceptArrowsAndMasterSword();
	}

	public unequipAll(types: ItemType[]) {
		this.internalSlots.forEach((ref)=>{
			const stack = ref.get();
			if(stack.equipped && types.includes(stack.item.type)){
				ref.set(stack.modify({equipped: false}));
			}
		});
	}

	public findFirstTab(type: ItemType, listHeadsInit: boolean): [Ref<ItemStack> | undefined, number] {
		// figure out the tabs first
		const tabArray: [ItemTab, number][] = [];
		const tabAdded = new Set();
		if(listHeadsInit){
			// scan inventory array for tabs
			let lastTab = ItemTab.None;
			for(let i =0;i<this.internalSlots.length;i++){
				const currentItemTab = this.internalSlots[i].get().item.tab;
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

		// then find tab of that type
		// if type is arrow, find the first arrow in that tab
		const tabToFind = getTabFromType(type);
		let foundTabItemIndex = -1;
		for(let i =0;i<tabArray.length;i++){
			const [tab, ref] = tabArray[i];
			if(tab === tabToFind){
				foundTabItemIndex = ref;
				break;
			}
		}
		if(foundTabItemIndex < 0){
			return [undefined, -1]; // not found
		}
		if(type === ItemType.Arrow){
			// [confirmed] even if there are weapons in between bows, the check will continue to find the arrow
			// https://github.com/zeldaret/botw/blob/9d3bc8cfe1c4ddd74c3b072bbe6418665aa06de1/src/Game/UI/uiPauseMenuDataMgr.cpp#L1215
			for(;foundTabItemIndex<this.internalSlots.length; foundTabItemIndex++){
				const type = this.internalSlots[foundTabItemIndex].get().item.type;
				if(type > ItemType.Arrow){
					// arrow not found
					return [undefined, -1];
				}
				if(type === ItemType.Arrow){
					break;
				}
			}
		}
		return [this.internalSlots[foundTabItemIndex], foundTabItemIndex];
	}

	public isTabDiscovered(_tab: ItemTab): boolean {
		// [confirmed] kinak: tabs are different when a tab is discovered vs not
		// https://discord.com/channels/269611402854006785/269616041435332608/997547358206300231
		// for now we assume tabs are always already discovered
		return true;
	}

	// public findFirstIndex(tab: ItemTab): number {
	// 	for(let i=0;i<this.internalSlots.length;i++){
	// 		if(this.internalSlots[i].item.tab === tab){
	// 			return i;
	// 		}
	// 	}
	// 	return -1;
	// }
}
