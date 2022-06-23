import { Inventory } from "./Inventory";
import { Item, ItemStack } from "./Item";
import { SimulationState } from "./SimulationState";

export interface Command {
	execute(state: SimulationState): void,
	getDisplayString(): string,
}

export class CommandNothing implements Command {

	execute(_state: Inventory): void {
		// nothing
	}
	getDisplayString(): string {
		return "";
	}
	
}

export class CommandInitialize implements Command {

	private stacks: ItemStack[];
	constructor(stacks: ItemStack[]){
		this.stacks = stacks;
	}

	public execute(state: SimulationState): void {
		state.initialize(this.stacks);
	}
	public getDisplayString(): string {
		return joinItemStackString("Initialize", this.stacks);
	}

}

export class CommandSave implements Command {

	public execute(state: SimulationState): void {
		state.save();
	}
	public getDisplayString(): string {
		return "Save";
	}
}

export class CommandSaveAs implements Command {
	private name: string;
	constructor(name: string){
		this.name = name;
	}
	public execute(state: SimulationState): void {
		state.save(this.name);
	}
	public getDisplayString(): string {
		return `Save As ${this.name}`;
	}
}

export class CommandReload implements Command {
	private name?: string;
	constructor(name?: string){
		this.name = name;
	}
	public execute(state: SimulationState): void {
		state.reload(this.name);
	}
	public getDisplayString(): string {
		return `Reload${this.name?` ${this.name}`:""}`;
	}
}

export class CommandUse implements Command {
	private name: string;
	constructor(name: string){
		this.name = name;
	}
	public execute(state: SimulationState): void {
		state.useSaveForNextReload(this.name);
	}
	public getDisplayString(): string {
		return `Use ${this.name}`;
	}
}

export class CommandBreakSlots implements Command {

	private numToBreak: number;
	constructor(numToBreak: number){
		this.numToBreak = numToBreak;
	}

	public execute(state: SimulationState): void {
		state.breakSlots(this.numToBreak);
	}
	public getDisplayString(): string {
		return `Break ${this.numToBreak} Slots`;
	}
}

export class CommandAdd implements Command {
	private verb: string;
	private count: number;
	private item: Item;
	constructor(verb: string, count: number, item: Item){
		this.verb = verb;
		this.count = count;
		this.item = item;
	}

	public execute(state: SimulationState): void {
		state.obtain(this.item, this.count);
	}
	public getDisplayString(): string {
		return `${this.verb} ${this.count} ${this.item}`;
	}
}

export class CommandAddWithoutCount implements Command {
	private verb: string;
	private item: Item;
	constructor(verb: string, item: Item){
		this.verb = verb;
		this.item = item;
	}

	public execute(state: SimulationState): void {
		state.obtain(this.item, 1);
	}
	public getDisplayString(): string {
		return `${this.verb} ${this.item}`;
	}
}

export class CommandAddMultiple implements Command {
	private verb: string;
	private stacks: ItemStack[];
	constructor(verb: string, stacks: ItemStack[]){
		this.verb = verb;
		this.stacks = stacks;
	}

	public execute(state: SimulationState): void {
		this.stacks.forEach(({item, count})=>state.obtain(item,count));
	}
	public getDisplayString(): string {
		return joinItemStackString(this.verb, this.stacks);

	}
}

export class CommandRemove implements Command {
	private verb: string;
	private count: number;
	private item: Item;
	private slot: number;
	private noSlot: boolean;
	constructor(verb: string, count: number, item: Item, slot: number, noSlot: boolean){
		this.verb = verb;
		this.count = count;
		this.item = item;
		this.slot = slot;
		this.noSlot = noSlot;
	}
	public execute(state: SimulationState): void {
		state.remove(this.item, this.count, this.slot);
	}
	public getDisplayString(): string {
		const slotString = this.noSlot ? "" : ` From Slot ${this.slot+1}`;
		return `${this.verb} ${this.count} ${this.item}${slotString}`;
	}
}

export class CommandRemoveWithoutCount implements Command {
	private verb: string;
	private item: Item;
	private slot: number;
	private noSlot: boolean;
	constructor(verb: string, item: Item, slot: number, noSlot: boolean){
		this.verb = verb;
		this.item = item;
		this.slot = slot;
		this.noSlot = noSlot;
	}
	public execute(state: SimulationState): void {
		state.remove(this.item, 1, this.slot);
	}
	public getDisplayString(): string {
		const slotString = this.noSlot ? "" : ` From Slot ${this.slot+1}`;
		return `${this.verb} ${this.item}${slotString}`;
	}
}

