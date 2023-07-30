import { AmountAllType } from "core/command";
import { Item, ItemStack, ItemType, MetaModifyOption } from "data/item";
import { Ref } from "data/util";
import { GameData } from "./GameData";
import { SlotDisplayForItemStack } from "./SlotDisplayForItemStack";
import { Slots } from "./Slots";
import { DisplayableInventory, GameFlags, SlotDisplay } from "./types";

/*
 * Implementation of Visible Inventory (PauseMenuDataMgr) in botw
 */
export class VisibleInventory implements DisplayableInventory{
	private slots: Slots = new Slots([]);
	// Difference between this.slots.length and what the game thinks the inventory size is
	// i.e. number of broken slots
	private offset: number;
	// Between an inventory wipe (load/new game) and the first item add (from GameData or in the world), tab
	// data is empty, which causes de-dupe checks to always be bypassed. Instead of simulating mListHeads
	// fully, we just indicate whether to pretend tabs are empty or not.
	// https://discord.com/channels/872350971383140422/1000992154140811325/1131656653561925824
	private listHeadsInit: Boolean = false;
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
		const result = this.slots.getView().map((stack, i)=>
			new SlotDisplayForItemStack(stack).init(i>=mCount, isIconAnimated)
		);
		return result;
	}

	public getSlots(): Slots {
		return this.slots;
	}

	public addDirectly(stack: ItemStack, index?: number): Ref<ItemStack>{
		const r = this.slots.addStackDirectly(stack, index);
		// adding an item *always* inits list heads, even if you're bumped up into mCount 0
		this.listHeadsInit = true;
		return r;
	}

	// return newly added ref, or lastAdded if no new slots are added
	public addWhenReload(stack: ItemStack, lastAdded: Ref<ItemStack> | undefined, flags: GameFlags): Ref<ItemStack> | undefined {
		const newlyAdded = this.slots.add(stack, true, this.getMCount(), flags);
		// if something was added, tab data is present and de-dupe checks can work
		this.listHeadsInit ||= newlyAdded !== undefined;
		const mostRecentlyAdded = newlyAdded || lastAdded;
		if(mostRecentlyAdded){
			// set cook data
			if(stack.item.type === ItemType.Food){
				mostRecentlyAdded.set(mostRecentlyAdded.get().transferExDataFrom(stack));
			}
		}
		return mostRecentlyAdded;
	}

	public addInGame(stack: ItemStack, flags: GameFlags) {
		this.slots.add(stack, false, this.getMCount(), flags, this.listHeadsInit);
		this.listHeadsInit = true;
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
		this.slots.equip(item, slot);
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
		this.slots.getView().forEach(({item, equipped}, i)=>{
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
		// get life value from last equipped, and update it to first slot
		// [confirmed] Jhent: updates both in visible inventory and gamedata
		// https://discord.com/channels/269611402854006785/269616041435332608/1042504927286657044
		const lastEquippedWeapon = this.slots.findLastEquipped(ItemType.Weapon);
		if(firstEquippedWeaponSlot >=0 && lastEquippedWeapon){
			gameData.updateLife(lastEquippedWeapon.get().count, firstEquippedWeaponSlot);
			this.slots.updateLife(lastEquippedWeapon.get().count, firstEquippedWeaponSlot);
		}
		const lastEquippedBow = this.slots.findLastEquipped(ItemType.Bow);
		if(firstEquippedBowSlot >=0 && lastEquippedBow){
			gameData.updateLife(lastEquippedBow.get().count, firstEquippedBowSlot);
			this.slots.updateLife(lastEquippedBow.get().count, firstEquippedBowSlot);
		}
		const lastEquippedShield = this.slots.findLastEquipped(ItemType.Shield);
		if(firstEquippedShieldSlot >=0 && lastEquippedShield){
			gameData.updateLife(lastEquippedShield.get().count, firstEquippedShieldSlot);
			this.slots.updateLife(lastEquippedShield.get().count, firstEquippedShieldSlot);
		}
	}

	public shootArrow(count: number | AmountAllType, gameData: GameData) {
		const updatedSlot = this.slots.shootArrow(count);
		if(updatedSlot>=0){
			const durability = this.slots.getView()[updatedSlot].count;
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

	public getItemSlotCounts(): number[] {
		const array: number[] = [];
		this.slots.getView().forEach(stack=>{
			array[stack.item.type]=(array[stack.item.type]||0)+1;
		});
		return array;
	}

	public swap(i: number, j: number) {
		this.slots.swap(i, j);
	}

	// public countItems(type: ItemType, countAnyWeapon: boolean): number {
	// 	// [confirmed] iTNTPiston: when mcount === 0, nothing is checked (only when =0)
	// 	const mcount = this.getMCount();
	// 	if(mcount === 0){
	// 		return 0;
	// 	}
	// 	return -999;
	// }
}
