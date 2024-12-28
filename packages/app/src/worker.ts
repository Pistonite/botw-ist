// import wasmUrl from "skybook-runtime-wasm/skybook_runtime_wasm_bg.wasm?url";
import { parse_script } from "skybook-runtime-wasm";

import { Delegate, hostFromDelegate } from "workex";

import { bindRuntimeApiHost } from "./runtime/interfaces/RuntimeApi.recv";
import { RuntimeApi } from "./runtime/protocol.ts";

async function boot() {

    const api = {
        parseScript: async (script) => {
            return parse_script(script);
        },
    } satisfies Delegate<RuntimeApi>;

    const handshake = bindRuntimeApiHost(hostFromDelegate(api), { worker: self });
    await handshake.initiate();
}

void boot();