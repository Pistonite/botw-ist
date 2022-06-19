import { Inventory } from "./Inventory";
import { Item } from "./Item";
import { ItemStack } from "./ItemStack";

export interface Command {
    execute(inv: Inventory): void,
    getDisplayString(): string,
}

export class CommandNothing implements Command {

    constructor(){
       
    }
    execute(inv: Inventory): void {
       
    }
    getDisplayString(): string {
        return "";
    }
    
}

export class CommandInitialize implements Command {
    private stacks: ItemStack[]
    constructor(stacks: ItemStack[]){
        this.stacks = stacks;
    }
    public execute(inv: Inventory): void {
        inv.init(this.stacks);
    }
    public getDisplayString(): string {
        const parts = ["Initialize"];
        this.stacks.forEach(({item, count})=>{
            parts.push(""+count);
            parts.push(item);
        })
        return parts.join(" ");
    }
}

export class CommandBreakSlots implements Command {
    private numToBreak: number;
    constructor(numToBreak: number){
        this.numToBreak = numToBreak;
    }
    public execute(inv: Inventory): void {
        inv.addBrokenSlots(this.numToBreak);
    }
    public getDisplayString(): string {
        return `Break ${this.numToBreak} Slots`;
    }
}

export class CommandSave implements Command {
    public execute(inv: Inventory): void {
        inv.save();
    }
    public getDisplayString(): string {
        return "Save";
    }
}

export class CommandReload implements Command {
    public execute(inv: Inventory): void {
        inv.reload();
    }
    public getDisplayString(): string {
        return "Reload";
    }
}

export class CommandSortKey implements Command {
    public execute(inv: Inventory): void {
        inv.sortKey();
    }
    public getDisplayString(): string {
        return "Sort Key";
    }
}

export class CommandSortMaterial implements Command {
    public execute(inv: Inventory): void {
        inv.sortMaterial();
    }
    public getDisplayString(): string {
        return "Sort Material";
    }
}

export class CommandRemoveMaterial implements Command {
    private verb: string;
    private count: number;
    private item: Item;
    private slot: number;
    constructor(verb: string, count: number, item: Item, slot: number){
        this.verb = verb;
        this.count = count;
        this.item = item;
        this.slot = slot;
    }
    public execute(inv: Inventory): void {
        inv.remove(this.item, this.count, this.slot);
    }
    public getDisplayString(): string {
        return `${this.verb} ${this.count} ${this.item} From Slot ${this.slot+1}`;
    }
}

export class CommandRemoveUnstackableMaterial implements Command {
    private verb: string;
    private item: Item;
    private slot: number;
    constructor(verb: string,item: Item, slot: number){
        this.verb = verb;
        this.item = item;
        this.slot = slot;
    }
    public execute(inv: Inventory): void {
        inv.remove(this.item, 1, this.slot);
    }
    public getDisplayString(): string {
        return `${this.verb} ${this.item} From Slot ${this.slot+1}`;
    }
}

export class CommandAddMaterial implements Command {
    private verb: string;
    private count: number;
    private item: Item;
    constructor(verb: string, count: number, item: Item){
        this.verb = verb;
        this.count = count;
        this.item = item;
    }
    public execute(inv: Inventory): void {
        inv.add(this.item, this.count);
    }
    public getDisplayString(): string {
        return `${this.verb} ${this.count} ${this.item}`;
    }
}
