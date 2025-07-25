import type { WxPromise } from "@pistonite/workex";

import type { ItemSearchResult } from "./types.ts";

/**
 * API provided by the simulator app that the runtime needs to call
 */
export interface RuntimeApp {
    /**
     * Resolve a quoted item search query to a single item, or
     * return undefined if the item cannot be resolved due to error
     * or no match.
     */
    resolveQuotedItem(query: string): WxPromise<ItemSearchResult | undefined>;

    /**
     * Get the custom BlueFlame image provided by the user.
     *
     * The runtime may request this if it's instructed to initialize
     * with a custom image. For the best user experience, the app should
     * prompt file selection and have the image ready before initializing,
     * and return the file in this callback.
     *
     * If the user did not provide a custom image, the app should return undefined
     * in which case the runtime initialization will fail.
     */
    getCustomBlueFlameImage(): WxPromise<Uint8Array | undefined>;

    /** Signal the application to crash because unrecoverable error occurred in the runtime */
    crashApplication(): WxPromise<void>;
}
