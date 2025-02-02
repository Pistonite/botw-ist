import React, { type PropsWithChildren } from "react";

import type { ItemTooltipContentProps } from "./ItemTooltipContent.tsx";
import { useSetItemTooltip } from "./ItemTooltipContext.ts";

/** Wrapper to show tooltip for an ItemSlot */
export const ItemTooltip: React.FC<
    PropsWithChildren<ItemTooltipContentProps>
> = ({ info, children }) => {
    const { setItemTooltip } = useSetItemTooltip();

    return (
        <span
            onMouseMove={(e) => {
                setItemTooltip(
                    e.clientX,
                    e.clientY,
                    info,
                    e.target as HTMLElement,
                );
            }}
            onMouseLeave={() => {
                setItemTooltip(-1, -1, undefined, undefined);
            }}
        >
            {children}
        </span>
    );
};
