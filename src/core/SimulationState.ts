import { Item, ItemStack, ItemType, MetaModifyOption } from "data/item";
import { AmountAll, AmountAllType, Command } from "./command";
import { DisplayableInventory, GameData, GameFlags, Slots, VisibleInventory } from "./inventory";

export const createSimulationState = (): SimulationState => {
	return new SimulationState(
		new GameData(new Slots([]), {}),
		null,
		{},
		new VisibleInventory(new Slots([]))
	);
};
/*
 * The state of simulation, including game data, visible inventory, and all save slots
 */
export class SimulationState {
	private gameData: GameData;
	private manualSave: GameData | null;
	private namedSaves: {[name: string]: GameData} = {};
	private pouch: VisibleInventory;
	private isOnEventide = false;
	private crashed = false;
	private errorTitles: string[] = [];
	private errorMessages: string[] = [];

	constructor(gameData: GameData, manualSave: GameData | null, namedSaves: {[name: string]: GameData}, pouch: VisibleInventory){
		this.gameData = gameData;
		this.manualSave = manualSave;
		this.namedSaves = namedSaves;
		this.pouch = pouch;
	}

	// Only used in E2E tests
	public dump() {
		const namedSaveDump: any = {}; // eslint-disable-line @typescript-eslint/no-explicit-any
		for(const name in this.namedSaves){
			namedSaveDump[name] = this.namedSaves[name].dump();
		}
		return {
			gameData: this.gameData.dump(),
			pouch: this.pouch.dump(),
			manualSave: this.manualSave?.dump(),
			namedSaves: namedSaveDump,
			isOnEventide: this.isOnEventide,
			crashed: this.crashed
		};
	}

	public deepClone(): SimulationState {
		const copyNamedSaves: {[name: string]: GameData} = {};
		for(const name in this.namedSaves){
			copyNamedSaves[name] = this.namedSaves[name].deepClone();
		}
		const newState = new SimulationState(
			this.gameData.deepClone(),
			this.manualSave ? this.manualSave.deepClone() : null,
			copyNamedSaves,
			this.pouch.deepClone()
		);
		newState.isOnEventide = this.isOnEventide;
		newState.crashed = this.crashed;

		return newState;
	}

	public isGameDataSyncedWithPouch(): boolean{
		return this.gameData.isSyncedWith(this.pouch);
	}

	// this is a wrapper that also have pre- and post-command checks
	public executeCommand(command: Command){
		this.errorTitles = [];
		this.errorMessages = [];
		this.crashed = false;
		command.execute(this);
		if(this.shouldCrash()){
			this.closeGame();
			this.crashed = true;
		}
	}

	public initialize(stacks: ItemStack[]) {
		this.pouch = new VisibleInventory(new Slots([]));
		this.addSlotsDirectly(stacks);
		this.gameData.syncWith(this.pouch);
	}

	public addSlotsDirectly(stacks: ItemStack[], index?: number) {
		if(index !== undefined){
			stacks.forEach((stack, i)=>this.pouch.addDirectly(stack, index+i));
		}else{
			stacks.forEach((stack)=>this.pouch.addDirectly(stack));
		}

	}

	public setGameData(stacks: ItemStack[]) {
		this.gameData = new GameData(new Slots([...stacks]), {});
	}

	public save(name?: string) {
		if(this.isOnEventide){
			this.errorTitles.push(
				"Save failed",
			);
			this.errorMessages.push(
				"You cannot save while on Eventide or inside Trial of the Sword"
			);
			return;
		}
		if(name){
			this.namedSaves[name] = this.gameData.deepClone();
		}else{
			this.manualSave = this.gameData.deepClone();
		}
	}

	public reload(name?: string) {
		if(name){
			if(name in this.namedSaves){
				this.reloadFrom(this.namedSaves[name]);
			}else{
				this.errorTitles.push("Reload failed");
				this.errorMessages.push(
					`You are trying to reload the file "${name}", which doesn't exist`
				);
			}
		}else{
			const save = this.manualSave;
			if(save){
				this.reloadFrom(save);
			}else{
				this.errorTitles.push("Reload failed");
				this.errorMessages.push(
					"There's no manual save to reload from"
				);
			}
		}
	}

	public numberOfSaves(): number {
		let count = 0;
		if (this.manualSave){
			count++;
		}
		count += Object.values(this.namedSaves).filter(Boolean).length;
		return count;
	}

	private reloadFrom(data: GameData) {
		this.gameData = data.deepClone();
		this.pouch.clearForReload();
		this.gameData.addAllToPouchOnReload(this.pouch);
		this.pouch.updateEquipmentDurability(this.gameData);
		this.isOnEventide = false;
	}

