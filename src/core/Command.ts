import { Inventory } from "./Inventory";
import { Item, ItemStack, itemToArrowType } from "./Item";
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
		const parts = ["Initialize"];
		this.stacks.forEach(({item, count})=>{
			parts.push(""+count);
			parts.push(item);
		});
		return parts.join(" ");
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



// export class CommandSortKey implements Command {
// 	static Op = 0x5;
// 	// public fromBuffer(_buf: Buffer): number {
// 	// 	return 0;
// 	// }
// 	// public toBuffer(): Buffer {
// 	// 	const buf: Buffer = Buffer.alloc(1);
// 	// 	buf.writeInt8(CommandSortKey.Op);
// 	// 	return buf;
// 	// }
// 	public execute(inv: Inventory): void {
// 		inv.sortKey();
// 	}
// 	public getDisplayString(): string {
// 		return "Sort Key";
// 	}
// }

// export class CommandSortMaterial implements Command {
// 	static Op = 0x6;
// 	// public fromBuffer(_buf: Buffer): number {
// 	// 	return 0;
// 	// }
// 	// public toBuffer(): Buffer {
// 	// 	const buf: Buffer = Buffer.alloc(1);
// 	// 	buf.writeInt8(CommandSortMaterial.Op);
// 	// 	return buf;
// 	// }
// 	public execute(inv: Inventory): void {
// 		inv.sortMaterial();
// 	}
// 	public getDisplayString(): string {
// 		return "Sort Material";
// 	}
// }

// const Verbs = ["?", "Remove", "Drop", "Sell", "Eat", "Cook", "Get", "Add", "Pickup"];
// const VerbToId = {
// 	"Remove" : 1,
// 	"Drop": 2,
// 	"Sell": 3,
// 	"Eat": 4,
// 	"Cook": 5,
// 	"Get": 6,
// 	"Add": 7,
// 	"Pickup": 8
// };

// export class CommandRemoveMaterial implements Command {
// 	static Op = 0x7;
// 	private verb: number;
// 	private count: number;
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(verb: string, count: number, item: Item, slot: number, noSlot: boolean){
// 		this.verb = VerbToId[verb as keyof typeof VerbToId]  || 0;
// 		this.count = count;
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}
// 	public execute(inv: Inventory): void {
// 		inv.remove(this.item, this.count, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` From Slot ${this.slot+1}`;
// 		return `${Verbs[this.verb]} ${this.count} ${this.item}${slotString}`;
// 	}
// }

// export class CommandRemoveUnstackableMaterial implements Command {
// 	static Op = 0x8;
// 	private verb: number;
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(verb: string,item: Item, slot: number, noSlot: boolean){
// 		this.verb = VerbToId[verb as keyof typeof VerbToId]  || 0;
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}
// 	public execute(inv: Inventory): void {
// 		inv.remove(this.item, 1, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` From Slot ${this.slot+1}`;
// 		return `${Verbs[this.verb]} ${this.item}${slotString}`;
// 	}
// }

// export class CommandAddMaterial implements Command {
// 	static Op = 0x9;
// 	private verb: number;
// 	private count: number;
// 	private item: Item;
// 	constructor(verb: string, count: number, item: Item){
// 		this.verb = VerbToId[verb as keyof typeof VerbToId]  || 0;
// 		this.count = count;
// 		this.item = item;
// 	}
// 	// public fromBuffer(buf: Buffer): number {
// 	// 	let read = 0;
// 	// 	const id = buf.readInt8(read);
// 	// 	read+=1;
// 	// 	this.item = idToItemData(id).item;

// 	// 	this.count = buf.readInt16LE(read);
// 	// 	read+=2;
// 	// 	this.verb = buf.readInt8(read);
// 	// 	read++;
// 	// 	return read;
// 	// }
// 	// public toBuffer(): Buffer {
// 	// 	const buf: Buffer = Buffer.alloc(1+1+2+1);
// 	// 	let write = 0;
// 	// 	buf.writeInt8(CommandAddMaterial.Op);
// 	// 	write++;
// 	// 	buf.writeInt8(itemToItemData(this.item).id, write);
// 	// 	write++;
// 	// 	buf.writeInt16LE(this.count, write);
// 	// 	write+=2;
// 	// 	buf.writeInt8(this.verb, write);
// 	// 	return buf;
// 	// }
// 	public execute(inv: Inventory): void {
// 		inv.add(this.item, this.count);
// 	}
// 	public getDisplayString(): string {
// 		return `${Verbs[this.verb]} ${this.count} ${this.item}`;
// 	}
// }

// export class CommandEquipArrow implements Command {
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(item: Item, slot: number, noSlot: boolean){
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}
	
// 	public execute(inv: Inventory): void {
// 		inv.equipEquipmentOrArrow(this.item, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
// 		return `Equip ${itemToArrowType(this.item)} Arrow${slotString}`;
// 	}
// }

// export class CommandEquip implements Command {
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(item: Item, slot: number, noSlot: boolean){
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}
	
// 	public execute(inv: Inventory): void {
// 		inv.equipEquipmentOrArrow(this.item, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
// 		return `Equip ${this.item}${slotString}`;
// 	}
// }

// export class CommandUnequip implements Command {
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(item: Item, slot: number, noSlot: boolean){
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}
	
// 	public execute(inv: Inventory): void {
// 		inv.unequipEquipment(this.item, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
// 		return `Unequip ${this.item}${slotString}`;
// 	}
// }



// export class CommandCloseGame implements Command {
// 	public execute(inv: Inventory): void {
// 		inv.closeGame();
// 	}
// 	public getDisplayString(): string {
// 		return "Close Game";
// 	}
// }

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
