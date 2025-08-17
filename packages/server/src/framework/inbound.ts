import type { InboundHook } from "./types.ts";
import { safeParseUrl } from "./url.ts";

/** Logs incoming requests */
export const useLogging: InboundHook = (req) => {
    const method = req.method;
    const ua = req.headers.get("User-Agent") || "<no UA>";
    const url = safeParseUrl(req.url);
    const pathname = "err" in url ? "<failed to parse request url>" : url.val.pathname;
    console.log(`${new Date().toISOString()} [${ua}] ${method} - ${pathname}`);
    return { val: undefined };
};
