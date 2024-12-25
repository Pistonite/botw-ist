import React, {
    useEffect,
    useState,
    PropsWithChildren,
} from "react";

import type { ItemTooltipContentProps } from "./ItemTooltipContent.tsx";
import { useSetItemTooltip } from "./ItemTooltipProvider.tsx";

/** Wrapper to show tooltip for an ItemSlot */
export const ItemTooltip: React.FC<PropsWithChildren<ItemTooltipContentProps>> = ({
    info,
    children,
}) => {
    const [coord, setCoord] = useState<[number, number]>([-1, -1]);
    const setTooltip = useSetItemTooltip();
    useEffect(() => {
        if (coord[0] < 0 || coord[1] < 0) {
            setTooltip(0, 0, undefined);
            return;
        }
        setTooltip(coord[0], coord[1], info);
    }, [setTooltip, coord, info]);

    return (
        <span
            onMouseMove={(e) => {
                setCoord([e.clientX, e.clientY]);
            }}
            onMouseLeave={() => {
                setCoord([-1, -1]);
            }}
        >
            {children}
        </span>
    );
};
