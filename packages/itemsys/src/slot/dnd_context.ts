import { createContext, useContext } from "react";

import type { ItemDragData } from "@pistonite/skybook-api";

export type ItemDragContextState = {
    data: ItemDragData | undefined;
    setData: (data: ItemDragData | undefined) => void;
};

export const ItemDragContext = createContext<ItemDragContextState>({
    data: undefined,
    setData: () => {
        console.error("ItemDragContext not provided");
    },
});

export const useItemDrag = () => {
    return useContext(ItemDragContext);
};
