import { useEffect, useState, type PropsWithChildren } from "react";
import { makeStyles } from "@fluentui/react-components";

import type { ItemDragData } from "@pistonite/skybook-api";

import { useItemDnD } from "./dnd_context.ts";

export type ItemDropZoneProps = {
    onDropItem: (data: ItemDragData) => void
} & React.HTMLAttributes<HTMLDivElement>;

const useStyles = makeStyles({
    effectDiv: {
        position: "absolute",
        top: 0,
        left: 0,
        bottom: 0,
        right: 0,
        zIndex: 1000,
        boxShadow: "inset -2px -2px 5px 0px #ffffffaa, inset 2px 2px 5px 0px #ffffffaa",
        willChange: "opacity",
        transitionDuration: "0.2s",
        pointerEvents: "none",
    },
});

/** Wrapper to make an area a drop target for dragging items */
export const ItemDropZone: React.FC<PropsWithChildren<ItemDropZoneProps>> = ({ 
    onDropItem, children, ...props }) => {
    const c = useStyles();
    const { isDragging, registerDropTarget } = useItemDnD();
    const [ref, setRef] = useState<HTMLDivElement | null>(null);

    useEffect(() => {
        if (!ref) {
            return;
        }
        const unregister = registerDropTarget({ element: ref, handler: onDropItem });
        return unregister;
    }, [ref, registerDropTarget, onDropItem]);

    return (<div ref={setRef} {...props} style={{ position: "relative", ...props.style }}
    >
        {children}
        <div className={c.effectDiv} style={{opacity: isDragging ? 1:0}}
        >
        </div>
    </div>);
};
