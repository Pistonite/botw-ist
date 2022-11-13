import { AmountAllType } from "core/command";
import { Item, ItemStack, ItemType, MetaModifyOption } from "data/item";
import { GameData } from "./GameData";
import { SlotDisplayForItemStack } from "./SlotDisplayForItemStack";
import { Slots } from "./Slots";
import { DisplayableInventory, SlotDisplay } from "./types";

/*
 * Implementation of Visible Inventory (PauseMenuDataMgr) in botw
 */
export class VisibleInventory implements DisplayableInventory{
	private slots: Slots = new Slots([]);
	// Difference between this.slots.length and what the game thinks the inventory size is
	// i.e. number of broken slots
	private offset: number;
	constructor(slots: Slots){
		this.slots = slots;
		this.offset = 0;
	}

	public dump() {
		return {
			slots: this.slots.dump(),
			offset: this.offset
		};
	}

	public equals(other: VisibleInventory): boolean {
		return this.slots.equals(other.slots) && this.offset === other.offset;
	}

	// Get "mCount", the number of items tracked by the linked list in botw
	public getMCount(): number {
		return this.slots.length - this.offset;
	}

	public getOffset(): number {
		return this.offset;
	}

	public modifyOffset(delta: number): void {
		this.offset+=delta;
	}

	public deepClone(): VisibleInventory {
		const copy = new VisibleInventory(this.slots.deepClone());
		copy.offset = this.offset;
		return copy;
	}

	public getDisplayedSlots(isIconAnimated: boolean): SlotDisplay[] {
		const mCount = this.getMCount();
		const result = this.slots.getSlotsRef().map((stack, i)=>
			new SlotDisplayForItemStack(stack).init(i>=mCount, isIconAnimated)
		);
		return result;
	}

	public getSlots(): Slots {
		return this.slots;
	}

	public addDirectly(stack: ItemStack){
		this.slots.addStackDirectly(stack);
	}

	public addWhenReload(stack: ItemStack) {
		this.slots.add(stack, true, this.getMCount());
	}

	public addInGame(stack: ItemStack) {
		this.slots.add(stack, false, this.getMCount());
	}

	// Standard remove: magically remove item from inventory
	// sell is also this
	// returns number of items removed
	public remove(stack: ItemStack, count: number | AmountAllType, startSlot: number): number {
		return this.slots.remove(stack, count, { startSlot });
	}

	// Eat: food are treated as stackable to handle corrupted case, and 0 slot are removed
	public eat(stack: ItemStack, count: number | AmountAllType, startSlot: number): number {
		return this.slots.remove(stack, count, {
			startSlot,
			forceStackableFood: true,
			forceDeleteZeroSlot: true
		});
	}

	public equip(item: Item, slot: number) {
		this.slots.equip(item, slot, this.getMCount());
	}

	public unequip(item: Item, slot: number) {
		this.slots.unequip(item, slot);
	}

	// Only clears first this.count
	public clearForReload() {
		const count = this.getMCount();
		if(count > 0){
			this.slots.clearFirst(count);
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

	public shootArrow(count: number | AmountAllType, gameData: GameData) {
		const updatedSlot = this.slots.shootArrow(count);
		if(updatedSlot>=0){
			const durability = this.slots.getSlotsRef()[updatedSlot].count;
			gameData.updateLife(durability, updatedSlot);
		}
	}

	public setMetadata(item: Item, slot: number, meta: MetaModifyOption) {
		this.slots.setMetadata(item, slot, meta);
	}

	public clearForEventide(): void {
		this.slots.removeAll([
			ItemType.Weapon,
			ItemType.Bow,
			ItemType.Arrow,
			ItemType.Shield,
			ItemType.ArmorLower,
			ItemType.ArmorMiddle,
			ItemType.ArmorUpper,
			ItemType.Material,
			ItemType.Food
		]);
	}

	public removeAll(types: ItemType[]): void {
		this.slots.removeAll(types);
	}

	public unequipAll(types: ItemType[]): void {
		this.slots.unequipAll(types);
	}
}
