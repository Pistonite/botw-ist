/**
 * This file is generated by workex
 */
import type { Runtime } from "../Runtime.ts";

import { type WorkexPromise, WorkexClient, type WorkexClientOptions } from "@pistonite/workex";
import type { Void } from "@pistonite/pure/result";
import type { ParserErrorReport } from ".././parser";
import type { ItemSearchResult, RuntimeInitArgs, RuntimeInitError } from ".././types.ts";

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
     * Parse the script and get diagnostics from the parser.
     * 
     * Note that the span in the errors are byte offsets, not character offsets.
     */
    public getParserDiagnostics( script: string ): WorkexPromise<ParserErrorReport[]> {
        return this.client.post<ParserErrorReport[]>(24 /* Runtime.getParserDiagnostics */, [ script ]);
    }

    /**
     * Parse the script and get semantic tokens in the range from the parser.
     * 
     * The output is triples of [start, length, tokenType]
     * 
     * The offsets in both inputs and outputs should be byte offsets, not character offsets.
     */
    public getSemanticTokens( script: string, start: number, end: number ): WorkexPromise<Uint32Array> {
        return this.client.post<Uint32Array>(25 /* Runtime.getSemanticTokens */, [ script, start, end ]);
    }

    /**
     * Initialize the runtime with the given arguments.
     */
    public initialize( args: RuntimeInitArgs ): WorkexPromise<Void<RuntimeInitError>> {
        return this.client.post<Void<RuntimeInitError>>(26 /* Runtime.initialize */, [ args ]);
    }

    /**
     * Resolve an item identifier search query to a list of items, ordered by score (best first).
     * Returns an empty list if no items are found.
     */
    public resolveItemIdent( query: string ): WorkexPromise<ItemSearchResult[]> {
        return this.client.post<ItemSearchResult[]>(27 /* Runtime.resolveItemIdent */, [ query ]);
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