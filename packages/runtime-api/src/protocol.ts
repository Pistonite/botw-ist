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
    onScriptChange(script: string): WorkexPromise<void>;

    getSemanticTokens(script: string, startPos: number, endPos: number): WorkexPromise<Uint32Array>;

    getParserDiagnostics(script: string): WorkexPromise<{ range: [number, number]; message: string }[]>;

    getRuntimeDiagnostics(script: string): WorkexPromise<{ range: [number, number]; message: string }[]>;

    getStepFromPos(script: string, pos: number): WorkexPromise<number>;

    getInventory(scriptHash: string, step: number): WorkexPromise<unknown>;
}
