import { DisplayableInventory, DisplayableSlot, itemStackToDisplayableSlot } from "./DisplayableInventory";
import { GameData } from "./GameData";
import { Slots } from "./Slots";
import { Item, ItemStack, ItemType, MetaOption } from "data/item";

/*
 * Implementation of Visible Inventory (PauseMenuDataMgr) in botw
 */
export class VisibleInventory implements DisplayableInventory{
	private slots: Slots = new Slots([]);
	/* Implementation of mCount in botw */
	private count = 0;
	constructor(slots: Slots, count: number){
		this.slots = slots;
		this.count = count;
	}

	public deepClone(): VisibleInventory {
		return new VisibleInventory(this.slots.deepClone(), this.count);
	}

	public getDisplayedSlots(isIconAnimated: boolean): DisplayableSlot[] {
		return this.slots.getSlotsRef().map((stack, i)=>itemStackToDisplayableSlot(stack, i>=this.count, isIconAnimated));
	}

	public getSlots(): Slots {
		return this.slots;
	}

	public addDirectly(stack: ItemStack){
		this.count+=this.slots.addStackDirectly(stack);
	}

	public addWhenReload(stack: ItemStack) {
		const slotsAdded = this.slots.add(stack, true, this.count);
		this.count+=slotsAdded;
	}

	public addInGame(stack: ItemStack) {
		const slotsAdded = this.slots.add(stack, false, this.count);
		this.count+=slotsAdded;
	}

	public remove(stack: ItemStack, slot: number) {
		const slotsRemoved = this.slots.remove(stack, slot);
		this.count-=slotsRemoved;
	}

	public equip(item: Item, slot: number) {
		this.slots.equip(item, slot, this.count);
	}

	public unequip(item: Item, slot: number) {
		this.slots.unequip(item, slot);
	}

	// Only clears first this.count
	public clearForReload() {
		if(this.count > 0){
			this.slots.clearFirst(this.count);
			this.count = 0;
		}
	}

	public updateEquipmentDurability(gameData: GameData) {
		// find last equipped weapon/bow/shield, but update the durability on first equipped slot
		// find first weapon/bow/shield. this one searches entire inventory
		let firstEquippedWeaponSlot = -1;
		let firstEquippedBowSlot = -1;
		let firstEquippedShieldSlot = -1;
		this.slots.getSlotsRef().forEach(({item, equipped}, i)=>{
			if(equipped){
				const type = item.type;
				if(type === ItemType.Weapon && firstEquippedWeaponSlot === -1){
					firstEquippedWeaponSlot = i;
				}
				if(type === ItemType.Bow && firstEquippedBowSlot === -1){
					firstEquippedBowSlot = i;
				}
				if(type === ItemType.Shield && firstEquippedShieldSlot === -1){
					firstEquippedShieldSlot = i;
				}
			}
		});
		// get life value from last equipped
		const lastEquippedWeaponSlot = this.slots.findLastEquippedSlot(ItemType.Weapon);
		if(firstEquippedWeaponSlot >=0 && lastEquippedWeaponSlot >=0){
			gameData.updateLife(this.slots.getSlotsRef()[lastEquippedWeaponSlot].count, firstEquippedWeaponSlot);
		}
		const lastEquippedBowSlot = this.slots.findLastEquippedSlot(ItemType.Bow);
		if(firstEquippedBowSlot >=0 && lastEquippedBowSlot >=0){
			gameData.updateLife(this.slots.getSlotsRef()[lastEquippedBowSlot].count, firstEquippedBowSlot);
		}
		const lastEquippedShieldSlot = this.slots.findLastEquippedSlot(ItemType.Shield);
		if(firstEquippedShieldSlot >=0 && lastEquippedShieldSlot >=0){
			gameData.updateLife(this.slots.getSlotsRef()[lastEquippedShieldSlot].count, firstEquippedShieldSlot);
		}
	}

	public shootArrow(count: number, gameData: GameData) {
		const updatedSlot = this.slots.shootArrow(count);
		if(updatedSlot>=0){
			const durability = this.slots.getSlotsRef()[updatedSlot].count;
			gameData.updateLife(durability, updatedSlot);
		}
	}

	public setMetadata(item: Item, slot: number, meta: MetaOption) {
		this.slots.setMetadata(item, slot, meta);
	}

	public getCount(): number {
		return this.count;
	}

	public modifyCount(delta: number): void {
		this.count+=delta;
	}

	public resetCount(): void {
		this.count = this.slots.length;
	}

	public clearForEventide(): void {
		this.count-=this.slots.clearAllButKeyItems();
	}
}
