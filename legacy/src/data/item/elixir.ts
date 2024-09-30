// The elixir API

import { CookEffect, Item } from "./type";

const ElixirCache = {} as Record<CookEffect, Item>;

// Elixirs are dynamic and the item changes depending on the cook effect
export const addElixir = (elixir: Item, effect: CookEffect): void => {
    ElixirCache[effect] = elixir;
};

export const getElixir = (effect: CookEffect): Item => {
    if (!ElixirCache[effect]) {
        throw new Error(
            `Elixir with effect ${CookEffect[effect]} is not registered`,
        );
    }
    return ElixirCache[effect] as Item;
};
