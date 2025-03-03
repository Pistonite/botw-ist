import type { InboundHook } from "./types.ts";

/** Logs incoming requests */
export const useLogging: InboundHook = (req, url) => {
    const method = req.method;
    console.log(`${new Date().toISOString()} ${method} - ${url.pathname}`);
    return { val: undefined };
};
