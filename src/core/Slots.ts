import { Item, ItemStack, itemToItemData, ItemType } from "./Item";

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
	public get(i: number): ItemStack{
		return this.internalSlots[i];
	}
	public getByType(type: ItemType): Slots {
		return new Slots(this.internalSlots.filter(s=>itemToItemData(s.item).type===type));
	}
	public getBeforeType(type: ItemType): Slots {
		return new Slots(this.internalSlots.filter(s=>itemToItemData(s.item).type<type));
	}
	public getAfterType(type: ItemType): Slots {
		return new Slots(this.internalSlots.filter(s=>itemToItemData(s.item).type>type));
	}
	public addSlotsToEnd(slots: Slots) {
		slots.internalSlots.forEach(s=>this.addStack(s));
	}
	public addStack(stack: ItemStack) {
		// Scan non-repeatables
		const data = itemToItemData(stack.item);
		if(!data.repeatable){
			for(let i=0;i<this.internalSlots.length;i++){
				if(this.internalSlots[i].item===stack.item){
					return;
				}
			}
		}
		this.internalSlots.push(stack);
	}
	public addStackCopy(stack: ItemStack) {
		this.addStack({...stack});
	}
	public sort() {
		this.internalSlots.sort((a,b)=>{
			return itemToItemData(a.item).sortOrder - itemToItemData(b.item).sortOrder;
		});
	}
	public removeFromEnd(count: number): Slots {
		const end = this.internalSlots.splice(-count, count);
		return new Slots(end);
	}
	public remove(item: Item, count: number, slot: number) {
		let s = 0;
		for(let i = 0; i<this.internalSlots.length;i++){
			if(this.internalSlots[i].item === item){
				if(s<slot){
					s++;
				}else{
					this.internalSlots[i].count-=count;
					break;
				}
			}
		}
		this.internalSlots = this.internalSlots.filter(({count})=>count>0);
	}

	public add(item: Item, count: number) {
		let added = false;
		const data = itemToItemData(item);
		if(data.stackable){
			for(let i = 0; i<this.internalSlots.length;i++){
				if(this.internalSlots[i].item === item){
					this.internalSlots[i].count+=count;
					added = true;
					break;
				}
			}
		}
		if(!added){
			const after = this.removeFromEnd(this.getAfterType(data.type).length);
            if(data.stackable){
                if(data.type===ItemType.Arrow){
                    // if currently equipped arrow == 0. new arrows are equiped
                    const shouldEquipNew = this.internalSlots.filter(s=>{
                        const sData = itemToItemData(s.item);
                        return sData.type === data.type && s.equipped && s.count > 0;
                    }).length === 0;
                    this.addStack({item,count,equipped:shouldEquipNew});
                }else{
                    this.addStack({item,count,equipped:false});
                }
                
            }else{
                if(data.type===ItemType.Weapon || data.type===ItemType.Bow || data.type===ItemType.Shield){
                    //Check equip
                    const shouldEquipNew = this.internalSlots.filter(s=>{
                        const sData = itemToItemData(s.item);
                        return sData.type === data.type && s.equipped;
                    }).length === 0;
                    this.addStack({item,count:1,equipped: shouldEquipNew});
                    for(let i=1;i<count;i++){
                        this.addStack({item,count:1,equipped: false});

                    }
                }else{
                    for(let i=0;i<count;i++){
                        this.addStack({item,count:1,equipped: false});
                    }
                }
                
            }

			this.addSlotsToEnd(after);
		}	
	}
    // this is for both equipments and arrows
    public equip(item: Item, slot: number) {
        let s = 0;
        const type = itemToItemData(item).type;
        const filtered = this.internalSlots.filter(s=>itemToItemData(s.item).type === type);
		for(let i = 0; i<filtered.length;i++){
            filtered[i].equipped=false;
			if(filtered[i].item === item){
				if (s===slot){
					filtered[i].equipped=true;
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

    // Difference between shoot and remove:
    // 1. can only be from first (leftmost) slot
    // 2. empty slots not removed
    public shoot(item: Item, count: number) {
        for(let i = 0; i<this.internalSlots.length;i++){
			if(this.internalSlots[i].item === item){
				this.internalSlots[i].count-=count;
			}
		}
    }

    public sortArrows() {
        const after = this.removeFromEnd(this.getAfterType(ItemType.Arrow).length);
        const arrows = this.removeFromEnd(this.getByType(ItemType.Arrow).length);
        arrows.sort();
        this.addSlotsToEnd(arrows);
        this.addSlotsToEnd(after);
    }

    public getFirstEquippedSlotIndex(type: ItemType): number {
        for(let i = 0; i<this.internalSlots.length;i++){
			if(this.internalSlots[i].equipped){
				const data = itemToItemData(this.internalSlots[i].item);
                if(data.type === type){
                    return i;
                }
			}
		}
        return -1;
    }
}