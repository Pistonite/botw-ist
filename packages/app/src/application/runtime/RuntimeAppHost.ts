import { wxWorker, wxWrapHandler } from "@pistonite/workex";

import type { ItemSearchResult, RuntimeApp } from "@pistonite/skybook-api";
import { skybookRuntime } from "@pistonite/skybook-api/interfaces/Runtime.bus";
import { searchItemLocalized } from "skybook-localization";

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
