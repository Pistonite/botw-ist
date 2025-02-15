/**
 * This file is generated by workex
 */
import type { ExtensionApp } from "../ExtensionApp.ts";

import type { Result } from "@pistonite/pure/result";
import { type WorkexPromise, WorkexClient, type WorkexClientOptions } from "@pistonite/workex";
import { ItemSearchResult } from ".././types.ts";

/**
 * API implemented by the application and called by the extension.
 * 
 * @workex:send extension
 * @workex:recv app
 */
export class ExtensionAppClient implements ExtensionApp {
    private client: WorkexClient<"skyb-api-0.0.1">

    constructor(options: WorkexClientOptions) {
        this.client = new WorkexClient("skyb-api-0.0.1", options);
    }

    /**
     * Get the current simulator script.
     */
    public getScript( ): WorkexPromise<string> {
        return this.client.post<string>(19 /* ExtensionApp.getScript */, [ ]);
    }

    /**
     * Resolve an item from a query
     * 
     * If localized is true, treat the query as a localized item search query (i.e. "[tag:]words"),
     * otherwise, treat it as an identifier search query.)
     * 
     * A localized error maybe returned if the query is invalid. However,
     * even when there is no error, the search result could be empty.
     */
    public resolveItem( query: string, localized: boolean, limit: number ): WorkexPromise<Result<ItemSearchResult[], string>> {
        return this.client.post<Result<ItemSearchResult[], string>>(20 /* ExtensionApp.resolveItem */, [ query, localized, limit ]);
    }

    /**
     * Set the simulator script.
     */
    public setScript( script: string ): WorkexPromise<void> {
        return this.client.postVoid(21 /* ExtensionApp.setScript */, [ script ]);
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