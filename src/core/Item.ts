import { ItemType } from "./ItemStack";

export enum Item {
    Slate = "Slate",
    Glider = "Glider",
    SpiritOrb = "SpiritOrb",

    
    Lotus = "Lotus",
    SilentPrincess = "SilentPrincess",
    Honey = "Honey",
    Acorn = "Acorn",
    FaroshScale = "FaroshScale",
    FaroshClaw = "FaroshClaw",
    FaroshHorn = "FaroshHorn",
    HeartyBass = "HeartyBass",
    Beetle = "Beetle",
    Opal = "Opal",
    Diamond = "Diamond",
    Tail = "Tail",
    Spring = "Spring",
    Shaft = "Shaft",
    Core = "Core",
    Wood = "Wood",

    SpeedFood = "SpeedFood"
}

export const ItemIds = {
    /* Do not change the ID once created. Otherwise you would break existing codes */
    [Item.Slate]: 0x00,
    [Item.Glider]: 0x01,
    [Item.SpiritOrb]: 0x02,

    [Item.Diamond]: 0x10,
}

export const itemToType = (item: Item): ItemType => {
    if (item === Item.Slate || item === Item.Glider || item === Item.SpiritOrb){
        return ItemType.Key;
    }
    if (item === Item.SpeedFood) {
        return ItemType.Meal;
    }
    return ItemType.Material;
}

export const shouldIgnoreOnReload = (item: Item): boolean => {
    return item === Item.Slate || item === Item.Glider;
}

export const isStackable = (item: Item): boolean => {
    return item !==Item.Slate && item !== Item.Glider && item !== Item.SpeedFood;
}

const KeyItemSortOrderMap = (()=>{
    const map: {[k in Item]?: number} = {};
    [
        Item.Slate,
        Item.Glider,
        Item.SpiritOrb
    ].forEach((item, i)=>map[item] = i);
    return map;
})();

export const getKeyItemSortOrder = (item: Item): number => {
    return KeyItemSortOrderMap[item] || -1;
}

const MaterialSortOrderMap = (()=>{
    const map: {[k in Item]?: number} = {};
    [
        Item.Lotus,
        Item.SilentPrincess,
        Item.Honey,
        Item.Acorn,
        Item.FaroshScale,
        Item.FaroshClaw,
        Item.FaroshHorn,
        Item.HeartyBass,
        Item.Beetle,
        Item.Opal,
        Item.Diamond,
        Item.Tail,
        Item.Spring,
        Item.Shaft,
        Item.Core,
        Item.Wood,
    ].forEach((item, i)=>map[item] = i);
    return map;
})();

export const getMaterialSortOrder = (item: Item): number => {
    return MaterialSortOrderMap[item] || -1;
}
