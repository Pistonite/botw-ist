import { type PropsWithChildren, useMemo } from "react";

import { ItemDragContext } from "@pistonite/skybook-itemsys";

import { useSessionStore } from "./session_store.ts";
// import {
//     addContainerEventListenersForRef,
//     registerDnDSystemApi,
//     registerDropTarget,
// } from "./dnd_system.ts";
//
// const useStyles = makeStyles({
//     draggingDiv: {
//         zIndex: 1000,
//         cursor: "pointer"
//     },
// });

export const MainWindowItemDragProvider: React.FC<PropsWithChildren> = ({ children }) => {
    // const m = useStyleEngine();
    // const c = useStyles();

    const dragData = useSessionStore((state) => state.dragData);
    const setDragData = useSessionStore((state) => state.setDragData);
    // const containerRef = useRef<HTMLDivElement>(null);
    // const draggingRef = useRef<HTMLDivElement>(null);
    // const abortFnRef = useRef<(() => void) | null>(null);

    // useEffect(() => {
    //     if (!dragData) {
    //         hideDraggingDiv(draggingRef);
    //         abortFnRef.current?.();
    //     }
    // }, [dragData]);
    // useEffect(() => {
    //     registerDnDSystemApi({
    //         attachDnDEvents: () => {
    //             addContainerEventListenersForRef(abortFnRef, containerRef, draggingRef);
    //         },
    //     });
    //     return () => {
    //         // clean up whatever last events registered on unmount
    //         // eslint-disable-next-line react-hooks/exhaustive-deps
    //         abortFnRef.current?.();
    //     };
    // }, []);

    // stable reference
    const contextState = useMemo(() => {
        return { data: dragData, setData: setDragData };
    }, [dragData, setDragData]);

    return <ItemDragContext.Provider value={contextState}>{children}</ItemDragContext.Provider>;
};
