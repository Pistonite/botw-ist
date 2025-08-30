import type { ItemDragData, ItemDropTarget } from "@pistonite/skybook-api";

import { useSessionStore } from "./session_store.ts";

export type DnDSystemApi = {
    /** Attach DnD events when start dragging remotely */
    attachDnDEvents: () => void,
};

let systemApi: DnDSystemApi | undefined = undefined;

export const registerDnDSystemApi = (api: DnDSystemApi) => {
    systemApi = api;
}
export const getDnDSystemApi = () => {
    return systemApi;
}


