import { DisplayableInventory } from "./DisplayableInventory";
import { GameData } from "./GameData";
import { ItemStack } from "./Item";
import { Slots } from "./Slots";
import { VisibleInventory } from "./VisibleInventory";

export const createSimulationState = (): SimulationState => {
    return new SimulationState(
        new GameData(new Slots([])),
        null,
        {},
        new VisibleInventory(new Slots([]), 0)
    );
}
/*
 * The state of simulation, including game data, visible inventory, and all save slots
 */
export class SimulationState {
    private gameData: GameData;
    private manualSave: GameData | null;
    private namedSaves: {[name: string]: GameData} = {};
    private pouch: VisibleInventory;
    private nextReloadName?: string;

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
        return new SimulationState(
            this.gameData.deepClone(),
            this.manualSave ? this.manualSave.deepClone() : null,
            copyNamedSaves,
            this.pouch.deepClone()
        );
    }

    public initialize(stacks: ItemStack[]) {
        this.pouch = new VisibleInventory(new Slots([]), 0);
        stacks.forEach((stack)=>this.pouch.addDirectly(stack));
        this.gameData.syncWith(this.pouch);
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

    private reloadFrom(data: GameData) {
        this.gameData = data.deepClone();
        this.pouch.clearForReload();
        this.gameData.addAllToPouchOnReload(this.pouch);
    }

    public useSaveForNextReload(name: string){
        this.nextReloadName = name;
    }

    public breakSlots(n: number) {
        this.pouch.modifyCount(-n);
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

    public getManualSave(): GameData | null {
        return this.manualSave;
    }

    public getNamedSaves(): {[name: string]: GameData} {
        return this.namedSaves;
    }

    // public get displayableGameData(): DisplayableInventory {
    //     return this.gameData;
    // }


}

// Save - save to hard save slot
// Save As <name> - save to a auto save slot
// Reload - reload hard save
// Reload <name> - reload a named auto save
// Use <name> - no effect, but next Reload reloads the named auto save
// Break X Slots
// Sort Key (In Tab X)
// Sort Material (In Tab X)
// Get/Add/Cook/Pickup X <item>, X can be omitted and default to 1
// Get/Add/Cook/Pickup X <item> Y <item2> ...
// Remove/Drop/Sell/Eat X <item> From Slot Y, X can be omitted and default to 1
// Remove/Drop/Sell/Eat X <item1> Y <item2> ...
// D&P X <item>, drop and pick up (to sort)
// Equip <item> (In Slot X)
// Unequip <item> (In Slot X), without slot, it unequipps the first equipped
// Shoot X Arrow, x can be ommited and default to 1
// Close Game
// Close Inventory, same as Resync GameData
