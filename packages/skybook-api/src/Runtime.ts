import type { WorkexPromise } from "@pistonite/workex";

import { ParserErrorReport } from "./parser";
import { ItemSearchResult } from "./types.ts";

/**
 * API provided by the simulator runtime, called by the application.
 *
 * @workex:send app
 * @workex:recv runtime
 */
export interface Runtime {
    /**
     * Resolve an item identifier search query to a list of items, ordered by score (best first).
     * Returns an empty list if no items are found.
     */
    resolveItemIdent(query: string): WorkexPromise<ItemSearchResult[]>;

    // /**
    //  * Set the script for the runtime, which starts executing
    //  * the script immediately
    //  */
    // onScriptChange(script: string): WorkexPromise<void>;
    //
    // getSemanticTokens(
    //     script: string,
    //     startPos: number,
    //     endPos: number,
    // ): WorkexPromise<Uint32Array>;

    /**
     * Parse the script and get diagnostics from the parser.
     *
     * This does not runtime diagnostics
     */
    getParserDiagnostics(script: string): WorkexPromise<ParserErrorReport[]>;

    // getRuntimeDiagnostics(
    //     script: string,
    // ): WorkexPromise<{ range: [number, number]; message: string }[]>;
    //
    // getStepFromPos(script: string, pos: number): WorkexPromise<number>;
    //
    // getInventory(scriptHash: string, step: number): WorkexPromise<unknown>;
}
