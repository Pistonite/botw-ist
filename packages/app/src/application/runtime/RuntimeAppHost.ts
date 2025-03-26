import type { Void } from "@pistonite/pure/result";
import { wxWorker, wxWrapHandler } from "@pistonite/workex";

import type {
    ItemSearchResult,
    Runtime,
    RuntimeApp,
    RuntimeInitArgs,
} from "@pistonite/skybook-api";
import { skybookRuntime } from "@pistonite/skybook-api/interfaces/Runtime.bus";
import {
    searchItemLocalized,
    translateGenericError,
} from "skybook-localization";

import { useApplicationStore, useSessionStore } from "self::application/store";

let customImage: Uint8Array | undefined;

/**
 * Create the runtime worker, but do not initialize it yet
 */
export async function createRuntime() {
    const appHost = createRuntimeAppHost();
    // create the runtime worker
    let url: string;
    if (import.meta.env.DEV) {
        console.log("[dev] using local runtime worker");
        url = "/runtime/worker.js";
    } else {
        const commitShort = import.meta.env.COMMIT.substring(0, 8);
        url = `/runtime/worker-${commitShort}.min.js`;
    }
    const worker = new Worker(url);
    const result = await wxWorker(worker)({
        runtime: skybookRuntime(appHost),
    });
    if (result.err) {
        console.error("[boot] failed to connect to runtime worker", result.err);
        throw new Error(
            "fatal boot failure: failed to connect to runtime worker",
        );
    }

    return result.val.protocols.runtime;
}

/** Initialize the runtime with the given arguments */
export async function initRuntime(
    runtime: Runtime,
    args: RuntimeInitArgs,
): Promise<Void<string>> {
    const isCustomImage = args.isCustomImage;
    updateLogo(isCustomImage);
    console.log(`[boot] initializing runtime, custom image: ${isCustomImage}`);

    const initWorkerResult = await runtime.initialize(args);

    if (initWorkerResult.err) {
        console.warn("[boot] runtime initialization failed (worker)");
        if (isCustomImage) {
            useApplicationStore.getState().setCustomImageVersion("");
        }
        return { err: translateGenericError(initWorkerResult.err.message) };
    }
    const initResult = initWorkerResult.val;
    if (initResult.err) {
        console.warn("[boot] runtime initialization failed");
        if (isCustomImage) {
            useApplicationStore.getState().setCustomImageVersion("");
        }
        // TODO: worker error structure, tracked by #69
        return { err: "initResult.err" };
    }

    const { version, storedVersion } = initResult.val;
    if (storedVersion !== "not-changed") {
        console.log(`[boot] updating stored image version to: ${version}`);
        useApplicationStore.getState().setCustomImageVersion(version);
    }

    useSessionStore.getState().setRunningCustomImageVersion(version);

    console.log(`[boot] runtime initialized successfully, version: ${version}`);
    return {};
}

/** Update the logo in the boot screen and the favicon */
export const updateLogo = (customImage: boolean) => {
    const image = customImage ? "/static/icon-purple.svg" : "/static/icon.svg";
    const linkIconTag = document.head.querySelector("link[rel='icon']");
    if (!linkIconTag) {
        const link = document.createElement("link");
        link.rel = "icon";
        link.type = "image/svg+xml";
        link.href = image;
        document.head.appendChild(link);
    } else {
        (linkIconTag as HTMLLinkElement).href = image;
    }

    const bootLogo = document.querySelector(".-boot-logo- img");
    if (!bootLogo || (bootLogo as HTMLImageElement).src === image) {
        return;
    }
    (bootLogo as HTMLImageElement).src = image;
};

/** Create the API for runtime to call the app */
const createRuntimeAppHost = (): RuntimeApp => {
    return {
        resolveQuotedItem: wxWrapHandler(
            async (query: string): Promise<ItemSearchResult | undefined> => {
                const result = await searchItemLocalized(query, 1);
                if ("err" in result || !result.val.length) {
                    return undefined;
                }
                const item = result.val[0];
                return item;
            },
        ),

        getCustomBlueFlameImage: wxWrapHandler(() => {
            return customImage;
        }),

        onRunCompleted: wxWrapHandler(() => {
            //TODO
        }),
    };
};

/**
 * Set the custom image to provide to the runtime if the runtime asks for it
 */
export const setCustomImageToProvide = (image: Uint8Array | undefined) => {
    customImage = image;
};
