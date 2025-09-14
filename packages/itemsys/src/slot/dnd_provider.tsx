import { useEffect, useMemo, useRef, useState, type PropsWithChildren } from "react";

import type { ExtensionApp, ItemDragData } from "@pistonite/skybook-api";

import { ItemDnDContext } from "./dnd_context.ts";
import { dndLog as log, DropTargets, hideDraggingDiv, updateDraggingDiv } from "./dnd_util.ts";
import { DraggingItemSlot } from "./dnd_slot.tsx";

export type RemoteItemDnDProviderProps = {
    app: ExtensionApp;
};

export const RemoteItemDnDProvider: React.FC<PropsWithChildren<RemoteItemDnDProviderProps>> = ({
    app,
    children,
}) => {
    const [dragData, setDragData] = useState<ItemDragData | undefined>(undefined);
    const needEnterCheck = useRef<boolean>(false);
    const containerRef = useRef<HTMLDivElement>(null);
    const draggingRef = useRef<HTMLDivElement>(null);
    const abortFnRef = useRef<(() => void) | null>(null);
    useEffect(() => {
        if (!dragData) {
            hideDraggingDiv(draggingRef);
            abortFnRef.current?.();
        }
    }, [dragData]);
    useEffect(() => {
        const container = containerRef.current;
        const controller = new AbortController();
        // the extension popout window always needs a listener
        // to detect when something is dragged into the window
        if (container) {
            // when dragged in, ask what the data is from app
            container.addEventListener(
                "mouseenter",
                async (e) => {
                    const isDragging = !!e.buttons;
                    if (!isDragging) {
                        log.info("not dragging");
                        await stopDragging(app);
                        return;
                    }
                    log.info("requesting dragging item from app");
                    const result = await app.getItemDragData();
                    if ("err" in result) {
                        log.error("failed to get drag data from app");
                        log.error(result.err);
                        // stop dragging in case we can't get data
                        await stopDragging(app);
                    }
                    // not dragging anything, stop
                    if (!result.val) {
                        log.info("did not get drag data from app");
                        return;
                    }
                    setDragData(result.val);
                },
                { signal: controller.signal },
            );
            // when dragged out, "forget" the data
            container.addEventListener("mouseleave", () => {
                setDragData(undefined);
            });
            log.info("attached base dnd events");
        }
        return () => {
            // clean up whatever last events registered on unmount
            // eslint-disable-next-line react-hooks/exhaustive-deps
            abortFnRef.current?.();
            controller.abort();
        };
    }, [app]);
    // stable reference
    const contextState = useMemo(() => {
        const startDragItem = async (data: ItemDragData, x: number, y: number) => {
            setDragData(data);
            updateDraggingDiv(draggingRef, x, y);
            addContainerEventListenersForRef(
                app,
                abortFnRef,
                containerRef,
                draggingRef,
                data,
                setDragData,
            );
            const result = await app.remoteItemDragStarted(data);
            if ("err" in result) {
                log.error("failed to send drag start signal to app");
                log.error(result.err);
            }
        };
        return {
            isDragging: !!dragData,
            startDragItem,
            registerDropTarget: dropTargets.registerDropTarget.bind(dropTargets),
        };
    }, [app, dragData]);
    return (
        <ItemDnDContext.Provider value={contextState}>
            <div
                ref={containerRef}
                style={{
                    position: "relative",
                    width: "100%",
                    height: "100%",
                }}
            >
                {children}
                {
                    <div
                        ref={draggingRef}
                        style={{
                            position: "absolute",
                            zIndex: 1000,
                        }}
                    >
                        {!!dragData && <DraggingItemSlot data={dragData} />}
                    </div>
                }
            </div>
        </ItemDnDContext.Provider>
    );
};

const dropTargets = new DropTargets();

const stopDragging = async (app: ExtensionApp) => {
    const result = await app.remoteItemDragStopped();
    if ("err" in result) {
        log.error("failed to send drag stopped signal to app");
        log.error(result.err);
    }
};
const addContainerEventListenersForRef = (
    app: ExtensionApp,
    abortFnRef: React.MutableRefObject<(() => void) | null>,
    containerRef: React.RefObject<HTMLDivElement>,
    draggingRef: React.RefObject<HTMLDivElement>,
    dragData: ItemDragData | undefined,
    setDragData: (data: ItemDragData | undefined) => void,
) => {
    const container = containerRef.current;
    if (container) {
        const controller = addContainerEventListeners(
            app,
            container,
            draggingRef,
            dragData,
            setDragData,
        );
        abortFnRef.current?.();
        abortFnRef.current = controller;
    }
};

const addContainerEventListeners = (
    app: ExtensionApp,
    container: HTMLDivElement,
    draggingRef: React.RefObject<HTMLDivElement>,
    dragData: ItemDragData | undefined,
    setDragData: (data: ItemDragData | undefined) => void,
): (() => void) => {
    log.info("attaching dnd events");
    const controller = new AbortController();
    // handle dropping the item
    container.addEventListener(
        "mouseup",
        (e) => {
            log.info("dropping item");
            if (dragData) {
                dropTargets.dropItem(dragData, e.clientX, e.clientY);
                void stopDragging(app);
            }
            setDragData(undefined);
            hideDraggingDiv(draggingRef);
            controller.abort();
        },
        { signal: controller.signal },
    );
    // handle dragging in the window and into the window
    container.addEventListener(
        "mousemove",
        (e) => {
            if (!e.buttons) {
                // if buttons are already released, abort the drag and drop
                setDragData(undefined);
                hideDraggingDiv(draggingRef);
                controller.abort();
                void stopDragging(app);
                return;
            }
            updateDraggingDiv(draggingRef, e.clientX, e.clientY);
        },
        { signal: controller.signal },
    );

    return () => {
        log.info("unregistering dnd events");
        controller.abort();
    };
};
