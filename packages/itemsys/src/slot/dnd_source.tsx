import { useState, type PropsWithChildren } from "react";

import type { ItemDragDataWithoutLocation } from "@pistonite/skybook-api";

import { useItemDrag } from "./dnd_context.ts";
import { dndLog as log } from "./dnd_util.ts";

export type DragSourceProps = {
    data: ItemDragDataWithoutLocation;
};

/**
 * A drag-and-drop source item. Clicking on this item will start dragging,
 * using the provided data
 */
export const DragSource: React.FC<PropsWithChildren<DragSourceProps>> = ({ data, children }) => {
    const { setData } = useItemDrag();
    const [dragging, setDragging] = useState(false);
    return (
        <div
            style={{
                opacity: dragging ? 0.1 : 1,
                cursor: "pointer",
            }}
            onDragStart={(e) => {
                log.info("start dragging item");
                setDragging(true);
                // keep location of the item if dragging with right mouse button
                const keepLocation = !!(e.buttons & 2);
                setData({ ...data, keepLocation });
                // this is for compatibility without using skybook-itemsys
                e.dataTransfer.clearData();
                e.dataTransfer.setData(
                    "application/skybook-item-drag-json",
                    JSON.stringify(data, (_key: string, value) => {
                        if (typeof value === "bigint") {
                            return 0;
                        }
                        return value;
                    }),
                );
            }}
            onDragEnd={() => {
                log.info("stop dragging item");
                setDragging(false);
                setData(undefined);
            }}
            draggable
        >
            {children}
        </div>
    );
};
