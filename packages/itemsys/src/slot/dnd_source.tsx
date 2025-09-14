import { useState, type PropsWithChildren } from "react";

import type { ItemDragDataWithoutLocation } from "@pistonite/skybook-api";

import { useItemDnD } from "./dnd_context.ts";

export type DragSourceProps = {
    data: ItemDragDataWithoutLocation;
};

/**
 * A drag-and-drop source item. Clicking on this item will start dragging,
 * using the provided data
 */
export const DragSource: React.FC<PropsWithChildren<DragSourceProps>> = ({ data, children }) => {
    const { startDragItem } = useItemDnD();
    const [dragging, setDragging] = useState(false);
    return (
        <div
            style={{
                opacity: dragging ? 0.1 : 1,
                // display: "contents",
                cursor: "pointer",
            }}
            // onMouseDown={(e) => {
            //     // e.preventDefault();
            //     // e.stopPropagation();
            //     // keep location of the item if dragging with right mouse button
            //     const keepLocation = !!(e.buttons & 2);
            //
            //     // void startDragItem({ ...data, keepLocation }, e.clientX, e.clientY);
            // }}
            onDragStart={(e) => {
                console.log("dragstart");
                setDragging(true);
            }}
            onDragEnd={(e) => {
                setDragging(false);
                console.log("dragend");
            }}
            draggable
        >
            {children}
        </div>
    );
};