	public breakSlots(n: number) {
		this.pouch.modifyOffset(n);
	}

	public obtain(stack: ItemStack) {
		this.pouch.addInGame(stack, this.gameData.getFlags());
		this.syncGameDataWithPouch();
	}

	public remove(stack: ItemStack, count: number | AmountAllType, slot: number): number {
		const removedCount = this.pouch.remove(stack, count, slot);
		this.syncGameDataWithPouch();
		if(count !== AmountAll && removedCount < count){
			this.errorTitles.push("Cannot remove item(s)");
			this.errorMessages.push(
				`Need to remove ${count}x${stack.item.id}, only ${removedCount} can be removed`
			);
		}
		return removedCount;
	}
	public eat(stack: ItemStack, count: number | AmountAllType, slot: number) {
		const removedCount = this.pouch.eat(stack, count, slot);
		this.syncGameDataWithPouch();
		if(count !== AmountAll && removedCount < count){
			this.errorTitles.push("Cannot eat item(s)");
			this.errorMessages.push(
				`Need to eat ${count}x${stack.item.id}, only ${removedCount} can be eaten`
			);
		}
	}

	public removeAll(types: ItemType[]) {
		this.pouch.removeAll(types);
		this.syncGameDataWithPouch();
	}

	public equip(item: Item, slot: number) {
		this.pouch.equip(item, slot);
		this.syncGameDataWithPouch();
	}

	public unequip(item: Item, slot: number){
		this.pouch.unequip(item, slot);
		this.syncGameDataWithPouch();
	}

	public unequipAll(types: ItemType[]){
		this.pouch.unequipAll(types);
		this.syncGameDataWithPouch();
	}

	public shootArrow(count: number | AmountAllType){
		this.pouch.shootArrow(count, this.gameData);
		// does not sync
	}

	public setMetadata(item: Item, slot: number, meta: MetaModifyOption) {
		this.pouch.setMetadata(item, slot, meta);
		// does not sync
	}

	public closeGame() {
		this.pouch = new VisibleInventory(new Slots([]));
		this.gameData = new GameData(new Slots([]), {});
		this.isOnEventide = false;
	}

	public setEventide(onEventide: boolean){
		if(onEventide){
			if(this.isOnEventide){
				this.errorTitles.push("Cannot enter trial");
				this.errorMessages.push("You are in another trial. Please exit first.");
			}else{
				// clear everything except for key items
				this.pouch.clearForEventide();
				// game data is not updated (?)
			}
		}else{
			if(!this.isOnEventide){
				this.errorTitles.push("Cannot leave trial");
				this.errorMessages.push("You are not in a trial. Please enter one first.");
			}else{
				// reload pouch from gamedata as if reloading a save
				this.reloadFrom(this.gameData);
			}
		}
		this.isOnEventide = onEventide;
	}

	public syncGameDataWithPouch() {
		if(!this.isOnEventide){
			this.gameData.syncWith(this.pouch);
		}
	}

	public shouldCrash(): boolean {
		return this.pouch.getSlots().length > 420;
	}

	public get displayableGameData(): DisplayableInventory {
		return this.gameData;
	}

	public get displayablePouch(): DisplayableInventory {
		return this.pouch;
	}

	public get inventoryMCount(): number {
		return this.pouch.getMCount();
	}

	public isCrashed(): boolean{
		return this.crashed;
	}

	public getManualSave(): GameData | null {
		return this.manualSave;
	}

	public getNamedSaves(): {[name: string]: GameData} {
		return this.namedSaves;
	}

	public get errors(): string[] {
		if (this.errorTitles.length === 0){
			return [];
		}
		const returnResult = [];
		const titleSet = new Set(this.errorTitles);
		if(titleSet.size === 1){
			returnResult.push(this.errorTitles[0]);
		}else{
			returnResult.push("This command gave multiple errors when executing:");
		}
		returnResult.push(...this.errorMessages);
		return returnResult;
	}

	public getInventoryInfo(): string[] {
		const text: string[] = [];
		text.push("Inventory Info");
		const itemCounts = this.pouch.getItemSlotCounts();
		itemCounts.forEach((count, itemType)=>{
			text.push(`${ItemType[itemType]} slot count: ${count}`);
		});
		text.push(`Max weapons: ${this.gameData.getFlag("weaponSlots")}`);
		text.push(`Max bows: ${this.gameData.getFlag("bowSlots")}`);
		text.push(`Max shields: ${this.gameData.getFlag("shieldSlots")}`);
		return text;
	}

	public setGameFlag(key: keyof GameFlags, value: string | number | boolean) {
		this.gameData.setFlag(key, value);
	}

	public swap(i: number, j: number) {
		this.pouch.swap(i,j);
	}

}
