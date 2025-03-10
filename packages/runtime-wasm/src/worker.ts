import { LRUCache } from "lru-cache";
import { type Delegate, hostFromDelegate } from "@pistonite/workex";

import {
    bindRuntimeHost,
    RuntimeAppClient,
} from "@pistonite/skybook-api/sides/runtime";
import type { ItemSearchResult, Runtime, RuntimeInitArgs } from "@pistonite/skybook-api";

import { getParserDiagnostics, type QuotedItemResolverFn } from "./parser.ts";
import { getImage, putImage } from "./imagedb.ts";

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
        // TODO: the error needs to be structured
        initialize: async (args) => {
            // TODO: errors from the worker are currently logged to console
            // and returned as blanket errors. Tracked by #69
            if (args.isCustomImage) {
                // try reading the image from the database
                let customImage = await getImage();
                if (!customImage) {
                    // try requesting the image from the app
                    const newImage = await app.getCustomBlueFlameImage();
                    if (newImage.err || !newImage.val) {
                        console.error("Failed to get custom image from app");
                        return { err: { type: "DatabaseError" } };
                    }
                    const ok = await putImage(newImage.val);
                    if (!ok) {
                        // technically we can still use the image in memory,
                        // but the state will be inconsistency the next time
                        return { err: { type: "DatabaseError" } };
                    }
                    customImage = newImage.val;
                }

                // TODO: actually use the image

                console.log("Custom image loaded");
                console.log("Custom image size: " + customImage.length);
                // TODO: actually use the image
                console.log(args);
                return {
                    val: {
                        version: "1.5", // TODO: read from image
                        storedVersion: "1.5", // TODO: read from image
                    }
                };
            }
            if (args.deleteCustomImage) {
                await putImage(undefined);
                return {
                    val: {
                        version: "",
                        storedVersion: ""
                    }
                };
            }
            return {
                val: {
                    version: "",
                    storedVersion: "not-changed"
                }
            };
        },
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
