import {
    bindRuntimeAppHost,
    RuntimeClient,
} from "@pistonite/skybook-api/sides/app";

import { createRuntimeAppHost } from "application/RuntimeAppHost.ts";

/**
 * Initialize the simulator runtime and make sure it's ready
 */
export async function initRuntime() {
    const appHost = createRuntimeAppHost();
    // create the runtime worker
    const worker = new Worker("/runtime/worker.js");
    // bind the host handler
    bindRuntimeAppHost(appHost, { worker });
    // create the client for calling the runtime
    const runtime = new RuntimeClient({ worker });
    // wait for the handshake to complete
    await runtime.handshake().established();

    return runtime;
}
