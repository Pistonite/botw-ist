import type { Result } from "@pistonite/pure/result";
import type { BunRequest } from "bun";

export type ResponsePayload = {
    body?: Bun.BodyInit;
    options?: ResponseInit;
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
    req: BunRequest,
) => Awaitable<Result<ResponsePayload | undefined, ResponsePayload>>;

export type Handler = (req: BunRequest) => Awaitable<ResponsePayload>;

export type OutboundHook = (
    req: BunRequest,
    ok: boolean,
    response: ResponsePayload,
) => Awaitable<ResponsePayload>;

export type RouteArgs = {
    inbound?: InboundHook[];
    handler: Handler;
    outbound?: OutboundHook[];
};

export type BunRequestHandler = (req: BunRequest) => Awaitable<Response>;

export type Routes = Record<
    string,
    BunRequestHandler | Response | Record<string, BunRequestHandler>
>;
