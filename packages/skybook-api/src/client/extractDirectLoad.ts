import type { DirectLoad } from "../directLoad.ts";

/** Extract the DirectLoad payload from the page, if exists */
export const extractDirectLoad = (): DirectLoad | undefined => {
    /* eslint-disable @typescript-eslint/no-explicit-any */
    if (typeof (globalThis as any).window === "undefined") {
        return undefined;
    }
    if (!("document" in (globalThis as any))) {
        return undefined;
    }
    if ("__skybook_direct_load" in (globalThis as any).window) {
        // Remove script tag that's already executed
        const directLoadPayload: DirectLoad = (globalThis as any).window
            .__skybook_direct_load;
        (globalThis as any).document
            .querySelector("script[data-skybook-direct-load]")
            ?.remove();
        // verify payload
        if (
            directLoadPayload.content &&
            (directLoadPayload.type === "v3" || directLoadPayload.type === "v4")
        ) {
            return directLoadPayload;
        }
    }
    return undefined;
    /* eslint-enable @typescript-eslint/no-explicit-any */
};
