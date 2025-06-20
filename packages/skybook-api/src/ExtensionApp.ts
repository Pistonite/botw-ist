import type { Result } from "@pistonite/pure/result";
import type { WxPromise } from "@pistonite/workex";

import type { Diagnostic, ItemSearchResult } from "./types.ts";

/**
 * API implemented by the application and called by the extension.
 */
export interface ExtensionApp {
    /** Get the current simulator script. */
    getScript(): WxPromise<string>;

    /**
     * Set the simulator script.
     *
     * This will trigger a rerun of the simulation using the new script
     *
     * Position is the current cursor position in the script as
     * a character offset (not byte offset) and is 0-based.
     */
    setScript(script: string, position: number): WxPromise<void>;

    /**
     * Resolve an item from a query
     *
     * If localized is true, treat the query as a localized item search query (i.e. "[tag:]words"),
     * otherwise, treat it as an identifier search query.)
     *
     * A localized error maybe returned if the query is invalid. However,
     * even when there is no error, the search result could be empty.
     */
    resolveItem(
        query: string,
        localized: boolean,
        limit: number,
    ): WxPromise<Result<ItemSearchResult[], string>>;

    /**
     * Invoke the parser for the script and get the diagnostics.
     */
    provideParserDiagnostics(script: string): WxPromise<Diagnostic[]>;

    /**
     * Get the semantic tokens for the script in the range.
     *
     * The output is triples of [start, length, tokenType].
     *
     * The offsets in both inputs and outputs should be character offsets, not byte offsets.
     * (Note this is different from Runtime.getSemanticTokens)
     */
    provideSemanticTokens(
        script: string,
        start: number,
        end: number,
    ): WxPromise<Uint32Array>;

    /**
     * Get the diagnostics from running the script.
     *
     * The taskId should be a UUID, which can be used to cancel the diagnostic
     * request if it's no longer needed to save CPU resource
     */
    provideRuntimeDiagnostics(
        script: string,
        taskId: string,
    ): WxPromise<Diagnostic[]>;

    /**
     * Cancel a previous request for runtime diagnostics.
     */
    cancelRuntimeDiagnosticsRequest(taskId: string): WxPromise<void>;
}
