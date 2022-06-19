import { itemToType, getKeyItemSortOrder, shouldIgnoreOnReload, getMaterialSortOrder, Item, isStackable } from "./Item";
import { ItemStack, ItemType } from "./ItemStack";

export class Inventory {
	private slots: ItemStack[] = [];
	private savedSlots: ItemStack[] = [];
	private numBroken = 0;
	private isInitialSort = false;
	private isAltered = true;
	private isSaveAltered = true;
	private inaccurate = false;
	public clone(): Inventory {
		const other = new Inventory();
		other.slots = [...this.slots.map(stack=>({...stack}))];
		other.savedSlots = [...this.savedSlots.map(stack=>({...stack}))];
		other.numBroken = this.numBroken;
		other.isInitialSort = this.isInitialSort;
		other.isAltered = this.isAltered;
		other.isSaveAltered = this.isSaveAltered;
		other.inaccurate = this.inaccurate;
		return other;
	}

	public getSlots(): ItemStack[] {
		return this.slots;
	}

	public getNumBroken(): number {
		return this.numBroken;
	}

	public isInaccurate(): boolean {
		return this.inaccurate;
	}

	public init(stacks: ItemStack[]) {
		this.savedSlots = [...stacks.map((stack)=>({...stack}))];
		this.slots = [...stacks.map((stack)=>({...stack}))];
		this.numBroken = 0;
		this.isInitialSort = false;
		this.isAltered = true;
		this.isSaveAltered = true;
		this.inaccurate = false;
	}

	public addBrokenSlots(num: number) {
		this.numBroken+=num;
	}

	public save() {
		this.isSaveAltered = this.isAltered;
		this.savedSlots = [...this.slots];
	}

	public reload() {
		if(!this.isSaveAltered){
			this.inaccurate = true;
		}
		// get things to dupe
		const dupeMap: {[k in ItemType]: ItemStack[]} = {
			[ItemType.Material]: [],
			[ItemType.Meal]: [],
			[ItemType.Key]: []
		};
		for(let i=Math.max(0, this.slots.length-this.numBroken);i<this.slots.length;i++){
			const stack = this.slots[i];
			if(!shouldIgnoreOnReload(stack.item)){
				dupeMap[itemToType(stack.item)].push(stack);
			}
		}
		// get materials, food, and key items
		const materials = this.savedSlots.filter(stack=>itemToType(stack.item)===ItemType.Material);
		const meals = this.savedSlots.filter(stack=>itemToType(stack.item)===ItemType.Meal);
		const keyItems = this.savedSlots.filter(stack=>itemToType(stack.item)===ItemType.Key);
		// apply dupe
		this.slots = [];
		// duped materials go to the left
		this.slots.push(...dupeMap[ItemType.Material].map(stack=>({...stack})));
		this.slots.push(...materials.map(stack=>({...stack})));
		this.slots.push(...dupeMap[ItemType.Meal].map(stack=>({...stack})));
		this.slots.push(...meals.map(stack=>({...stack})));
		// key items to the right
		this.slots.push(...keyItems.map(stack=>({...stack})));
		this.slots.push(...dupeMap[ItemType.Key].map(stack=>({...stack})));

		this.isInitialSort = true;
		this.isAltered = false;
		this.isSaveAltered = false;
	}

	public sortKey() {
		const nonKeyItems = this.slots.filter(stack=>itemToType(stack.item)!==ItemType.Key);
		const keyItems = this.slots.filter(stack=>itemToType(stack.item)===ItemType.Key);
		keyItems.sort((a,b)=>{
			return getKeyItemSortOrder(a.item) - getKeyItemSortOrder(b.item);
		});
		this.slots = [...nonKeyItems, ...keyItems];
		this.isAltered=true;
		this.isInitialSort=false;
	}

	public sortMaterial() {
		const nonMaterial = this.slots.filter(stack=>itemToType(stack.item)!==ItemType.Material);
		const materials = this.slots.filter(stack=>itemToType(stack.item)===ItemType.Material);
		if(this.isInitialSort){
			// the materials in broken slots are not sorted
			const brokenSlots = Math.max(0, this.numBroken - nonMaterial.length);
			const sortPart = materials.splice(0, materials.length-brokenSlots);
			sortPart.sort((a,b)=>{
				return getMaterialSortOrder(a.item) - getMaterialSortOrder(b.item);
			});
			this.slots = [...sortPart, ...materials, ...nonMaterial];
			this.isInitialSort = false;
		}else{
			materials.sort((a,b)=>{
				return getMaterialSortOrder(a.item) - getMaterialSortOrder(b.item);
			});
			this.slots = [...materials, ...nonMaterial];
		}
		this.isAltered=true;
	}

	public remove(item: Item, count: number, slot: number) {
		let s = 0;
		for(let i = 0; i<this.slots.length;i++){
			if(this.slots[i].item === item){
				if(s<slot){
					s++;
				}else{
					this.slots[i].count-=count;
					break;
				}
			}
		}
		this.slots = this.slots.filter(({count})=>count>0);
		this.isAltered=true;
	}

	public add(item: Item, count: number) {
		let added = false;
		if(isStackable(item)){
			for(let i = 0; i<this.slots.length;i++){
				if(this.slots[i].item === item){
					this.slots[i].count+=count;
					added = true;
					break;
				}
			}
		}
		if(!added){
			// add to the correct type
			switch(itemToType(item)){
				case ItemType.Material: {
					const materials = this.slots.filter(stack=>itemToType(stack.item)===ItemType.Material);
					this.slots.splice(materials.length, 0, {
						item, count
					});
					break;
				}
				case ItemType.Meal: {
					const keyItems = this.slots.filter(stack=>itemToType(stack.item)===ItemType.Key);
					this.slots.splice(-keyItems.length, 0, {
						item, count
					});
					break;
				}
				case ItemType.Key: {
					this.slots.push({
						item, count
					});
					break;
				}
			}
		}
		this.isAltered=true;
	}
}
