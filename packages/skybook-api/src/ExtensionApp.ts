import type { Result } from "@pistonite/pure/result";
import type { WorkexPromise } from "@pistonite/workex";

import { Diagnostic, ItemSearchResult } from "./types.ts";

/**
 * API implemented by the application and called by the extension.
 *
 * @workex:send extension
 * @workex:recv app
 */
export interface ExtensionApp {
    /** Get the current simulator script. */
    getScript(): WorkexPromise<string>;

    /** 
     * Set the simulator script.
     *
     * This will trigger a rerun of the simulation using the new script
     */
    setScript(script: string): WorkexPromise<void>;

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
    ): WorkexPromise<Result<ItemSearchResult[], string>>;

    /**
     * Invoke the parser for the script and get the diagnostics.
     */
    provideParserDiagnostics(script: string): WorkexPromise<Diagnostic[]>;

    // /** Get the semantic tokens for the current script */
    // provideSemanticTokens(start: number, end: number): WorkexPromise<Uint32Array>;
}
