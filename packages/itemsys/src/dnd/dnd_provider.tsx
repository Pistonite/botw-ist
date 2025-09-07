import { logger } from "@pistonite/pure/log";

import { useEffect, useMemo, useRef, useState, type PropsWithChildren } from "react";
import { ExtensionApp, ItemDragData, ItemDropTarget } from "@pistonite/skybook-api";

import { ItemDnDContext } from "./dnd_context.ts";

const log = logger("dnd", "#b2dc9b").default();

export type RemoteItemDnDProviderProps = {
    app: ExtensionApp,
};
//
export const RemoteItemDnDProvider: React.FC<PropsWithChildren<RemoteItemDnDProviderProps>> = ({app, children}) => {
    const [dragData, setDragData] = useState<ItemDragData | undefined>(undefined);
    const containerRef = useRef<HTMLDivElement>(null);
    const draggingRef = useRef<HTMLDivElement>(null);
    const abortFnRef = useRef<(() => void) | null>(null);
    useEffect(() => {
        if (!dragData) {
            hideDraggingDiv(draggingRef)
            abortFnRef.current?.()
        }
    }, [dragData]);
    useEffect(() => {
        const container = containerRef.current;
        const controller = new AbortController();
        if (container) {
            container.addEventListener("mouseenter", (e) => {
                if (e.buttons) {

                }
                const {clientX, clientY, buttons} = e;

                // check if we are dragging
            })
        }
        return () => {
            // clean up whatever last events registered on unmount
            abortFnRef.current?.()
            controller.abort();
        }
    }, []);
    // stable reference
    const contextState = useMemo(() => {
        const startDragItem = async (data: ItemDragData, x: number, y: number) => {
            const result = await app.remoteItemDragStarted(data);
            if ("err" in result) {
                log.error("failed to send drag start signal to app");
                log.error(result.err);
            }

            setDragData(data);
            // 
            // await new Promise(resolve => setTimeout(resolve, 1));
            updateDraggingDiv(draggingRef, x, y);
            const container = containerRef.current;
            if (container) {
                const controller = addContainerEventListeners(container, draggingRef);
                abortFnRef.current?.();
                abortFnRef.current = controller;
            }
        };
        return {
            startDragItem, registerDropTarget
        };
    }
        , [app]);
    return (
    <ItemDnDContext.Provider value={contextState}>
            <div ref={containerRef} className={m("pos-rel wh-100")}>
                {children}
                {
                    <div 
                            ref={draggingRef}
                            className={m("pos-abs", c.zIndex)}
                        >
                            { !!dragData &&
                            <DraggingItemSlot data={dragData} />
                        }
                    </div>
                }
            </div>
    </ItemDnDContext.Provider>
    );
}
const dropTargets = new Set<ItemDropTarget>();
const registerDropTarget = (target: ItemDropTarget): () => void => {
    dropTargets.add(target);
    return () => {
        dropTargets.delete(target);
    }
}
export const updateDraggingDiv = (draggingRef: React.RefObject<HTMLDivElement>, x: number, y: number) => {
    const dragging = draggingRef.current;
    if (dragging) {
        // this is correct because the container div should always be the same
        // size as the viewport. Otherwise we need to adjust for the client
        // rect for the container div
        dragging.style.top = `${y - 36}px`;
        dragging.style.left = `${x - 36}px`;
        dragging.style.display = "unset";
    }
}

export const hideDraggingDiv = (draggingRef: React.RefObject<HTMLDivElement>) => {
    const dragging = draggingRef.current;
    if (dragging) {
        dragging.style.display = "none";
    }
}