export class CommandRemoveMultiple implements Command {
	private verb: string;
	private stacks: ItemStack[];
	constructor(verb: string, stacks: ItemStack[]){
		this.verb = verb;
		this.stacks = stacks;
	}

	public execute(state: SimulationState): void {
		this.stacks.forEach(({item, count})=>state.remove(item,count,0));
	}
	public getDisplayString(): string {
		return joinItemStackString(this.verb, this.stacks);
	}
}

const joinItemStackString = (initial: string, stacks: ItemStack[]): string => {
	const parts: string[] = [initial];
	stacks.forEach(({item, count})=>{
		parts.push(""+count);
		parts.push(item);
	});
	return parts.join(" ");
};

export class CommandDaP implements Command {
	private count: number;
	private item: Item;

	constructor(count: number, item: Item,){
		this.count = count;
		this.item = item;
	}
	public execute(state: SimulationState): void {
		state.remove(this.item, this.count, 0);
		state.obtain(this.item, this.count);
	}
	public getDisplayString(): string {
		return `D&P ${this.count} ${this.item}`;
	}
}

export class CommandEquip implements Command {
	private item: Item;
	private slot: number;
	private noSlot: boolean;
	constructor(item: Item, slot: number, noSlot: boolean){
		this.item = item;
		this.slot = slot;
		this.noSlot = noSlot;
	}
	
	public execute(state: SimulationState): void {
		state.equip(this.item, this.slot);
	}
	public getDisplayString(): string {
		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
		return `Equip ${this.item}${slotString}`;
	}
}

export class CommandUnequip implements Command {
	private item: Item;
	private slot: number;
	private noSlot: boolean;
	constructor(item: Item, slot: number, noSlot: boolean){
		this.item = item;
		this.slot = slot;
		this.noSlot = noSlot;
	}
	
	public execute(state: SimulationState): void {
		state.unequip(this.item, this.slot);
	}
	public getDisplayString(): string {
		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
		return `Unequip ${this.item}${slotString}`;
	}
}

export class CommandShootArrow implements Command {
	private count: number;
	constructor(count: number){
		this.count = count;
	}
	
	public execute(state: SimulationState): void {
		state.shootArrow(this.count);
	}
	public getDisplayString(): string {
		return `Shoot ${this.count} Arrow`;
	}
}

export class CommandCloseGame implements Command {
	public execute(state: SimulationState): void {
		state.closeGame();
	}
	public getDisplayString(): string {
		return "Close Game";
	}
}

export class CommandSync implements Command {
	private actionString: string;
	constructor(actionString: string){
		this.actionString = actionString;
	}

	public execute(state: SimulationState): void {
		state.syncGameDataWithPouch();
	}
	public getDisplayString(): string {
		return this.actionString;
	}
}

export class CommandComment implements Command {
	private name: string;
	constructor(name: string){
		this.name = name;
	}
	public execute(_state: SimulationState): void {
		// nothing
	}
	public getDisplayString(): string {
		return `# ${this.name}`;
	}
}

export class CommandSortKey implements Command {
	static Op = 0x5;
	// public fromBuffer(_buf: Buffer): number {
	// 	return 0;
	// }
	// public toBuffer(): Buffer {
	// 	const buf: Buffer = Buffer.alloc(1);
	// 	buf.writeInt8(CommandSortKey.Op);
	// 	return buf;
	// }
	public execute(_state: SimulationState): void {
		// wip
	}
	public getDisplayString(): string {
		return "Sort Key";
	}
}

export class CommandSortMaterial implements Command {
	static Op = 0x6;
	// public fromBuffer(_buf: Buffer): number {
	// 	return 0;
	// }
	// public toBuffer(): Buffer {
	// 	const buf: Buffer = Buffer.alloc(1);
	// 	buf.writeInt8(CommandSortMaterial.Op);
	// 	return buf;
	// }
	public execute(_state: SimulationState): void {
		// wip
	}
	public getDisplayString(): string {
		return "Sort Material";
	}
}
