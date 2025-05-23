import type { WxPromise } from "@pistonite/workex";
import type { Result } from "@pistonite/pure/result";

import type { ParserErrorReport } from "./parser";
import type {
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
} from "./runtime";
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

    /** Get index of the step from byte position in the script */
    getStepFromPos(script: string, pos: number): WxPromise<number>;

    /**
     * Start executing the script in the background
     */
    executeScript(script: string): WxPromise<void>;

    /**
     * Execute the script if not up-to-date, and return the pouch inventory list view
     * at the byte offset `pos` in the script.
     */
    getPouchList(script: string, pos: number): WxPromise<InvView_PouchList>;

    /**
     * Execute the script if not up-to-date, and return the GDT inventory view
     * at the byte offset `pos` in the script.
     */
    getGdtInventory(script: string, pos: number): WxPromise<InvView_Gdt>;

    /**
     * Execute the script if not up-to-date, and return the overworld item view
     * at the byte offset `pos` in the script.
     */
    getOverworldItems(
        script: string,
        pos: number,
    ): WxPromise<InvView_Overworld>;

    // getRuntimeDiagnostics(
    //     script: string,
    // ): WorkexPromise<{ range: [number, number]; message: string }[]>;
}
