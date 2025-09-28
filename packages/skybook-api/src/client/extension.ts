/**
 * Utilities for extension popout windows
 */

import { wxWindowOwner } from "@pistonite/workex";

import type { ExtensionModule } from "./extension_types.ts";
import { skybookExtensionApp } from "../interfaces/ExtensionApp.bus.ts";
import type { ExtensionApp } from "../extension_app.ts";

/**
 * Initialize an extension popout window
 *
 * This should be called in the popout window, and will
 * get everything needed from the URL search params
 */
export const readExtensionProperties = (): ExtensionProperties => {
    /* eslint-disable @typescript-eslint/no-explicit-any */
    if (typeof (globalThis as any).window === "undefined") {
        throw new Error("initExtensionWindow must be called in a browser window");
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
): Promise<ExtensionApp | undefined> => {
    const result = await wxWindowOwner(properties.hostOrigin)({
        app: skybookExtensionApp(extension),
    });
    if (result.err) {
        console.error("Failed to establish connection with host window", result.err);
        return undefined;
    }
    const {
        protocols: { app },
    } = result.val;
    extension.onAppConnectionEstablished(app);

    return app;
};
