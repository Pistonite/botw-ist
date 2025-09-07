import type { PropsWithChildren } from "react";

import type { ItemDragData, ItemDragDataWithoutLocation } from "@pistonite/skybook-api";

import { useItemDnD } from "./context.ts";

export type DragSourceProps = {
    data: ItemDragDataWithoutLocation;
};

/** 
 * A drag-and-drop source item. Clicking on this item will start dragging,
 * using the provided data
 */
export const DragSource: React.FC<PropsWithChildren<DragSourceProps>> = ({data, children}) => {
    const {startDragItem} = useItemDnD();
    return (
    <div 
            style={{
                display: "contents",
                cursor: "pointer"
            }}
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
