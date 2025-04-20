import { create } from "zustand";
import { debounce } from "@pistonite/pure/sync";
import { useDebounce } from "@uidotdev/usehooks";

import type { ScriptEnvImage } from "@pistonite/skybook-api";
import { translateUI } from "skybook-localization";

import { useApplicationStore } from "./ApplicationStore.ts";
import { charPosToBytePos } from "@pistonite/intwc";

/** State of the current session. This is not persisted */
export type SessionStore = {
    /** Mode of the current session */
    mode: SessionMode;

    /**
     * The initial script when the mode is edit-only, used for comparring
     * if the script has changed
     */
    initialScript: string;

    /**
     * Set the session mode to local
     *
     * activeScript will be saved to local storage immediately,
     * overriding the previous script
     */
    setModeToLocal: () => void;

    /**
     * Set the session mode to edit-only
     *
     * Both activeScript and initialScript will be set to the given script,
     * use undefined to use the current activeScript
     */
    setModeToEditOnly: (initialScript: string | undefined) => void;

    /**
     * Set the session mode to read-only
     *
     * If the script is not undefined, the activeScript will be set
     * to that
     */
    setModeToReadOnly: (script: string | undefined) => void;

    /**
     * If activeScript is different from the script in localStorage
     *
     * When rendering UI that depends on this value, use the debounced
     * hook useDebouncedHasUnsavedChanges
     */
    hasUnsavedChanges: boolean;
    setHasUnsavedChanges: (value: boolean) => void;

    /** The script that is currently being edited */
    activeScript: string;
    setActiveScript: (script: string) => void;

    /** The version of the custom image that is currently running */
    runningCustomImageVersion: ScriptEnvImage | "";
    setRunningCustomImageVersion: (value: ScriptEnvImage | "") => void;

    /** Current byte position of the active selection (caret) in the script */
    bytePos: number;
    setBytePos: (bytePos: number) => void;
    setCharPos: (charPos: number) => void;
};

/**
 * Mode of the current session
 *
 * - local: edits are saved to local storage immediately
 * - edit-only: edits are only in-memory
 * - read-only: edits not allowed
 */
export type SessionMode = "local" | "edit-only" | "read-only";

export const useSessionStore = create<SessionStore>()((set) => {
    const { savedScript } = useApplicationStore.getState();

    // Hook to prompt user when closing the tab
    window.onbeforeunload = () => {
        // fallback message in case translation fails
        const fallbackMessage = "Unsaved changes will be lost!";
        const { mode, hasUnsavedChanges } = useSessionStore.getState();
        if (!hasUnsavedChanges) {
            return undefined;
        }
        let message = undefined;
        if (mode === "local") {
            message = translateUI("prompt.closing.local");
        } else {
            message = translateUI("prompt.closing.edit");
        }
        if (message && typeof message === "string") {
            return message;
        }
        return fallbackMessage;
    };

    const persistScript = debounce({
        interval: 50,
        fn: (script: string) => {
            const { setSavedScript } = useApplicationStore.getState();
            setSavedScript(script);
            set({ hasUnsavedChanges: false });
        },
    });

    return {
        mode: "local",
        initialScript: "",
        setModeToLocal: () => {
            set(({ activeScript }) => {
                const { setSavedScript } = useApplicationStore.getState();
                setSavedScript(activeScript);
                window.history.pushState({}, "", "/");
                return {
                    mode: "local",
                    hasUnsavedChanges: false,
                };
            });
        },
        setModeToEditOnly: (initialScript) => {
            if (initialScript === undefined) {
                set(({ activeScript }) => {
                    return {
                        mode: "edit-only",
                        initialScript: activeScript,
                        hasUnsavedChanges: false,
                    };
                });
                return;
            }
            set(({ activeScript }) => {
                return {
                    mode: "edit-only",
                    initialScript,
                    hasUnsavedChanges: activeScript !== initialScript,
                };
            });
        },
        setModeToReadOnly: (script) => {
            if (script !== undefined) {
                set({
                    mode: "read-only",
                    hasUnsavedChanges: false,
                    activeScript: script,
                });
                return;
            }
            set({
                mode: "read-only",
                hasUnsavedChanges: false,
            });
        },

        hasUnsavedChanges: false,
        setHasUnsavedChanges: (value) => set({ hasUnsavedChanges: value }),

        activeScript: savedScript,
        setActiveScript: (script) => {
            set(({ mode, initialScript }) => {
                if (mode === "read-only") {
                    return {};
                }
                if (mode === "edit-only") {
                    const hasUnsavedChanges = initialScript !== script;
                    return {
                        activeScript: script,
                        hasUnsavedChanges,
                    };
                }
                const { savedScript } = useApplicationStore.getState();
                const hasUnsavedChanges = savedScript !== script;
                setTimeout(() => persistScript(script), 0);
                return {
                    activeScript: script,
                    hasUnsavedChanges,
                };
            });
        },

        runningCustomImageVersion: "",
        setRunningCustomImageVersion: (value) => {
            set({ runningCustomImageVersion: value });
        },

        bytePos: 0,
        setBytePos: (bytePos) => {
            set({ bytePos });
        },
        setCharPos: (charPos) => {
            set(({ activeScript }) => {
                return { bytePos: charPosToBytePos(activeScript, charPos) };
            });
        },
    };
});

/**
 * Get the debounced value of hasUnsavedChanges of the session
 */
export const useDebouncedHasUnsavedChanges = (delay: number) => {
    const hasUnsavedChanges = useSessionStore(
        (state) => state.hasUnsavedChanges,
    );
    return useDebounce(hasUnsavedChanges, delay);
};
