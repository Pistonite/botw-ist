import { type PropsWithChildren, useMemo } from "react";

import { ItemDragContext } from "@pistonite/skybook-itemsys";

import { useSessionStore } from "./session_store.ts";

export const MainWindowItemDragProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const dragData = useSessionStore((state) => state.dragData);
    const setDragData = useSessionStore((state) => state.setDragData);
    // stable reference
    const contextState = useMemo(() => {
        return { data: dragData, setData: setDragData };
    }, [dragData, setDragData]);

    return <ItemDragContext.Provider value={contextState}>{children}</ItemDragContext.Provider>;
};
