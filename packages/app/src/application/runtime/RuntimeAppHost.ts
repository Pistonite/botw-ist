import { type Delegate, hostFromDelegate } from "@pistonite/workex";

import type { RuntimeApp } from "@pistonite/skybook-api";
import {
    bindRuntimeAppHost,
    RuntimeClient,
} from "@pistonite/skybook-api/sides/app";
import { searchItemLocalized } from "skybook-localization";

let customImage: Uint8Array | undefined;

/**
 * Initialize the simulator runtime and make sure it's ready
 */
export async function initRuntime() {
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
    // bind the host handler
    bindRuntimeAppHost(appHost, { worker });
    // create the client for calling the runtime
    const runtime = new RuntimeClient({ worker });
    // wait for the handshake to complete
    await runtime.handshake().established();

    return runtime;
}

/** Create the API for runtime to call the app */
const createRuntimeAppHost = (): RuntimeApp => {
    const appHostDelegate = {
        resolveQuotedItem: async (query) => {
            const result = await searchItemLocalized(query, 1);
            if ("err" in result || !result.val.length) {
                return undefined;
            }
            const item = result.val[0];
            return item;
        },

        getCustomBlueFlameImage: async () => {
            return customImage;
        },

        onRunCompleted: async () => {
            return;
        },
    } satisfies Delegate<RuntimeApp>;

    return hostFromDelegate(appHostDelegate);
};

/** 
 * Set the custom image to provide to the runtime if the runtime asks for it
 */
export const setCustomImageToProvide = (image: Uint8Array | undefined) => {
    customImage = image;
}
