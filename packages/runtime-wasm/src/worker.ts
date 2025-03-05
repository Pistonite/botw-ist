import { LRUCache } from "lru-cache";
import { type Delegate, hostFromDelegate } from "@pistonite/workex";

import {
    bindRuntimeHost,
    RuntimeAppClient,
} from "@pistonite/skybook-api/sides/runtime";
import type { ItemSearchResult, Runtime } from "@pistonite/skybook-api";

import { getParserDiagnostics, type QuotedItemResolverFn } from "./parser.ts";

const app = new RuntimeAppClient({ worker: self });

// cache the item so we don't need to resolve it with the main thread
// every time.
// using `false` to represent "not found"
const quotedItemCache = new LRUCache<string, ItemSearchResult | false>({
    max: 5120,
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
    const item: ItemSearchResult | undefined = result.val;
    quotedItemCache.set(query, item);
    return item;
};

async function boot() {
    // This is injected by the build process
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const wasmModuleBase = (self as any)["__skybook_path_base"] as string;

    await wasm_bindgen({ module_or_path: wasmModuleBase + ".wasm" });

    // TODO: any init here

    const api = {
        resolveItemIdent: async (query) => {
            return wasm_bindgen.resolve_item_ident(query);
        },
        getParserDiagnostics: (script) => {
            return getParserDiagnostics(script, resolveQuotedItem);
        },
        getSemanticTokens: async (script, start, end) => {
            return wasm_bindgen.parse_script_semantic(script, start, end);
        },
    } satisfies Delegate<Runtime>;

    const handshake = bindRuntimeHost(hostFromDelegate(api), {
        worker: self,
    });
    await handshake.initiate();
}

void boot();
