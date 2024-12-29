import { RuntimeApiClient } from "./interfaces/RuntimeApi.send.ts";
import RuntimeWorker from "./worker.ts?worker";

/** 
 * Initialize the simulator runtime and make sure it's ready
 */
export async function initRuntime() {
    const worker = new RuntimeWorker();
    const runtime = new RuntimeApiClient({worker});
    await runtime.handshake().established();
    return runtime;
}
