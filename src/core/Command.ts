import { Inventory } from "./Inventory";
import { Item, ItemIds } from "./Item";
import { ItemStack, ItemType } from "./ItemStack";

const Buffer = require("buffer/").Buffer;

export interface Command {
    execute(inv: Inventory): void,
    getDisplayString(): string,
    fromBuffer(buf: Buffer): number,
    toBuffer(): Buffer,
}

export class CommandNothing implements Command {
    static Op = 0x0;
    constructor(){
       
    }

    fromBuffer(buf: Buffer): number {
        return 0;
    }
    toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1);
        buf.writeInt8(CommandNothing.Op);
        return buf;
    }
    
    execute(inv: Inventory): void {
       
    }
    getDisplayString(): string {
        return "";
    }
    
}

export class CommandInitialize implements Command {
    static Op = 0x1;
    private stacks: ItemStack[]
    constructor(stacks: ItemStack[]){
        this.stacks = stacks;
    }
    public fromBuffer(buf: Buffer): number {
        let read = 0;
        const size = buf.readUInt16LE();
        read+=2;
        const stacks: ItemStack[] = [];
        for(let i=0;i<size;i++){
            const count = buf.readInt16LE(read);
            read+=2;
            const id = buf.readInt8(read);
            read++;
            for(const item in ItemIds){
                if(ItemIds[item as Item] === id){
                    stacks.push({item: item as Item, count});
                }
            }
        }
        this.stacks = stacks;
        return read;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(3*this.stacks.length+3);
        let write = 0;
        buf.writeInt8(CommandInitialize.Op);
        write++;
        buf.writeInt16LE(this.stacks.length, write);
        write+=2;
        this.stacks.forEach(({item,count})=>{
            console.log(count);
            buf.writeInt16LE(count&0xffff, write);
            write+=2;
            buf.writeInt8(ItemIds[item], write);
            write++;
        });
        return buf;
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
    static Op = 0x2;
    private numToBreak: number;
    constructor(numToBreak: number){
        this.numToBreak = numToBreak;
    }
    public fromBuffer(buf: Buffer): number {
        this.numToBreak = buf.readInt16LE();
        return 2;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(3);
        buf.writeUInt8(CommandBreakSlots.Op);
        buf.writeInt16LE(this.numToBreak, 1);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.addBrokenSlots(this.numToBreak);
    }
    public getDisplayString(): string {
        return `Break ${this.numToBreak} Slots`;
    }
}

export class CommandSave implements Command {
    static Op = 0x3;
    public fromBuffer(buf: Buffer): number {
        return 0;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1);
        buf.writeInt8(CommandSave.Op);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.save();
    }
    public getDisplayString(): string {
        return "Save";
    }
}

export class CommandReload implements Command {
    static Op = 0x4;
    public fromBuffer(buf: Buffer): number {
        return 0;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1);
        buf.writeInt8(CommandReload.Op);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.reload();
    }
    public getDisplayString(): string {
        return "Reload";
    }
}

export class CommandSortKey implements Command {
    static Op = 0x5;
    public fromBuffer(buf: Buffer): number {
        return 0;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1);
        buf.writeInt8(CommandSortKey.Op);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.sortKey();
    }
    public getDisplayString(): string {
        return "Sort Key";
    }
}

export class CommandSortMaterial implements Command {
    static Op = 0x6;
    public fromBuffer(buf: Buffer): number {
        return 0;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1);
        buf.writeInt8(CommandSortMaterial.Op);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.sortMaterial();
    }
    public getDisplayString(): string {
        return "Sort Material";
    }
}

const Verbs = ["?", "Remove", "Drop", "Sell", "Eat", "Cook", "Get", "Add", "Pickup"];
const VerbToId = {
    "Remove" : 1,
    "Drop": 2,
    "Sell": 3,
    "Eat": 4,
    "Cook": 5,
    "Get": 6,
    "Add": 7,
    "Pickup": 8
}

