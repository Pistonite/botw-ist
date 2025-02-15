/**
 * This file is generated by workex
 */
import type { Runtime } from "../Runtime.ts";

import { type WorkexPromise, WorkexClient, type WorkexClientOptions } from "@pistonite/workex";
import { ParserErrorReport } from ".././parser";
import { ItemSearchResult } from ".././types.ts";

/**
 * API provided by the simulator runtime, called by the application.
 * 
 * @workex:send app
 * @workex:recv runtime
 */
export class RuntimeClient implements Runtime {
    private client: WorkexClient<"skyb-api-0.0.1">

    constructor(options: WorkexClientOptions) {
        this.client = new WorkexClient("skyb-api-0.0.1", options);
    }

    /**
     * Set the script for the runtime, which starts executing
     * the script immediately
     * 
     * onScriptChange(script: string): WorkexPromise<void>;
     * getSemanticTokens(
     * script: string,
     * startPos: number,
     * endPos: number,
     * ): WorkexPromise<Uint32Array>;
     * 
     * Parse the script and get diagnostics from the parser.
     * 
     * This does not runtime diagnostics
     */
    public getParserDiagnostics( script: string ): WorkexPromise<ParserErrorReport[]> {
        return this.client.post<ParserErrorReport[]>(22 /* Runtime.getParserDiagnostics */, [ script ]);
    }

    /**
     * Resolve an item identifier search query to a list of items, ordered by score (best first).
     * Returns an empty list if no items are found.
     */
    public resolveItemIdent( query: string ): WorkexPromise<ItemSearchResult[]> {
        return this.client.post<ItemSearchResult[]>(23 /* Runtime.resolveItemIdent */, [ query ]);
    }

    /**
     * Terminate the client and the underlying worker
     *
     * This method is generated by workex
     */
    public terminate() {
        this.client.terminate();
    }

    /**
     * Get the protocol identifier used by the underlying workex communication
     *
     * This method is generated by workex
     */
    public protocol(): "skyb-api-0.0.1" {
        return "skyb-api-0.0.1";
    }

    /**
     * Create a client-only handshake
     *
     * Generally, handshakes should be created using the `bindHost` function on each side.
     * However, if one side is a client-only side, this method can be used to bind a stub host
     * to establish the handshake.
     *
     * This method is generated by workex
     */
    public handshake() {
        return this.client.handshake();
    }
}