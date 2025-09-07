/**
 * The Drag-and-Drop (DnD) system.
 *
 * This system enables dragging any item slot displayed that's draggable on the app,
 * to any registered "drop target" to transfer the item data to the target.
 * The items can be dragged and dropped from both the main app window
 * and the extension popout windows, and can be dragged between different windows.
 *
 * The main app window holds the source of truth for what is being dragged.
 * When drag starts, the app window will register event handlers for handling the DnD action.
 * If the drag is started from a popout window, the popout must have a RemoteItemDnDProvider
 * to send the drag start signal to the main app using skybook-api.
 *
 * The app will always track whether the dragged item is still in the main app window,
 * or dragged out of the app window. On the other hand, extension window
 * will stop tracking the item when it's dragged out of the window, and will ask
 * the app for the state when something is dragged into the window.
 *
 * 3rd-party extensions should use the `@pistonite/skybook-react` package to hook into
 * the DnD system. Currently, only adding drop targets are supported, since the item slots
 * UI currently requires the `skybook-item-system` internal package.
 *
 * Currently, only mouse is supported, not touch.
 *
 * @module
 */

import { logger } from "@pistonite/pure/log";

import { DropTargets , hideDraggingDiv, updateDraggingDiv } from "@pistonite/skybook-react";

import { useSessionStore } from "./session_store.ts";

export type DnDSystemApi = {
    /** Attach DnD events when start dragging remotely */
    attachDnDEvents: () => void,
};

export const dndLog = logger("dnd", "#b2dc9b").default();

let systemApi: DnDSystemApi | undefined = undefined;

export const registerDnDSystemApi = (api: DnDSystemApi) => {
    dndLog.info("dnd system api registered");
    systemApi = api;
}

export const attachDnDEvents = () => {
    systemApi?.attachDnDEvents();
}

const dropTargets = new DropTargets();
export const registerDropTarget = dropTargets.registerDropTarget.bind(dropTargets);

export const addContainerEventListenersForRef = (
    abortFnRef: React.MutableRefObject<(() => void) | null>,
    containerRef: React.RefObject<HTMLDivElement>,
    draggingRef: React.RefObject<HTMLDivElement>,
    ) => {
    const container = containerRef.current;
    if (container) {
        const controller = addContainerEventListeners(container, draggingRef);
        abortFnRef.current?.();
        abortFnRef.current = controller;
    }
}

const addContainerEventListeners = (
    container: HTMLDivElement,
    draggingRef: React.RefObject<HTMLDivElement>,
): () => void => {
    dndLog.info("attaching dnd events");
    const controller = new AbortController();
    // handle dropping the item
    container.addEventListener("mouseup", (e) => {
        dndLog.info("dropping item");
        const {dragData, setDragData} = useSessionStore.getState();
        if (dragData) {
            dropTargets.dropItem(dragData, e.clientX, e.clientY);
        }
        setDragData(undefined);
        hideDraggingDiv(draggingRef);
        controller.abort();
    }, { signal: controller.signal });
    // handle dragging out of the window
    container.addEventListener("mouseleave", () => {
        dndLog.info("item left main window");
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
        dndLog.info("unregistering dnd events");
        controller.abort();
    };
}
