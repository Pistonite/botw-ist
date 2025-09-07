import { createContext, useContext } from "react";

import type { ItemTooltipWithContextProps } from "./tooltip_props.ts";

export type SetItemTooltipFn = (
    x: number,
    y: number,
    props: ItemTooltipWithContextProps | undefined,
    target: HTMLElement | undefined,
    verbose: boolean,
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
