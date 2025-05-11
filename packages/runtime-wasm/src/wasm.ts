/** All the weird bonkers stuff we need to do because wasm */

import type { Result } from "@pistonite/pure/result";

/**
 * Safe wrappers around panics from WASM
 *
 * Because WASM exceptions are not implemented yet, rust currently
 * does not support unwinding, so panicking from WASM will potentially
 * leave the module in a bad state, and we should not try to use
 * it anymore.
 *
 * The panic handler will send this information to let the app
 * display a fatal error UI and force the user to reload the page.
 */
let wasmPanicked = false;
let wasmPanicHandler: () => void = () => {
    console.warn("Forgot to set WASM panic handler?");
};

// This is a hack to make WASM able to invoke crash directly
// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any)["__global_crash_handler"] = () => {
    console.error("Panic invoked from WASM. Recovery is NOT possible!");
    wasmPanicked = true;
    wasmPanicHandler();
};

export const setWasmPanicHandler = (handler: () => void) => {
    wasmPanicHandler = handler;
};

/** Execute the closure if WASM did not previously panic */
export const safeExecWasm = async <T>(
    fn: () => T | Promise<T>,
): Promise<Result<Awaited<T>, "panic">> => {
    if (wasmPanicked) {
        return { err: "panic" };
    }
    try {
        return { val: await fn() };
    } catch (e) {
        console.error(e);
        console.error("Panic detected in WASM. Recovery is NOT possible!");
        wasmPanicked = true;
        wasmPanicHandler();
    }
    return { err: "panic" };
};
