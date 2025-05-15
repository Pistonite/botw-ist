/** Wrapper for app calls */

import { LRUCache } from "lru-cache";
import { wxMakePromise } from "@pistonite/workex";

import type {
    ItemSearchResult,
    PerformanceData,
    RuntimeApp,
} from "@pistonite/skybook-api";

import type { QuotedItemResolverFn } from "./NativeApi.ts";

const { promise: appPromise, resolve: resolveApp } =
    wxMakePromise<RuntimeApp>();

export const resolveAppPromise = resolveApp;

// cache the item so we don't need to resolve it with the main thread
// every time.
// using `false` to represent "not found"
const quotedItemCache = new LRUCache<string, ItemSearchResult | false>({
    max: 5120,
});
export const resolveQuotedItem: QuotedItemResolverFn = async (query) => {
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

export const getCustomBlueFlameImage = async () => {
    return (await appPromise).getCustomBlueFlameImage();
};

export const sendPerfData = async (data: PerformanceData) => {
    return (await appPromise).updatePerfData(data);
};

export const crashApplication = async () => {
    try {
        return (await appPromise).crashApplication();
    } catch (e) {
        console.error("Failed to signal the application to crash", e);
    }
};
