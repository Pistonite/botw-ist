import { create } from "zustand";
import { debounce } from "@pistonite/pure/sync";

import type {
    InvView_Gdt,
    InvView_PouchList,
    ScriptEnvImage,
} from "@pistonite/skybook-api";
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
    setActiveScript: (script: string, charPos?: number) => void;

    /** The version of the custom image that is currently running */
    runningCustomImageVersion: ScriptEnvImage | "";
    setRunningCustomImageVersion: (value: ScriptEnvImage | "") => void;

    /** Current byte position of the active selection (caret) in the script */
    bytePos: number;
    setBytePos: (bytePos: number) => void;
    setCharPos: (charPos: number) => void;

    /** Current step index of the active selection */
    stepIndex: number;
    setStepIndex: (stepIndex: number) => void;
    initiallyExecuted: boolean;
    executionInProgress: boolean;
    setExecutionInProgress: (inProgress: boolean) => void;

    /** Cached Pouch list views. Key is the step index */
    upToDatePouchSteps: number[];
    pouchViews: Record<number, InvView_PouchList>;
    setPouchViewInCache: (step: number, view: InvView_PouchList) => void;
    /** Cached GDT inventory views. Key is the step index */
    upToDateGdtSteps: number[];
    gdtViews: Record<number, InvView_Gdt>;
    setGdtViewInCache: (step: number, view: InvView_Gdt) => void;
    /** Invalidate all cached inventory views */
    invalidateInventoryCache: () => void;

    /** Performance: Instructions Per Second */
    perfIps: number;
    /** Performance: Steps Per Second */
    perfSps: number;
    setPerfData: (ips: number, sps: number) => void;
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
        setActiveScript: (script, charPos) => {
            set(({ mode, initialScript }) => {
                if (mode === "read-only") {
                    return {};
                }
                if (mode === "edit-only") {
                    const hasUnsavedChanges = initialScript !== script;
                    return getSetActiveScriptPayload(
                        script,
                        hasUnsavedChanges,
                        charPos,
                    );
                }
                const { savedScript } = useApplicationStore.getState();
                const hasUnsavedChanges = savedScript !== script;
                setTimeout(() => persistScript(script), 0);
                return getSetActiveScriptPayload(
                    script,
                    hasUnsavedChanges,
                    charPos,
                );
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

        stepIndex: 0,
        setStepIndex: (stepIndex) => {
            set({ stepIndex });
        },

        initiallyExecuted: false,
        executionInProgress: false,
        setExecutionInProgress: (inProgress) => {
            if (inProgress) {
                set({ executionInProgress: true, initiallyExecuted: true });
            } else {
                set({ executionInProgress: false });
            }
        },

        upToDatePouchSteps: [],
        pouchViews: {},
        setPouchViewInCache: (step, view) => {
            set(({ pouchViews, upToDatePouchSteps }) => {
                return {
                    pouchViews: {
                        ...pouchViews,
                        [step]: view,
                    },
                    upToDatePouchSteps: [...upToDatePouchSteps, step],
                };
            });
        },
        upToDateGdtSteps: [],
        gdtViews: {},
        setGdtViewInCache: (step, view) => {
            set(({ gdtViews, upToDateGdtSteps }) => {
                return {
                    gdtViews: {
                        ...gdtViews,
                        [step]: view,
                    },
                    upToDateGdtSteps: [...upToDateGdtSteps, step],
                };
            });
        },
        invalidateInventoryCache: () => {
            set({
                upToDatePouchSteps: [],
                upToDateGdtSteps: [],
            });
        },

        perfIps: -1,
        perfSps: -1,
        setPerfData: (ips, sps) => {
            set({ perfIps: ips, perfSps: sps });
        },
    };
});

const getSetActiveScriptPayload = (
    script: string,
    hasUnsavedChanges: boolean,
    charPos: number | undefined,
) => {
    if (charPos !== undefined) {
        return {
            activeScript: script,
            hasUnsavedChanges,
            bytePos: charPosToBytePos(script, charPos),
        };
    }
    return {
        activeScript: script,
        hasUnsavedChanges,
    };
};
