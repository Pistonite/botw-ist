// The elixir API

import { CookEffect, Item } from "./type";

const ElixirCache: {
    [C in CookEffect]: Item | undefined
} = {
    [CookEffect.None]: undefined,
    [CookEffect.HotResist]: undefined,
    [CookEffect.ColdResist]: undefined,
    [CookEffect.ElectricResist]: undefined,
    [CookEffect.Stealth]: undefined,
    [CookEffect.Energizing]: undefined,
    [CookEffect.Enduring]: undefined,
    [CookEffect.Speed]: undefined,
    [CookEffect.Attack]: undefined,
    [CookEffect.Defense]: undefined,
    [CookEffect.Fireproof]: undefined,
    [CookEffect.Hearty]: undefined,
};

// Elixirs are dynamic and the item changes depending on the cook effect
export const addElixir = (elixir: Item, effect: CookEffect): void => {
    ElixirCache[effect] = elixir;
};

export const getElixir = (effect: CookEffect): Item => {
    if(!ElixirCache[effect]){
        throw new Error(`Elixir with effect ${CookEffect[effect]} is not registered`);
    }
    return ElixirCache[effect] as Item;
};
