import { makeStyles } from "@fluentui/react-components";
import { type PropsWithChildren, useEffect, useMemo, useRef} from "react";

import type { ItemDragData } from "@pistonite/skybook-api";
import { ItemDnDContext, hideDraggingDiv, updateDraggingDiv } from "@pistonite/skybook-react";
import { 
    DraggingItemSlot, 
} from "skybook-item-system";

import { useStyleEngine } from "self::util";

import { useSessionStore } from "./session_store.ts";
import { addContainerEventListenersForRef, registerDnDSystemApi, registerDropTarget } from "./dnd_system.ts";

const useStyles = makeStyles({
    zIndex: {
        zIndex: 1000
    }
});

export const MainWindowItemDnDProvider: React.FC<PropsWithChildren> =
({children}) => {
    const m = useStyleEngine();
    const c = useStyles();

    const dragData = useSessionStore(state => state.dragData);
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
            registerDnDSystemApi( {
                attachDnDEvents: () => {
                    addContainerEventListenersForRef(abortFnRef, containerRef, draggingRef);
                }
            }
            )
        return () => {
            // clean up whatever last events registered on unmount
            // eslint-disable-next-line react-hooks/exhaustive-deps
            abortFnRef.current?.()
        }
    }, []);

    // stable reference
    const contextState = useMemo(() => {
        const startDragItem = async (data: ItemDragData, x: number, y: number) => {
            const {setDragData} = useSessionStore.getState();
            setDragData(data);
            updateDraggingDiv(draggingRef, x, y);
                addContainerEventListenersForRef(abortFnRef, containerRef, draggingRef);
        };
        return {
            startDragItem, 
                registerDropTarget
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


}
