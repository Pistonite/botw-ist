import React, { type PropsWithChildren } from "react";

import type { ItemSlotContextProps } from "../slot";

import { useSetItemTooltip } from "./ItemTooltipContext.ts";
import type { ItemTooltipProps } from "./ItemTooltipProps.ts";

export type ItemTooltipWithContextProps = ItemTooltipProps &
    ItemSlotContextProps;

/** Wrapper to show tooltip for an ItemSlot */
export const ItemTooltip: React.FC<
    PropsWithChildren<ItemTooltipWithContextProps>
> = ({ children, ...props }) => {
    const { setItemTooltip } = useSetItemTooltip();

    return (
        <span
            onMouseMove={(e) => {
                setItemTooltip(
                    e.clientX,
                    e.clientY,
                    props,
                    e.target as HTMLElement,
                    e.shiftKey,
                );
            }}
            onMouseLeave={() => {
                setItemTooltip(-1, -1, undefined, undefined, false);
            }}
        >
            {children}
        </span>
    );
};
