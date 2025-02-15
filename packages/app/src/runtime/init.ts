import { hostFromDelegate, type Delegate } from "@pistonite/workex";
import { searchItemLocalized } from "skybook-localization";

import type { RuntimeAppHost } from "skybook-runtime-api";
import { bindRuntimeAppHostHost, RuntimeApiClient } from "skybook-runtime-api/sides/app";

/**
 * Initialize the simulator runtime and make sure it's ready
 */
export async function initRuntime() {
    // funtions callable from the runtime
    const appHostDelegate = {
        resolveQuotedItem: async (query) => {
            const result = await searchItemLocalized(query, 1);
            if ("err" in result || !result.val.length) {
                return { err: "not found" };
            }
            const item = result.val[0];
            return { val: item };
        },

        onRunCompleted: async () => {
            return;
        },
    } satisfies Delegate<RuntimeAppHost>;

    // create the runtime worker
    const worker = new Worker("/runtime/worker.js");
    // bind the host handler
    bindRuntimeAppHostHost(hostFromDelegate(appHostDelegate), { worker });
    // create the client for calling the runtime
    const runtime = new RuntimeApiClient({ worker });
    // wait for the handshake to complete
    await runtime.handshake().established();

    return runtime;
}
