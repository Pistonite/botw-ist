import { RuntimeApiClient } from "skybook-runtime-api/sides/app";

/** 
 * Initialize the simulator runtime and make sure it's ready
 */
export async function initRuntime() {
    const worker = new Worker("/runtime/worker.js");
    const runtime = new RuntimeApiClient({worker});
    await runtime.handshake().established();
    return runtime;
}
