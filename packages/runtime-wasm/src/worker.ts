import { LRUCache } from "lru-cache";
import { type Delegate, hostFromDelegate } from "@pistonite/workex";

import { bindRuntimeApiHost, RuntimeAppHostClient } from "skybook-runtime-api/sides/runtime";
import type { RuntimeApi } from "skybook-runtime-api";

import { getParserDiagnostics, type QuotedItemResolverFn } from "./parser.ts";

const app = new RuntimeAppHostClient({ worker: self });

// cache the item so we don't need to resolve it with the main thread
// every time.
// using `false` to represent "not found"
const quotedItemCache = new LRUCache<string, wasm_bindgen.ItemSearchResult | false>({
    max: 5120
});
const resolveQuotedItem: QuotedItemResolverFn = async (query) => {
    const cachedResult = quotedItemCache.get(query);
    if (cachedResult !== undefined) {
        return cachedResult ? cachedResult : undefined;
    }
    const result = await app.resolveQuotedItem(query);
    // communication error
    if (result.err) {
        return undefined;
    }
    // resolve error - ignore the error and assume not found
    if (result.val.err) {
        quotedItemCache.set(query, false);
        return undefined;
    }
    const item = result.val.val;
    quotedItemCache.set(query, item);
    return item;
}

async function boot() {
    await wasm_bindgen({ module_or_path: "/runtime/skybook.wasm" });

    // TODO: any init here

    const api = {
        resolveItemIdent: async (query) => {
            return wasm_bindgen.resolve_item_ident(query);
        },
        getParserDiagnostics: (script) => {
            return getParserDiagnostics(script, resolveQuotedItem);
        },

    } satisfies Delegate<RuntimeApi>;

    const handshake = bindRuntimeApiHost(hostFromDelegate(api), {
        worker: self,
    });
    await handshake.initiate();
}

void boot();
