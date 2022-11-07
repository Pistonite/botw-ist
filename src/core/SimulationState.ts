import { Item, ItemStack, MetaOption } from "data/item";
import { Command } from "./command/command";
import { DisplayableInventory } from "./DisplayableInventory";
import { GameData } from "./GameData";
import { Slots } from "./Slots";
import { VisibleInventory } from "./VisibleInventory";

export const createSimulationState = (): SimulationState => {
	return new SimulationState(
		new GameData(new Slots([])),
		null,
		{},
		new VisibleInventory(new Slots([]), 0)
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
	private nextReloadName?: string;
	private isOnEventide = false;
	private crashed = false;

	constructor(gameData: GameData, manualSave: GameData | null, namedSaves: {[name: string]: GameData}, pouch: VisibleInventory){
		this.gameData = gameData;
		this.manualSave = manualSave;
		this.namedSaves = namedSaves;
		this.pouch = pouch;
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
		newState.nextReloadName = this.nextReloadName;
		newState.isOnEventide = this.isOnEventide;
		newState.crashed = this.crashed;

		return newState;
	}

	public isGameDataSyncedWithPouch(): boolean{
		return this.gameData.isSyncedWith(this.pouch);
	}

	// this is a wrapper that also have pre- and post-command checks
	public executeCommand(command: Command){
		this.crashed = false;
		command.execute(this);
		if(this.shouldCrash()){
			this.closeGame();
			this.crashed = true;
		}
	}

	public initialize(stacks: ItemStack[]) {
		this.pouch = new VisibleInventory(new Slots([]), 0);
		stacks.forEach((stack)=>this.pouch.addDirectly(stack));
		this.gameData.syncWith(this.pouch);
	}

	public setGameData(stacks: ItemStack[]) {
		this.gameData = new GameData(new Slots([...stacks]));
	}

	public save(name?: string) {
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
			}
		}else{
			if(this.nextReloadName){
				if(this.nextReloadName in this.namedSaves){
					this.reloadFrom(this.namedSaves[this.nextReloadName]);
				}
			}else{
				const save = this.manualSave;
				if(save){
					this.reloadFrom(save);
				}
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

	public useSaveForNextReload(name: string){
		this.nextReloadName = name;
	}

	public breakSlots(n: number) {
		this.pouch.modifyCount(-n);
	}

	public obtain(stack: ItemStack) {
		this.pouch.addInGame(stack);
		this.syncGameDataWithPouch();
	}

	public remove(stack: ItemStack, slot: number) {
		this.pouch.remove(stack, slot);
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

	public shootArrow(count: number){
		this.pouch.shootArrow(count, this.gameData);
		// does not sync
	}

	public setMetadata(item: Item, slot: number, meta: MetaOption) {
		this.pouch.setMetadata(item, slot, meta);
		this.syncGameDataWithPouch();
	}

	public closeGame() {
		this.pouch = new VisibleInventory(new Slots([]), 0);
		this.gameData = new GameData(new Slots([]));
		this.isOnEventide = false;
	}

	public setEventide(onEventide: boolean){
		if(this.isOnEventide !== onEventide){
			if(onEventide){
				// clear everything except for key items
				this.pouch.clearForEventide();
				// game data is not updated (?)

			}else{
				// reload pouch from gamedata as if reloading a save
				this.reloadFrom(this.gameData);
			}
			this.isOnEventide = onEventide;
		}

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
		return this.pouch.getCount();
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

}
