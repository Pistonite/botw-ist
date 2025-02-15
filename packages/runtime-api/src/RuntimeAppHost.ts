import type { Result } from "@pistonite/pure/result";
import type { WorkexPromise } from "@pistonite/workex";

/**
 * API provided by the simulator app that the runtime needs to call
 *
 * @workex:send runtime
 * @workex:recv app
 */
export interface RuntimeAppHost {
    /**
     * Resolve a quoted item search query to a single item, or else
     * return a localized error message
     *
     * cook effect is the game's representation, or 0 for no effect
     */
    resolveQuotedItem(
        query: string,
    ): WorkexPromise<Result<{ actor: string; cookEffect: number }, string>>;

    /**
     * The app will be notified whenever a simulation run completes.
     * Note if multiple runs are queued, this will only be called for the
     * last one.
     */
    onRunCompleted(): WorkexPromise<void>;

}
