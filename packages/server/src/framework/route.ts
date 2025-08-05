import { type Result, errstr } from "@pistonite/pure/result";
import type { BunRequest } from "bun";

import type {
    BunRequestHandler,
    Handler,
    InboundHook,
    OutboundHook,
    ResponsePayload,
    RouteArgs,
} from "./types.ts";

/** Use a RouteBuilder to create routes with shared inbound and outbound hooks */
export type RouteBuilder = {
    inbound: (hook: InboundHook) => RouteBuilder;
    outbound: (hook: OutboundHook) => RouteBuilder;
    route: (args: RouteArgs | Handler) => BunRequestHandler;
};

export const routeBuilder = (): RouteBuilder => {
    const inboundHooks: InboundHook[] = [];
    const outboundHooks: OutboundHook[] = [];
    const builder = {
        inbound: (hook: InboundHook) => {
            inboundHooks.push(hook);
            return builder;
        },
        outbound: (hook: OutboundHook) => {
            outboundHooks.push(hook);
            return builder;
        },
        route: (args: RouteArgs | Handler) => {
            if (!inboundHooks.length && !outboundHooks.length) {
                return route(args);
            }
            if (typeof args === "function") {
                return route({
                    handler: args,
                    inbound: inboundHooks,
                    outbound: outboundHooks,
                });
            }
            const inboundHooksMerged = [...inboundHooks, ...(args.inbound ?? [])];
            const outboundHooksMerged = [...(args.outbound ?? []), ...outboundHooks];
            return route({
                handler: args.handler,
                inbound: inboundHooksMerged,
                outbound: outboundHooksMerged,
            });
        },
    };
    return builder;
};

export const route = (args: RouteArgs | Handler): BunRequestHandler => {
    if (typeof args === "function") {
        return route({ handler: args });
    }
    const handler = args.handler;
    const inbound = args.inbound;
    const outbound = args.outbound;
    if (inbound?.length && outbound?.length) {
        return async (req: BunRequest) => {
            const url = new URL(req.url) as URL;
            const inboundResult = await executeInboundHooks(req, url, inbound);
            if (inboundResult.err) {
                return handleOutboundHooks(req, url, false, inboundResult.err, outbound);
            }
            if (inboundResult.val) {
                return handleOutboundHooks(req, url, true, inboundResult.val, outbound);
            }
            const result = await executeHandler(req, url, handler);
            if (result.val) {
                return handleOutboundHooks(req, url, true, result.val, outbound);
            }
            return handleOutboundHooks(req, url, false, result.err, outbound);
        };
    }
    if (inbound?.length) {
        return async (req: BunRequest) => {
            const url = new URL(req.url) as URL;
            const inboundResult = await executeInboundHooks(req, url, inbound);
            if (inboundResult.err) {
                return handleResponsePayload(false, inboundResult.err);
            }
            if (inboundResult.val) {
                return handleResponsePayload(true, inboundResult.val);
            }
            const result = await executeHandler(req, url, handler);
            if (result.val) {
                return handleResponsePayload(true, result.val);
            }
            return handleResponsePayload(false, result.err);
        };
    }
    if (outbound?.length) {
        return async (req: BunRequest) => {
            const url = new URL(req.url) as URL;
            const result = await executeHandler(req, url, handler);
            if (result.val) {
                return handleOutboundHooks(req, url, true, result.val, outbound);
            }
            return handleOutboundHooks(req, url, false, result.err, outbound);
        };
    }
    return async (req: BunRequest) => {
        const url = new URL(req.url) as URL;
        const result = await executeHandler(req, url, handler);
        if (result.val) {
            return handleResponsePayload(true, result.val);
        }
        return handleResponsePayload(false, result.err);
    };
};

const executeInboundHooks = async (
    req: BunRequest,
    url: URL,
    hooks: InboundHook[],
): Promise<Result<ResponsePayload | undefined, ResponsePayload>> => {
    const len = hooks.length;
    for (let i = 0; i < len; i++) {
        const result = await hooks[i](req, url);
        // short-circuit
        if (result.val || result.err) {
            return result;
        }
    }
    // continue to handler
    return { val: undefined };
};

const executeHandler = async (
    req: BunRequest,
    url: URL,
    handler: Handler,
): Promise<Result<ResponsePayload, ResponsePayload>> => {
    try {
        return { val: await handler(req, url) };
    } catch (e) {
        console.error(e);
        if (e && typeof e === "object") {
            if ("body" in e && "options" in e && e.options && typeof e.options === "object") {
                // treat the thrown object as a ResponsePayload
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                return { err: e as any };
            }
        }
        return { err: { body: errstr(e), options: { status: 500 } } };
    }
};

const handleOutboundHooks = async (
    req: BunRequest,
    url: URL,
    ok: boolean,
    response: ResponsePayload,
    hooks: OutboundHook[],
): Promise<Response> => {
    const len = hooks.length;
    for (let i = 0; i < len; i++) {
        response = await hooks[i](req, url, ok, response);
    }
    return handleResponsePayload(ok, response);
};

const handleResponsePayload = async (ok: boolean, response: ResponsePayload): Promise<Response> => {
    if (ok) {
        if (!response.body) {
            console.warn("OK Response has no body!", response);
        }
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        return new Response(response.body as any, response.options as any);
    }
    // ensure an error response has a status
    const body = response.body ?? "Internal Error";
    if (!response.options) {
        response.options = { status: 500 };
    } else if (!response.options.status) {
        response.options = { ...response.options, status: 500 };
    }
    // cast - IDE issue with node/bun confusion
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return new Response(body as any, response.options as any);
};
