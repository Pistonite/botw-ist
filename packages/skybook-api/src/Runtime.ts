import type { WxPromise } from "@pistonite/workex";
import type { Result } from "@pistonite/pure/result";

import type { ParserErrorReport } from "./parser";
import type {
    ItemSearchResult,
    RuntimeInitArgs,
    RuntimeInitError,
    RuntimeInitOutput,
} from "./types.ts";

/**
 * API provided by the simulator runtime, called by the application.
 */
export interface Runtime {
    /**
     * Initialize the runtime with the given arguments.
     */
    initialize(
        args: RuntimeInitArgs,
    ): WxPromise<Result<RuntimeInitOutput, RuntimeInitError>>;

    /**
     * Resolve an item identifier search query to a list of items, ordered by score (best first).
     * Returns an empty list if no items are found.
     */
    resolveItemIdent(query: string): WxPromise<ItemSearchResult[]>;

    /**
     * Parse the script and get diagnostics from the parser.
     *
     * Note that the span in the errors are byte offsets, not character offsets.
     */
    getParserDiagnostics(script: string): WxPromise<ParserErrorReport[]>;

    /**
     * Parse the script and get semantic tokens in the range from the parser.
     *
     * The output is triples of [start, length, tokenType]
     *
     * The offsets in both inputs and outputs should be byte offsets, not character offsets.
     */
    getSemanticTokens(
        script: string,
        start: number,
        end: number,
    ): WxPromise<Uint32Array>;

    // /**
    //  * Set the script for the runtime, which starts executing
    //  * the script immediately
    //  */
    // onScriptChange(script: string): WorkexPromise<void>;

    // getRuntimeDiagnostics(
    //     script: string,
    // ): WorkexPromise<{ range: [number, number]; message: string }[]>;
    //
    // getStepFromPos(script: string, pos: number): WorkexPromise<number>;
    //
    // getInventory(scriptHash: string, step: number): WorkexPromise<unknown>;
}
