import type { WorkexPromise } from "@pistonite/workex";

import type { ItemSearchResult } from "./types.ts";

/**
 * API provided by the simulator app that the runtime needs to call
 *
 * @workex:send runtime
 * @workex:recv app
 */
export interface RuntimeApp {
    /**
     * Resolve a quoted item search query to a single item, or
     * return undefined if the item cannot be resolved due to error
     * or no match.
     */
    resolveQuotedItem(
        query: string,
    ): WorkexPromise<ItemSearchResult | undefined>;

    /**
     * The app will be notified whenever a simulation run completes.
     * Note if multiple runs are queued, this will only be called for the
     * last one.
     */
    onRunCompleted(): WorkexPromise<void>;
}
