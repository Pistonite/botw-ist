/**
 * Request Framework
 *
 * This is a very simple framework built on top of Bun server.
 *
 * Each route is built with 3 components: Inbound hooks -> handler -> and outbound hooks.
 *
 * Each request is first passed through the inbound hooks. Each
 * inbound hook can:
 * - execute side effect (such as logging)
 * - short-circuit the request by returning an Ok<ResponsePayload>
 *   - Note the Ok just means no error happened during handling
 *   - The response payload can contain a 4XX or 5XX error response
 * - indicate an unexpected error by returning an Err<ResponsePayload>
 *
 * Inbound hooks can technically modify the request, but that is not recommended
 * as it makes the behavior not obvious in the handler.
 *
 * If the request is short-circuited by an inbound hook (through success or error response),
 * the request will then go directly to the outbound hooks.
 * Otherwise, the request is passed through the handler,
 * which is a (Request, URL) => ResponsePayload function.
 *
 * Note the handler does not return a Result<ResponsePayload>.
 * This is to make the handling more explicit. Uncaught errors
 * will become Err<ResponsePayload>, and the returned value will become Ok<ResponsePayload>
 *
 * Finally, the result is turned into [ok, response] and transformed
 * by the outbound hooks.
 *
 *
 * In this framework, the middleware are not magic that can extract
 * information from the request and pass to the handler. It's discouraged
 * to do that because it makes it harder to understand where the data comes from
 * by looking at the handler.
 *
 * Instead, the information are extracted by extractor functions in the handler.
 * These functions take in the request and other arguments, and return the extracted information.
 * They can also short-circuit the request in case of error by throwing a ResponsePayload.
 *
 * Naming convention:
 * - Both inbound hooks and extractors should be useXXX
 * - Handlers should be makeXXX. XXX should not end with 'Response', as it's implied
 * - outbound hooks should be withXXX
 *
 * Use `route()` to build a route handler. If shared inbound/outbound hook sequences
 * are needed for multiple routes (for example, logging), use `builder()`.
 */

export * from "./types.ts";
export * from "./route.ts";
export * from "./extractors.ts";
export * from "./handlers.ts";
export * from "./inbound.ts";
export * from "./outbound.ts";
