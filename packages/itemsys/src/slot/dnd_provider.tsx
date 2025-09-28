import { useMemo, useSyncExternalStore, type PropsWithChildren } from "react";

import type { ExtensionApp, ItemDragData } from "@pistonite/skybook-api";

import { ItemDragContext } from "./dnd_context.ts";
import { dndLog as log } from "./dnd_util.ts";

export type PopoutItemDragProviderProps = {
    app: ExtensionApp;
    subscribeData: (fn: () => void) => () => void;
    getData: () => ItemDragData | undefined;
};

export const PopoutItemDragProvider: React.FC<PropsWithChildren<PopoutItemDragProviderProps>> = ({
    app,
    subscribeData,
    getData,
    children,
}) => {
    const dragData = useSyncExternalStore(subscribeData, getData);
    // stable reference
    const contextState = useMemo(() => {
        const setData = async (data: ItemDragData | undefined) => {
            // notify the app
            const result = await app.handleItemDrag(data);
            if ("err" in result) {
                log.error("failed to send drag signal to app");
                log.error(result.err);
            }
        };
        return {
            data: dragData,
            setData,
        };
    }, [app, dragData]);
    return (
        <ItemDragContext.Provider value={contextState}>
{children}
        </ItemDragContext.Provider>
    );
};
//
// const dropTargets = new DropTargets();
//
// const stopDragging = async (app: ExtensionApp) => {
//     const result = await app.remoteItemDragStopped();
//     if ("err" in result) {
//         log.error("failed to send drag stopped signal to app");
//         log.error(result.err);
//     }
// };
// const addContainerEventListenersForRef = (
//     app: ExtensionApp,
//     abortFnRef: React.MutableRefObject<(() => void) | null>,
//     containerRef: React.RefObject<HTMLDivElement>,
//     draggingRef: React.RefObject<HTMLDivElement>,
//     dragData: ItemDragData | undefined,
//     setDragData: (data: ItemDragData | undefined) => void,
// ) => {
//     const container = containerRef.current;
//     if (container) {
//         const controller = addContainerEventListeners(
//             app,
//             container,
//             draggingRef,
//             dragData,
//             setDragData,
//         );
//         abortFnRef.current?.();
//         abortFnRef.current = controller;
//     }
// };
//
// const addContainerEventListeners = (
//     app: ExtensionApp,
//     container: HTMLDivElement,
//     draggingRef: React.RefObject<HTMLDivElement>,
//     dragData: ItemDragData | undefined,
//     setDragData: (data: ItemDragData | undefined) => void,
// ): (() => void) => {
//     log.info("attaching dnd events");
//     const controller = new AbortController();
//     // handle dropping the item
//     container.addEventListener(
//         "mouseup",
//         (e) => {
//             log.info("dropping item");
//             if (dragData) {
//                 dropTargets.dropItem(dragData, e.clientX, e.clientY);
//                 void stopDragging(app);
//             }
//             setDragData(undefined);
//             hideDraggingDiv(draggingRef);
//             controller.abort();
//         },
//         { signal: controller.signal },
//     );
//     // handle dragging in the window and into the window
//     container.addEventListener(
//         "mousemove",
//         (e) => {
//             if (!e.buttons) {
//                 // if buttons are already released, abort the drag and drop
//                 setDragData(undefined);
//                 hideDraggingDiv(draggingRef);
//                 controller.abort();
//                 void stopDragging(app);
//                 return;
//             }
//             updateDraggingDiv(draggingRef, e.clientX, e.clientY);
//         },
//         { signal: controller.signal },
//     );
//
//     return () => {
//         log.info("unregistering dnd events");
//         controller.abort();
//     };
// };
