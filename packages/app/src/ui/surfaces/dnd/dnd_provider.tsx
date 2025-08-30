import { forwardRef, memo, PropsWithChildren, useCallback, useEffect, useImperativeHandle, useMemo, useRef, useState } from "react";
import { makeStyles } from "@fluentui/react-components";

import type { ItemDragData, ItemDropTarget } from "@pistonite/skybook-api";
import { DraggingItemSlot, hideDraggingDiv, ItemDnDContext, updateDraggingDiv } from "skybook-item-system";

import { DnDSystemApi, useSessionStore } from "self::application";
import { useStyleEngine } from "self::util";

const useStyles = makeStyles({
    zIndex: {
        zIndex: 1000
    }
});

export const MainWindowItemDnDProvider =
forwardRef<DnDSystemApi, PropsWithChildren>(
({children}, apiRef) => {
    const m = useStyleEngine();
    const c = useStyles();

    const dragData = useSessionStore(state => state.dragData);
    const containerRef = useRef<HTMLDivElement>(null);
    const draggingRef = useRef<HTMLDivElement>(null);
    const abortFnRef = useRef<(() => void) | null>(null);

    useImperativeHandle(apiRef, () => {
            return {
                attachDnDEvents: () => {
            const container = containerRef.current;
            if (container) {
                const controller = addContainerEventListeners(container, draggingRef);
                abortFnRef.current?.();
                abortFnRef.current = controller;
            }
                }
            }
        }, []);

    useEffect(() => {
        if (!dragData) {
            hideDraggingDiv(draggingRef)
            abortFnRef.current?.()
        }
    }, [dragData]);
    useEffect(() => {
        return () => {
            // clean up whatever last events registered on unmount
            abortFnRef.current?.()
        }
    }, []);

    // stable reference
    const contextState = useMemo(() => {
        const startDragItem = async (data: ItemDragData, x: number, y: number) => {
            const {setDragData} = useSessionStore.getState();
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
        , []);

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


});

MainWindowItemDnDProvider.displayName = "MainWindowItemDnDProvider";

const dropTargets = new Set<ItemDropTarget>();
const registerDropTarget = (target: ItemDropTarget): () => void => {
    dropTargets.add(target);
    return () => {
        dropTargets.delete(target);
    }
}

/** Invoke the handler on the target if the item is dropped on a target */
const dropItem = (
    data: ItemDragData,
    clientX: number, clientY: number
) => {
    const toRemove = [];
    let foundHandler: ItemDropTarget["handler"] | undefined = undefined;
    for (const target of dropTargets) {
        const {element, handler} = target;
        if (!element.isConnected) {
            toRemove.push(target);
            continue;
        }
        const {top,left,right,bottom} = element.getBoundingClientRect();
        if (top > clientY || left > clientX || right < clientX || bottom < clientY) {
            continue;
        }
        foundHandler = handler;
        break;
    }
    for (const target of toRemove) {
        dropTargets.delete(target);
    }
    foundHandler?.(data);
}


const addContainerEventListeners = (
    container: HTMLDivElement,
    draggingRef: React.RefObject<HTMLDivElement>,
): () => void => {
    const controller = new AbortController();
    // handle dropping the item
    container.addEventListener("mouseup", (e) => {
        const {dragData, setDragData} = useSessionStore.getState();
        if (dragData) {
            dropItem(dragData, e.clientX, e.clientY);
        }
        setDragData(undefined);
        hideDraggingDiv(draggingRef);
        controller.abort();
    }, { signal: controller.signal });
    // handle dragging out of the window
    container.addEventListener("mouseleave", () => {
        hideDraggingDiv(draggingRef);
    }, {signal: controller.signal});
    // handle dragging in the window and into the window
    container.addEventListener("mousemove", (e) => {
        if(!e.buttons) {
            // if buttons are already released, abort the drag and drop
            const {setDragData} = useSessionStore.getState();
            setDragData(undefined);
            hideDraggingDiv(draggingRef);
            controller.abort();
            return;
        }
        updateDraggingDiv(draggingRef, e.clientX, e.clientY);
    }, {signal: controller.signal});

    return () => {
        controller.abort();
    };
}

// const addMouseEnterEventListener = ():

