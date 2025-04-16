import { routeBuilder, useLogging } from "self::framework";
import { createCrypto, randomKey, VERSION } from "self::util";

import { createApiRoutes } from "./api/router.ts";
import { createAppRoutes } from "./app/router.ts";

const initCrypto = () => {
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
};

const initTls = async () => {
    const certPath = process.env.SKYBOOK_TLS_PATH;
    if (!certPath) {
        return undefined;
    }
    console.log("loading tls key/cert from", certPath);
    const key = Bun.file(certPath + "/cert.key");
    const cert = Bun.file(certPath + "/cert.pem");
    let hostname: string | undefined = undefined;
    try {
        const text = await cert.text();
        for (const line of text.split("\n")) {
            const l = line.trim();
            if (l.toLowerCase().startsWith("subject=")) {
                const rest = l.substring("subject=".length);
                // find the CN (common name)
                const parts = rest.split(",");
                for (const part of parts) {
                    const p = part.trim();
                    const [k, v] = p.split("=");
                    if (k.trim().toLowerCase() === "cn") {
                        hostname = v.trim();
                        break;
                    }
                }
                break;
            }
        }
    } catch (e) {
        console.error("failed to load tls key/cert", e);
    }
    return {
        hostname,
        key,
        cert,
    };
};

async function main() {
    console.log("version: " + VERSION);

    const crypto = initCrypto();
    const tls = await initTls();

    const port = parseInt(process.env.SKYBOOK_PORT || "80");
    console.log(
        `starting server on http${tls ? "s" : ""}://${tls?.hostname || "localhost"}:${port}`,
    );

    const builder = routeBuilder().inbound(useLogging);

    Bun.serve({
        port,
        reusePort: true,
        hostname: "0.0.0.0",
        routes: {
            ...createAppRoutes(builder),
            ...createApiRoutes(crypto, builder),
        },
        tls: tls
            ? {
                  key: tls.key,
                  cert: tls.cert,
              }
            : undefined,
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
}

void main();
