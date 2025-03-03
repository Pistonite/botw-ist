import type { Result } from "@pistonite/pure/result";

// eslint-disable-next-line @typescript-eslint/consistent-type-imports
export type URL = import("url").URL;

export type ResponsePayload = {
    body?: Bun.BodyInit;
    options?: Bun.ResponseInit;
};

type Awaitable<T> = T | Promise<T>;

/**
 * Inbound hook
 *
 * Return:
 * - Ok<undefined> to continue to the next hook or handler
 * - Ok<ResponsePayload> or Err<ResponsePayload> to short-circuit the request
 */
export type InboundHook = (
    req: Request,
    url: URL,
) => Awaitable<Result<ResponsePayload | undefined, ResponsePayload>>;

export type Handler = (req: Request, url: URL) => Awaitable<ResponsePayload>;

export type OutboundHook = (
    req: Request,
    url: URL,
    ok: boolean,
    response: ResponsePayload,
) => Awaitable<ResponsePayload>;

export type RouteArgs = {
    inbound?: InboundHook[];
    handler: Handler;
    outbound?: OutboundHook[];
};

export type BunRequestHandler = (req: Request) => Awaitable<Response>;
