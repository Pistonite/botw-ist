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
    [Item.Lotus]: 0x11,
    [Item.SilentPrincess]: 0x12,
    [Item.Honey]: 0x13,
    [Item.Acorn]: 0x14,
    [Item.FaroshScale]: 0x15,
    [Item.FaroshClaw]: 0x16,
    [Item.FaroshHorn]: 0x17,
    [Item.HeartyBass]: 0x18,
    [Item.Beetle]: 0x19,
    [Item.Opal]: 0x1a,
    [Item.Tail]: 0x1b,
    [Item.Spring]: 0x1c,
    [Item.Shaft]: 0x1d,
    [Item.Core]: 0x1e,
    [Item.Wood]: 0x1f,

    [Item.SpeedFood]: 0x40,
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
