import React, { type PropsWithChildren } from "react";

import type { ItemTooltipWithContextProps } from "./tooltip_props.ts";
import { useSetItemTooltip } from "./tooltip_context.ts";

/** Wrapper to show tooltip for an ItemSlot */
export const TooltipSource: React.FC<PropsWithChildren<ItemTooltipWithContextProps>> = ({
    children,
    ...props
}) => {
    const { setItemTooltip } = useSetItemTooltip();

    return (
        <span
            onMouseMove={(e) => {
                setItemTooltip(e.clientX, e.clientY, props, e.target as HTMLElement, e.shiftKey);
            }}
            onMouseLeave={() => {
                setItemTooltip(-1, -1, undefined, undefined, false);
            }}
        >
            {children}
        </span>
    );
};
