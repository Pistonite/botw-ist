import { Item, ItemStack, itemToItemData, ItemType, ItemTypes } from "./Item";
import { Slots } from "./Slots";



export class Inventory {
	private slots: Slots = new Slots([]);
	private savedSlots: Slots = new Slots([]);
	private namedSlots: {[name: string]: Slots} = {};
	private numBroken = 0;
	private isInitialSort = false;
	private isAltered = true;
	private inaccurate = false;
	private turnedInOrbs = 0;
	public deepClone(): Inventory {
		const other = new Inventory();
		other.slots = this.slots.deepClone();
		other.savedSlots = this.savedSlots.deepClone();
		other.numBroken = this.numBroken;
		other.isInitialSort = this.isInitialSort;
		other.isAltered = this.isAltered;
		other.inaccurate = this.inaccurate;
		other.turnedInOrbs = this.turnedInOrbs;
		other.namedSlots = {};
		for(const name in this.namedSlots){
			other.namedSlots[name] = this.namedSlots[name].deepClone();
		}
		return other;
	}

	public getSlots(): Slots {
		return this.slots;
	}

	public getSavedSlots(): Slots {
		return this.savedSlots;
	}

	public getNumBroken(): number {
		return this.numBroken;
	}

	public isInaccurate(): boolean {
		return this.inaccurate;
	}

	public getTurnedInOrbs(): number {
		return this.turnedInOrbs;
	}

	public init(stacks: ItemStack[]) {
		this.slots = new Slots([]);
		stacks.forEach(s=>{
			this.slots.add(s.item, s.count)
		});
		this.numBroken = 0;
		this.isInitialSort = false;
		this.isAltered = true;
		this.inaccurate = false;
	}

	public closeGame() {
		this.numBroken = 0;
		this.isInitialSort = false;
		this.isAltered = true;
		this.inaccurate = false;
		this.slots = new Slots([]);
	}

	public addBrokenSlots(num: number) {
		this.numBroken+=num;
	}

	public setTag(name: string){
		this.namedSlots[name] = this.savedSlots.deepClone();
	}

	public applyTag(name: string){
		if(name in this.namedSlots){
			this.savedSlots = this.namedSlots[name].deepClone();
		}else{
			this.savedSlots = new Slots([]);
		}
	}

	public save() {
		if(this.isAltered){
			this.savedSlots = this.slots.deepClone();
		}
		// Inventory Corruption
		// get durability transfer slots
		const durabilityTransferSlots: number[] = [];
		const equippedWeapon = this.slots.getFirstEquippedSlotIndex(ItemType.Weapon);
		if(equippedWeapon>=0){
			durabilityTransferSlots.push(equippedWeapon);
		}
		const equippedBow = this.slots.getFirstEquippedSlotIndex(ItemType.Bow);
		if(equippedBow>=0){
			durabilityTransferSlots.push(equippedBow);
		}
		const equippedShield = this.slots.getFirstEquippedSlotIndex(ItemType.Shield);
		if(equippedShield>=0){
			durabilityTransferSlots.push(equippedShield);
		}
		durabilityTransferSlots.forEach(s=>{
			if(s<this.savedSlots.length){
				// We ignore the case where durability transfer happens from equipment to equipment

				if(itemToItemData(this.savedSlots.get(s).item).stackable){
					this.savedSlots.get(s).count = 999;
				}
			}
		})
	}

	public reload() {
		
		// get things to dupe
		const dupeMap: {[k in ItemType]: Slots} = {
			[ItemType.Weapon]: new Slots([]),
			[ItemType.Bow]: new Slots([]),
			[ItemType.Arrow]: new Slots([]),
			[ItemType.Shield]: new Slots([]),
			[ItemType.Material]: new Slots([]),
			[ItemType.Meal]: new Slots([]),
			[ItemType.Key]: new Slots([])
		};
		for(let i=Math.max(0, this.slots.length-this.numBroken);i<this.slots.length;i++){
			const stack = this.slots.get(i);
			const itemData = itemToItemData(stack.item);
			dupeMap[itemData.type].addStackCopy(stack);
		}
		// apply dupe
		//console.log(dupeMap);
		this.slots = new Slots([]);
		// const dupeType = (type: ItemType) => {
			
		// }
		ItemTypes.forEach(type=>{
			this.slots.addSlotsToEnd(dupeMap[type]);
			this.slots.addSlotsToEnd(this.savedSlots.getByType(type).deepClone());
		});

		this.slots.sortArrows();
		this.isInitialSort = true;
		this.isAltered = false;
	}

	public sortKey() {
		const nonKeyItems = this.slots.getBeforeType(ItemType.Key);
		const keyItems = this.slots.getByType(ItemType.Key);
		keyItems.sort();
		nonKeyItems.addSlotsToEnd(keyItems);
		this.slots = nonKeyItems;
		this.isAltered=true;
		this.isInitialSort=false;
	}

	public sortMaterial() {
		const beforeMaterial = this.slots.getBeforeType(ItemType.Material);
		const afterMaterial = this.slots.getAfterType(ItemType.Material);
		const materials = this.slots.getByType(ItemType.Material);
		if(this.isInitialSort){
			// the materials in broken slots are not sorted
			const brokenSlots = Math.max(0, this.numBroken - afterMaterial.length);
			const noSortPart = materials.removeFromEnd(brokenSlots);
			materials.sort();
			beforeMaterial.addSlotsToEnd(materials);
			beforeMaterial.addSlotsToEnd(noSortPart);
			beforeMaterial.addSlotsToEnd(afterMaterial);
		}else{
			materials.sort();
			beforeMaterial.addSlotsToEnd(materials);
			beforeMaterial.addSlotsToEnd(afterMaterial);
		}
		
		this.slots = beforeMaterial;
		this.isInitialSort = false;
		this.isAltered=true;
	}

	public remove(item: Item, count: number, slot: number) {
		this.slots.remove(item, count, slot);
		if(item===Item.SpiritOrb){
			this.turnedInOrbs+=count;
		}
		this.isAltered=true;
	}

	public add(item: Item, count: number) {
		this.slots.add(item, count);
		if(itemToItemData(item).type===ItemType.Arrow){
			this.slots.sortArrows();
		}
		this.isAltered=true;
	}

	public equipEquipmentOrArrow(item: Item, slot: number) {
		this.slots.equip(item, slot);
		this.isAltered=true;
	}

	public unequipEquipment(item: Item, slot: number){
		this.slots.unequip(item, slot);
		this.isAltered=true;
	}

	public shootArrow(item: Item, count: number){
		this.slots.shoot(item, count);
		this.isAltered=true;
	}

}
