import { wxMakePromise, wxWorkerGlobal, wxWrapHandler } from "@pistonite/workex";
import type { Result } from "@pistonite/pure/result";
import { LRUCache } from "lru-cache";

import type { ItemSearchResult, Runtime, RuntimeApp, RuntimeInitArgs, RuntimeInitError, RuntimeInitOutput } from "@pistonite/skybook-api";
import { skybookRuntimeApp } from "@pistonite/skybook-api/interfaces/RuntimeApp.bus";

import { getParserDiagnostics, getStepFromPos, type QuotedItemResolverFn } from "./parser.ts";
import { getImage, putImage } from "./imagedb.ts";
import { executeScript, getInventoryListView } from "./runtime.ts";

const { promise: appPromise, resolve: resolveApp } = wxMakePromise<RuntimeApp>();

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
    const result = await (await appPromise).resolveQuotedItem(query);
    // communication error
    if (result.err) {
        return undefined;
    }

    const item: ItemSearchResult | undefined = result.val;
    if (!item) {
        quotedItemCache.set(query, false);
        return undefined;
    }

    quotedItemCache.set(query, item);
    return item;
};

async function boot() {
    // This is injected by the build process
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const wasmModuleBase = (self as any)["__skybook_path_base"] as string;

    await wasm_bindgen({ module_or_path: wasmModuleBase + ".wasm" });

    const {promise: initializePromise, resolve: markInitialized } = wxMakePromise();

    wasm_bindgen.module_init();

    const api: Runtime = {
        // TODO: the error needs to be structured
        initialize: wxWrapHandler(async (args: RuntimeInitArgs): Promise<Result<RuntimeInitOutput, RuntimeInitError>> => {
            // TODO: errors from the worker are currently logged to console
            // and returned as blanket errors. Tracked by #69
            if (args.isCustomImage) {
                // try reading the image from the database
                let customImage = await getImage();
                if (!customImage) {
                    // try requesting the image from the app
                    const newImage = await (await appPromise).getCustomBlueFlameImage();
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
                markInitialized(undefined);
                return {
                    val: {
                        version: "1.5", // TODO: read from image
                        storedVersion: "1.5", // TODO: read from image
                    }
                };
            }
            markInitialized(undefined);
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
        }),
        resolveItemIdent: wxWrapHandler((query) => {
            return wasm_bindgen.resolve_item_ident(query);
        }),
        getParserDiagnostics: wxWrapHandler(async (script) => {
            await initializePromise;
            return getParserDiagnostics(script, resolveQuotedItem);
        }),
        getSemanticTokens: wxWrapHandler(async (script, start, end) => {
            await initializePromise;
            return wasm_bindgen.parse_script_semantic(script, start, end);
        }),
        getStepFromPos: wxWrapHandler(async (script, pos) => {
            await initializePromise;
            return getStepFromPos(script, resolveQuotedItem, pos);
        }),
        executeScript: wxWrapHandler(async (script) => {
            await initializePromise;
            (await executeScript(script, resolveQuotedItem)).free();
        }),
        getInventoryListView: wxWrapHandler(async (script, pos) => {
            await initializePromise;
            return await getInventoryListView(script, resolveQuotedItem, pos);
        })
    };

    await wxWorkerGlobal()({
        app: skybookRuntimeApp(api, resolveApp),
    });
}

void boot();
