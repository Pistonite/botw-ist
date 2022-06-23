import { stableSort } from "data/mergeSort";
import { Item, ItemStack, itemToItemData, ItemType } from "./Item";

/*
 * This is the data model common to GameData and VisibleInventory
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
		return new Slots(this.internalSlots.map(s=>({...s})));
	}
	public get length(): number {
		return this.internalSlots.length;
	}

	// Sort the item types as they appear in game. Arrows are also sorted amongst each other
	// input mCount = null will skip the optimization. Otherwise if mCount <= 1, do nothing
	public sortItemType(mCount: number | null) {
		if(mCount === null){
			mCount = this.internalSlots.length;
		}
		if(mCount <= 1){
			return;
		}
		stableSort(this.internalSlots, (a,b)=>{
			const aData = itemToItemData(a.item);
			const bData = itemToItemData(b.item);
			if(aData.type === ItemType.Arrow && bData.type === ItemType.Arrow){
				return aData.sortOrder - bData.sortOrder;
			}
			return aData.type - bData.type;
		});
	}

	public clearFirst(count: number) {
		this.internalSlots.splice(0, count);
	}

	public addStackDirectly(stack: ItemStack): number {
		const data = itemToItemData(stack.item);
		if(data.stackable){
			this.internalSlots.push({...stack});
			return 1;
		}
		for(let i=0;i<stack.count;i++){
			this.internalSlots.push({...stack, count: 1});
		}
		return stack.count;
	}
	public addSlot(stack: ItemStack, mCount: number | null) {
		this.internalSlots.push({...stack});
		this.sortItemType(mCount);
	}

	// remove item(s) start from slot
	// return number of slots removed
	public remove(item: Item, count: number, slot: number): number {
		const oldLength = this.internalSlots.length;
		let s = 0;
		for(let i = 0; i<this.internalSlots.length && count > 0;i++){
			if(this.internalSlots[i].item === item){
				if(s<slot){
					s++;
				}else{
					if(this.internalSlots[i].count<count){
						count-=this.internalSlots[i].count;
						this.internalSlots[i].count=0;
						
					}else{
						this.internalSlots[i].count-=count;
						break;
					}
					
				}
			}
		}
		
		this.internalSlots = this.internalSlots.filter(({count})=>count>0);
		return oldLength-this.internalSlots.length;
	}

	// Add something to inventory in game
	// returns number of slots added
	public add(item: Item, count: number, equippedDuringReload: boolean, reloading: boolean, mCount: number | null): number {
		if(mCount === null){
			mCount = this.internalSlots.length;
		}
		//let added = false;
		const data = itemToItemData(item);
		// If item is stackable (arrow, material, spirit orbs)
		// Check if there's already a slot, if so, add it to that and cap it at 999
		if(data.stackable){
			for(let i = 0; i<this.internalSlots.length;i++){
				if(this.internalSlots[i].item === item){
					if(reloading){
						if(this.internalSlots[i].count + count > 999){
							// do not add new stack during loading save, if it would exceed 999
							return 0;
						}
						// Otherwise add the stack directly
						this.addSlot({item, count, equipped: equippedDuringReload}, mCount+1);
						return 1;
					}
					this.internalSlots[i].count = Math.min(999, this.internalSlots[i].count+count);
					return 0;
				}
			}
		}
		// Need to add new slot
		// Key item check: if the key item or master sword already exists in the first tab, do not add
		if(mCount != 0){
			if(data.type === ItemType.Key || item === Item.MasterSword) {
				let i=0;
				while(i<this.internalSlots.length && itemToItemData(this.internalSlots[i].item).type < data.type){
					i++;
				}
				for(;i<this.internalSlots.length && itemToItemData(this.internalSlots[i].item).type === data.type;i++){
					if(this.internalSlots[i].item === item){
						// Found the key item/master sword, do not add
						return 0;
					}
				}
			}
		}
		
		// Checks finish, do add new slot
		if(data.stackable){
			if(reloading){
				this.addSlot({item,count,equipped:equippedDuringReload}, mCount+1);
			}else{
				if(data.type===ItemType.Arrow){
					// if currently equipped arrow == 0. new arrows are equiped
					// TODO: botw needs more testing on how arrows are handled in various cases
					const shouldEquipNew = this.internalSlots.filter(s=>{
						const sData = itemToItemData(s.item);
						return sData.type === data.type && s.equipped && s.count > 0;
					}).length === 0;
					this.addSlot({item,count,equipped:shouldEquipNew}, mCount+1);
				}else{
					this.addSlot({item,count,equipped:false}, mCount+1);
				}
			}
			
			return 1;
		}

		if(reloading){
			this.addSlot({item,count,equipped: equippedDuringReload}, mCount+1);
			return 1;
		}
		if(data.type===ItemType.Weapon || data.type===ItemType.Bow || data.type===ItemType.Shield){
			//Check equip
			const shouldEquipNew = this.internalSlots.filter(s=>{
				const sData = itemToItemData(s.item);
				return sData.type === data.type && s.equipped;
			}).length === 0;
			this.addSlot({item,count:1,equipped: shouldEquipNew}, mCount+1);
			for(let i=1;i<count;i++){
				this.addSlot({item,count:1,equipped: false}, mCount+i+1);

			}
		}else{
			for(let i=0;i<count;i++){
				this.addSlot({item,count:1,equipped: false}, mCount+i+1);
			}
		}

		return count;
	}

	// this is for both equipments and arrows
	public equip(item: Item, slot: number) {
		let s = 0;
		// unequip same type in first tab
		const type = itemToItemData(item).type;
		let i=0;
		while(i<this.internalSlots.length && itemToItemData(this.internalSlots[i].item).type < type){
			i++;
		}
		for(;i<this.internalSlots.length && itemToItemData(this.internalSlots[i].item).type === type;i++){
			this.internalSlots[i].equipped = false;
		}
		// now search for the one the player selects and equip it
		for(let i = 0; i<this.internalSlots.length;i++){
			if(this.internalSlots[i].item === item){
				if (s===slot){
					this.internalSlots[i].equipped=true;
					break;
				}
				s++;
			}
		}
	}
	public unequip(item:Item, slot: number) {
		let s = 0;
		const type = itemToItemData(item).type;
		if (type===ItemType.Arrow){
			return; // cannot unequip arrow
		}
		for(let i = 0; i<this.internalSlots.length;i++){
			if(this.internalSlots[i].item === item){
				if(slot < 0){
					if(this.internalSlots[i].equipped){
						this.internalSlots[i].equipped=false;
						break;
					}
				}else{
					if(s<slot){
						s++;
					}else{
						this.internalSlots[i].equipped=false;
						break;
					}
				}
			}
		}
	}

	public corrupt(durability: number, slot: number) {
		if(slot < 0 || slot >= this.internalSlots.length){
			return;
		}
		const thisData = itemToItemData(this.internalSlots[slot].item);
		// Currently only supports corrupting arrows, material, meal and key items as durability values are not simulated on equipments
		if(thisData.type >= ItemType.Material || thisData.stackable){
			this.internalSlots[slot].count = durability;
		}
	}

	// shoot count arrows. return the slot that was updated, or -1
	public shootArrow(count: number): number {
		// first find equipped arrow, search entire inventory
		// this is the last equipped arrow before armor
		let i=0;
		let equippedArrow: Item | undefined = undefined;
		for(;i<this.internalSlots.length;i++){
			const data = itemToItemData(this.internalSlots[i].item);
			if(data.type > ItemType.Shield){
				break;
			}
			if(this.internalSlots[i].equipped && data.type === ItemType.Arrow){
				equippedArrow = data.item;
			}
		}
		if(i>=this.internalSlots.length){
			//can't find equipped arrow
			return -1;
		}
		// now find the first slot of that arrow and update
		for(let j=0;j<this.internalSlots.length;j++){
			if(this.internalSlots[j].item === equippedArrow){
				this.internalSlots[j].count = Math.max(0, this.internalSlots[j].count-count);
				return j;
			}
		}
		//for some reason cannot find that arrow now?
		return -1;

	}

}
