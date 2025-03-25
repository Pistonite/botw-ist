/**
 * Utilities for extension popout windows
 */

import { WorkerLike, withTargetOrigin } from "@pistonite/workex";

import { ExtensionAppClient } from "../interfaces/ExtensionApp.send.ts";
import type { ExtensionModule } from "./ExtensionTypes.ts";
import { bindExtensionHost } from "../interfaces/Extension.recv.ts";

/**
 * Initialize an extension popout window
 *
 * This should be called in the popout window, and will
 * get everything needed from the URL search params
 */
export const readExtensionProperties = (): ExtensionProperties => {
    /* eslint-disable @typescript-eslint/no-explicit-any */
    if (typeof (globalThis as any).window === "undefined") {
        throw new Error(
            "initExtensionWindow must be called in a browser window",
        );
    }

    const search = new URLSearchParams((globalThis as any).location.search);
    const origin: string = (globalThis as any).location.origin;

    const hostOrigin = search.get("skybookHostOrigin") || origin;

    // get the extension id, if it's a first party extension
    let extensionId = "";
    if (hostOrigin === origin) {
        extensionId = search.get("skybookExtensionId") || "";
    }

    // remove skybook* keys
    const keysToRemove = [];
    for (const key of search.keys()) {
        if (key.startsWith("skybook")) {
            keysToRemove.push(key);
        }
    }
    for (const key of keysToRemove) {
        search.delete(key);
    }

    return {
        extensionId,
        hostOrigin,
        params: search,
    };
    /* eslint-enable @typescript-eslint/no-explicit-any */
};

export type ExtensionProperties = {
    /**
     * If the extension is a first party extension, return its id.
     *
     * For third party extensions, this should be empty string
     */
    extensionId: string;

    /**
     * The origin of the host window.
     *
     * For first party extensions, this will be the same as window.location.origin,
     * for third party extensions, this will be the origin of the host window,
     * for example https://ist.pistonite.app
     */
    hostOrigin: string;

    /**
     * The rest of the URL search params
     *
     * This can be used to define custom parameters for 3rd party extensions
     */
    params: URLSearchParams;
};

export const connectPopoutExtensionWindow = async (
    extension: ExtensionModule,
    properties: ExtensionProperties,
) => {
    /* eslint-disable @typescript-eslint/no-explicit-any */
    let worker: WorkerLike;
    if ("skybookChannel" in (globalThis as any).window) {
        worker = (globalThis as any).window.skybookChannel;
    } else {
        worker = withTargetOrigin(properties.hostOrigin, (globalThis as any).window);
    }
    const app = new ExtensionAppClient({ worker });
    const handshake = bindExtensionHost(extension, { worker });
    await handshake.initiate();

    extension.onAppConnectionEstablished(app);
    /* eslint-enable @typescript-eslint/no-explicit-any */
};
