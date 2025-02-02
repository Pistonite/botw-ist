import type { WorkexPromise } from "@pistonite/workex";

/**
 * API provided by the simulator runtime
 *
 * @workex:send app
 * @workex:recv runtime
 */
export interface RuntimeApi {
    /**
     * Resolve an item identifier search query to a list of items, ordered by score (best first).
     * Returns an empty list if no items are found.
     *
     * cook effect is the game's representation, or 0 for no effect
     */
    resolveItemIdent(
        query: string,
    ): WorkexPromise<{ actor: string; cookEffect: number }[]>;

    /**
     * Set the script for the runtime, which starts executing
     * the script immediately
     */
    setScript(script: string): WorkexPromise<string>;
}
