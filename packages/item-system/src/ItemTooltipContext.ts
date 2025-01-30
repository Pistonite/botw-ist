import { createContext, useContext } from "react";

import type { ItemSlotInfo } from "./data/ItemSlotInfo.ts";

export type SetItemTooltipFn = (
    x: number,
    y: number,
    info: ItemSlotInfo | undefined,
) => void;

export const ItemTooltipContext = createContext<SetItemTooltipFn>(() => {
    /* empty */
});

export const useSetItemTooltip = () => {
    return useContext(ItemTooltipContext);
};
