import { createContext, useContext } from "react";

import type { ItemDragData, ItemDropTarget } from "@pistonite/skybook-api";

export type ItemDnDContextState = {
    /** Start dragging an item in this DnD context */
    startDragItem: (data: ItemDragData, x: number, y: number) => Promise<void>;

    /** Register a drop target. Return a function to unregister this target */
    registerDropTarget: (target: ItemDropTarget) => () => void;
};

export const ItemDnDContext = createContext<ItemDnDContextState>({
    startDragItem: async () => {
        console.error("DnD context is not provided!");
    },
    registerDropTarget: () => {
        console.error("DnD context is not provided!");
        return () => {};
    },
});

export const useItemDnD = () => {
    return useContext(ItemDnDContext);
};
