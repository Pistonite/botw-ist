import { useApplicationStore } from "./ApplicationStore.ts";

export const loadRecoveryScriptIfNeeded = () => {
    try {
        const script = localStorage.getItem("Skybook.RecoveryScript");
        if (script === null) {
            return;
        }
        localStorage.removeItem("Skybook.RecoveryScript");
        useApplicationStore.getState().setSavedScript(script);
        console.log("[boot] recovery script detected and set");
    } catch {
        // Ignore errors
    }
};

let crashHandler: () => void = () => {
    console.error("No crash handler registered!!!");
    alert(
        "App crashed but no crash handler is registered. Please report this issue.",
    );
    window.location.reload();
};
export const registerCrashHandler = (handler: () => void) => {
    crashHandler = handler;
};
export const crashApp = () => {
    console.log("=== crashing app ===");
    crashHandler();
};
