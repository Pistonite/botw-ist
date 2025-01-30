import React, { type PropsWithChildren } from "react";

import type { ItemTooltipContentProps } from "./ItemTooltipContent.tsx";
import { useSetItemTooltip } from "./ItemTooltipContext.ts";

/** Wrapper to show tooltip for an ItemSlot */
export const ItemTooltip: React.FC<
    PropsWithChildren<ItemTooltipContentProps>
> = ({ info, children }) => {
    const setTooltip = useSetItemTooltip();

    return (
        <span
            onMouseMove={(e) => {
                setTooltip(e.clientX, e.clientY, info);
            }}
            onMouseLeave={() => {
                setTooltip(-1, -1, undefined);
            }}
        >
            {children}
        </span>
    );
};
