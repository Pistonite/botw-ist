import { arrayEqual, stableSort, inPlaceFilter } from "data/util";
import { getTabFromType, Item, ItemStack, ItemTab, ItemType, iterateItemTabs, MetaOption } from "data/item";

// This is the "core" of Slots with basic getter and manipulation methods
export class SlotsCore {
    // Internal array. Must guarantee that the reference doesn't change
	public internalSlots: ItemStack[] = [];
	constructor(slots: ItemStack[]) {
		this.internalSlots = slots;
	}

	// Used to decide if game data is synced with inventory
	// Two Slots are equal if the ItemStacks equal, including metadata equality
	public equals(other: SlotsCore): boolean {
		return arrayEqual(this.internalSlots, other.internalSlots);
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

	public addStackDirectly(stack: ItemStack) {
		this.internalSlots.push(stack);
	}

	public addSlot(stack: ItemStack, mCount: number | null) {
		this.internalSlots.push(stack);
		this.sortItemByTab(mCount);
	}

	public removeZeroStackExceptArrows(): void {
		inPlaceFilter(this.internalSlots, ({item, count})=>{
			return item.type === ItemType.Arrow || count > 0;
		});
	}

    public modifySlot(i: number, option: Partial<ItemStack>) {
		this.internalSlots[i] = this.internalSlots[i].modify(option);
	}

	public findLastEquippedSlot(type: ItemType): number {
		let i = 0;
		let result = -1;
		// [needs confirm] does this check entire inventory?
		for(;i<this.internalSlots.length;i++){
			// [needs confirm] does this break when == type+1?
			// [needs confirm] does this matter when tabs are undiscovered?
			if(this.internalSlots[i].item.type > type+1){
				break;
			}
			if(this.internalSlots[i].equipped && this.internalSlots[i].item.type === type){
				// constantly update result as long as a new slot is found
				// In the end, this will be the last equipped slots of that type
				result = i;
			}
		}
		// will be -1 if never found
		return result;
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

	public clearAllButKeyItems() {
		inPlaceFilter(this.internalSlots, stack=>stack.item.type === ItemType.Key);
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

	public isTabDiscovered(_tab: ItemTab): boolean {
		// [confirmed] kinak: tabs are different when a tab is discovered vs not
		// https://discord.com/channels/269611402854006785/269616041435332608/997547358206300231
		// for now we assume tabs are always already discovered
		return true;
	}
}


