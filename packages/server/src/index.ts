import { routeBuilder, useLogging } from "server/framework";
import { createApiRoutes } from "server/api";
import { createAppRoutes } from "server/app";

import { createCrypto, randomKey } from "server/crypto.ts";

import { VERSION } from "./version.ts";
console.log("version: " + VERSION);

/** === Environment Initialization === */

// Initialize the crypto object
const crypto = (() => {
    let masterKey = process.env.SKYBOOK_CRYPTO_KEY;
    process.env.SKYBOOK_CRYPTO_KEY = "";
    if (!masterKey) {
        console.warn("crypto key is not provided, generating a random key");
        masterKey = randomKey();
    }
    const crypto = createCrypto(masterKey);
    if ("err" in crypto) {
        throw new Error(crypto.err);
    }
    return crypto.val;
})();

const hostname = "0.0.0.0";
const port = 8000;
console.log("starting server on http://" + hostname + ":" + port);

const builder = routeBuilder().inbound(useLogging);

Bun.serve({
    port,
    reusePort: true,
    hostname,
    routes: {
        ...(await createAppRoutes(builder)),
        ...createApiRoutes(crypto, builder),
    },
    // Global error handler
    error(error) {
        console.error(error);
        return new Response(`Internal Error: ${error.message}`, {
            status: 500,
            headers: {
                "Content-Type": "text/plain",
            },
        });
    },
});
