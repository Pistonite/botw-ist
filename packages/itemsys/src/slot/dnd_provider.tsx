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
    return <ItemDragContext.Provider value={contextState}>{children}</ItemDragContext.Provider>;
};
