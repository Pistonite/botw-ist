import { useMemo, useSyncExternalStore, type PropsWithChildren } from "react";

import type { ExtensionApp, ItemDragData } from "@pistonite/skybook-api";

import { log } from "#util";

import { ItemDragContext } from "./dnd_context.ts";

export interface PopoutItemDragProviderProps {
    app: ExtensionApp;
    subscribeData: (fn: () => void) => () => void;
    getData: () => ItemDragData | undefined;
}

export const PopoutItemDragProvider: React.FC<PropsWithChildren<PopoutItemDragProviderProps>> = (
    props,
) => {
    const { app, subscribeData, getData, children } = props;
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
    return <ItemDragContext.Provider value={contextState}>{children}</ItemDragContext.Provider>;
};
