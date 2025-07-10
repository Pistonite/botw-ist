/**
 * Handle app crashes
 *
 * Currently, crashes could happen in 2 scenarios:
 * - Uncaught exception during renderring, which would normally cause
 *   the entire UI to not render
 * - Panic in WASM. Since Rust WASM only has panic-abort, the WASM moduling can
 *   no longer be used after panicking.
 */
import { errstr } from "@pistonite/pure/result";

import { bootLog, log } from "self::util";

import { usePersistStore } from "./persist_store.ts";

export const loadRecoveryScriptIfNeeded = () => {
    try {
        const script = localStorage.getItem("Skybook.RecoveryScript");
        if (script === null) {
            return;
        }
        localStorage.removeItem("Skybook.RecoveryScript");
        usePersistStore.getState().setSavedScript(script);
        bootLog.info("recovery script detected and set");
    } catch (e) {
        bootLog.debug(
            `detected error during boot recovery script, ignored: ${errstr(e)}`,
        );
    }
};

let crashHandler: () => void = () => {
    log.error("No crash handler registered!!!");
    alert(
        "App crashed but no crash handler is registered. Please report this issue.",
    );
    window.location.reload();
};
export const registerCrashHandler = (handler: () => void) => {
    crashHandler = handler;
};

let crashed = false;
export const crashApp = () => {
    if (crashed) {
        log.warn("crash handler invoked multiple times");
        return;
    }
    crashed = true;
    log.error("CRASHING APP");
    crashHandler();
};

export const isCrashed = () => crashed;
