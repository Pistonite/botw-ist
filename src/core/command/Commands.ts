import { Item } from "data/item";
// import { SimulationState } from "../SimulationState";
// import { processWrappers } from "./helper";
// import { ItemStackArg } from "./ItemStackArg";


// export class CommandDaP extends CommandImpl  {
// 	private stacks: ItemStackArg[];

// 	constructor(stacks: ItemStackArg[]){
// 		super();
// 		this.stacks = stacks;
// 	}
// 	public execute(state: SimulationState): void {
// 		processWrappers(this.stacks).forEach(stack=>{
// 			state.remove(stack, 0);
// 			state.obtain(stack);
// 		});
// 	}
// }

// export class CommandEquip extends CommandImpl  {
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(item: Item, slot: number, noSlot: boolean){
// 		super();
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}

// 	public execute(state: SimulationState): void {
// 		state.equip(this.item, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
// 		return `Equip ${this.item}${slotString}`;
// 	}
// }

// export class CommandUnequip extends CommandImpl {
// 	private item: Item;
// 	private slot: number;
// 	private noSlot: boolean;
// 	constructor(item: Item, slot: number, noSlot: boolean){
// 		super();
// 		this.item = item;
// 		this.slot = slot;
// 		this.noSlot = noSlot;
// 	}

// 	public execute(state: SimulationState): void {
// 		state.unequip(this.item, this.slot);
// 	}
// 	public getDisplayString(): string {
// 		const slotString = this.noSlot ? "" : ` In Slot ${this.slot+1}`;
// 		return `Unequip ${this.item}${slotString}`;
// 	}
// }

// export class CommandShootArrow extends CommandImpl  {
// 	private count: number;
// 	constructor(count: number){
// 		super();
// 		this.count = count;
// 	}

// 	public execute(state: SimulationState): void {
// 		state.shootArrow(this.count);
// 	}
// 	public getDisplayString(): string {
// 		return `Shoot ${this.count} Arrow`;
// 	}
// }

// export class CommandCloseGame extends CommandImpl  {
// 	public execute(state: SimulationState): void {
// 		state.closeGame();
// 	}
// 	public getDisplayString(): string {
// 		return "Close Game";
// 	}
// }



// export class CommandEventide extends CommandImpl  {
// 	private enter: boolean;
// 	constructor(enter: boolean){
// 		super();
// 		this.enter = enter;
// 	}

// 	public execute(state: SimulationState): void {
// 		state.setEventide(this.enter);
// 	}
// 	public getDisplayString(): string {
// 		return `${this.enter? "Enter":"Exit"} Eventide`;
// 	}
// }

// export class CommandNop extends CommandImpl  {
// 	private text: string;
// 	private error: string;
// 	constructor(text: string, error: string){
// 		super();
// 		this.text = text;
// 		this.error = error;
// 	}
// 	public getError(): string | undefined {
// 		return this.error;
// 	}
// 	public execute(_state: SimulationState): void {
// 		// nothing
// 	}
// 	public getDisplayString(): string {
// 		return this.text;
// 	}
// 	public equals(other: Command): boolean {
// 		return other instanceof CommandNop && this.text === other.text && this.error === other.error;
// 	}
// }

// export class CommandSortKey extends CommandImpl  {
// 	static Op = 0x5;
// 	// public fromBuffer(_buf: Buffer): number {
// 	// 	return 0;
// 	// }
// 	// public toBuffer(): Buffer {
// 	// 	const buf: Buffer = Buffer.alloc(1);
// 	// 	buf.writeInt8(CommandSortKey.Op);
// 	// 	return buf;
// 	// }
// 	public execute(_state: SimulationState): void {
// 		// wip
// 	}
// 	public getError(): string {
// 		return "This command is currently not supported";
// 	}
// }

// export class CommandSortMaterial extends CommandImpl  {
// 	static Op = 0x6;
// 	// public fromBuffer(_buf: Buffer): number {
// 	// 	return 0;
// 	// }
// 	// public toBuffer(): Buffer {
// 	// 	const buf: Buffer = Buffer.alloc(1);
// 	// 	buf.writeInt8(CommandSortMaterial.Op);
// 	// 	return buf;
// 	// }
// 	public execute(_state: SimulationState): void {
// 		// wip
// 	}
// 	public getError(): string {
// 		return "This command is currently not supported";
// 	}
// }
