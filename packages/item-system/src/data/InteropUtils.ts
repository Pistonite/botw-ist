import type { InvView_GdtItemData } from "@pistonite/skybook-api";

import { PouchItemType } from "./EnumTypes.ts";

export const isGdtDataEquipmentType = (
    data: InvView_GdtItemData,
): data is InvView_GdtItemData & { type: "sword" | "shield" | "bow" } => {
    return data.type === "sword" || data.type === "bow" || data.type === "shield";
};

export const isGdtDataFoodType = (
    data: InvView_GdtItemData,
): data is InvView_GdtItemData & { type: "food" } => {
    return data.type === "food";
};

/** Convert type from InvView_GdtItemData to PouchItemType enum */
export const gdtTypeToPouchItemType = (type: string): PouchItemType => {
    switch (type) {
        case "sword":
            return PouchItemType.Sword;
        case "bow":
            return PouchItemType.Bow;
        case "shield":
            return PouchItemType.Shield;
        case "food":
            return PouchItemType.Food;
    }
    return PouchItemType.Invalid;
};

export const normalizeIngredients = (ingredients: string[]): string[] => {
    const output = [...ingredients];
    while (output.length > 0 && output[output.length - 1] === "") {
        output.pop();
    }
    return output;
};