export class CommandRemoveMaterial implements Command {
    static Op = 0x7;
    private verb: number;
    private count: number;
    private item: Item;
    private slot: number;
    constructor(verb: string, count: number, item: Item, slot: number){
        this.verb = VerbToId[verb as keyof typeof VerbToId]  || 0;
        this.count = count;
        this.item = item;
        this.slot = slot;
    }
    public fromBuffer(buf: Buffer): number {
        let read = 0;
        this.count = buf.readInt16LE(read);
        read+=2;
        const id = buf.readInt8(read);
        read+=1;
        for(const item in ItemIds){
            if(ItemIds[item as Item] === id){
                this.item = item as Item;
            }
        }
        this.slot = buf.readInt16LE(read);
        read+=2;
        this.verb = buf.readInt8(read);
        read++;
        return read;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1+2+1+2+1);
        let write = 0;
        buf.writeInt8(CommandRemoveMaterial.Op);
        write++;
        buf.writeInt16LE(this.count, write);
        write+=2;
        buf.writeInt8(ItemIds[this.item], write);
        write++;
        buf.writeInt16LE(this.slot, write);
        write+=2;
        buf.writeInt8(this.verb, write);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.remove(this.item, this.count, this.slot);
    }
    public getDisplayString(): string {
        return `${Verbs[this.verb]} ${this.count} ${this.item} From Slot ${this.slot+1}`;
    }
}

export class CommandRemoveUnstackableMaterial implements Command {
    static Op = 0x8;
    private verb: number;
    private item: Item;
    private slot: number;
    constructor(verb: string,item: Item, slot: number){
        this.verb = VerbToId[verb as keyof typeof VerbToId]  || 0;;
        this.item = item;
        this.slot = slot;
    }
    public fromBuffer(buf: Buffer): number {
        let read = 0;
        const id = buf.readInt8(read);
        read+=1;
        for(const item in ItemIds){
            if(ItemIds[item as Item] === id){
                this.item = item as Item;
            }
        }
        this.slot = buf.readInt16LE(read);
        read+=2;
        this.verb = buf.readInt8(read);
        read++;
        return read;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1+1+2+1);
        let write = 0;
        buf.writeInt8(CommandRemoveUnstackableMaterial.Op);
        write++;
        buf.writeInt8(ItemIds[this.item], write);
        write++;
        buf.writeInt16LE(this.slot, write);
        write+=2;
        buf.writeInt8(this.verb, write);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.remove(this.item, 1, this.slot);
    }
    public getDisplayString(): string {
        return `${Verbs[this.verb]} ${this.item} From Slot ${this.slot+1}`;
    }
}

export class CommandAddMaterial implements Command {
    static Op = 0x9;
    private verb: number;
    private count: number;
    private item: Item;
    constructor(verb: string, count: number, item: Item){
        this.verb = VerbToId[verb as keyof typeof VerbToId]  || 0;
        this.count = count;
        this.item = item;
    }
    public fromBuffer(buf: Buffer): number {
        let read = 0;
        const id = buf.readInt8(read);
        read+=1;
        for(const item in ItemIds){
            if(ItemIds[item as Item] === id){
                this.item = item as Item;
            }
        }
        this.count = buf.readInt16LE(read);
        read+=2;
        this.verb = buf.readInt8(read);
        read++;
        return read;
    }
    public toBuffer(): Buffer {
        const buf: Buffer = Buffer.alloc(1+1+2+1);
        let write = 0;
        buf.writeInt8(CommandAddMaterial.Op);
        write++;
        buf.writeInt8(ItemIds[this.item], write);
        write++;
        buf.writeInt16LE(this.count, write);
        write+=2;
        buf.writeInt8(this.verb, write);
        return buf;
    }
    public execute(inv: Inventory): void {
        inv.add(this.item, this.count);
    }
    public getDisplayString(): string {
        return `${Verbs[this.verb]} ${this.count} ${this.item}`;
    }
}
