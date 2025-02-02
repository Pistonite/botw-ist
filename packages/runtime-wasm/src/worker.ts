import { type Delegate, hostFromDelegate } from "@pistonite/workex";

import { bindRuntimeApiHost } from "skybook-runtime-api/sides/runtime";
import type { RuntimeApi } from "skybook-runtime-api";

async function boot() {
    await wasm_bindgen({ module_or_path: "/runtime/skybook.wasm" });

    // TODO: any init here

    const api = {
        setScript: async (script) => {
            return wasm_bindgen.parse_script(script);
        },
        resolveItemIdent: async (query) => {
            return wasm_bindgen.resolve_item_ident(query);
        },
    } satisfies Delegate<RuntimeApi>;

    const handshake = bindRuntimeApiHost(hostFromDelegate(api), {
        worker: self,
    });
    await handshake.initiate();
}

void boot();
