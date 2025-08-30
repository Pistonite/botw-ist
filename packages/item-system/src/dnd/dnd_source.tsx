import type { PropsWithChildren } from "react";
import { makeStyles } from "@fluentui/react-components";

import type { ItemDragData, ItemDragDataWithoutLocation } from "@pistonite/skybook-api";

import { useItemDnD } from "./dnd_context.ts";

export type DnDSourceProps = {
    data: ItemDragDataWithoutLocation;
};

const useStyles = makeStyles({
    container: {
        display: "contents",
        cursor: "pointer"
    }
});

/** 
 * A drag-and-drop source item. Clicking on this item will start dragging,
 * using the provided data
 */
export const DnDSource: React.FC<PropsWithChildren<DnDSourceProps>> = ({data, children}) => {
    const c = useStyles();
    const {startDragItem} = useItemDnD();
    return (
    <div 
            className={c.container}
            onMouseDown={(e)=>{
                // keep location of the item if dragging with right mouse button
                const keepLocation = !!(e.buttons & 2);
                void startDragItem({...data, keepLocation}, e.clientX, e.clientY);
            }}
        >
            {children}
    </div>
    );
}
