import { createContext, useContext } from "react";

import type { ItemSlotInfo } from "./data/ItemSlotInfo.ts";

export type SetItemTooltipFn = (
    x: number,
    y: number,
    info: ItemSlotInfo | undefined,
    target: HTMLElement | undefined,
) => void;

export type ItemTooltipContextState = {
    setItemTooltip: SetItemTooltipFn;
    tooltipTarget: HTMLElement | undefined;
};

export const ItemTooltipContext = createContext<ItemTooltipContextState>({
    setItemTooltip: () => {
        /* empty */
    },
    tooltipTarget: undefined,
});

export const useSetItemTooltip = () => {
    return useContext(ItemTooltipContext);
};
