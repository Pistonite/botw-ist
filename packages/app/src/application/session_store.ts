/**
 * The state for current session
 */

import { create } from "zustand";
import { debounce } from "@pistonite/pure/sync";
import type { Result } from "@pistonite/pure/result";
import { charPosToBytePos } from "@pistonite/intwc";
import { useDebounce } from "@uidotdev/usehooks";

import type {
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    RuntimeViewError,
    ScriptEnvImage,
    SessionMode,
} from "@pistonite/skybook-api";
import { translateUI } from "skybook-localization";

import { usePersistStore } from "./persist_store.ts";
import { log } from "self::util";

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
     * initialScript will be set to the given script,
     * use undefined to use the current activeScript.
     * Current activeScript will not be changed
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
    setActiveScript: (script: string, charPos: number) => void;

    /** The version of the custom image that is currently running */
    runningCustomImageVersion: ScriptEnvImage | "";
    setRunningCustomImageVersion: (value: ScriptEnvImage | "") => void;

    /** Current byte position of the active selection (caret) in the script */
    bytePos: number;
    /** Current step index of the active selection */
    stepIndex: number;
    setStepIndex: (stepIndex: number) => void;
    /**
     * Currently active background script execution ID
     * usually, non-empty means a run is being executed
     * in the background
     */
    inProgressTaskId: string;
    setInProgressTaskId: (id: string) => void;

    // The inventory view errors (RuntimeViewError) is not an error in the app
    // It's expected if the simulation corrupted the game in some way
    // so the inventory cannot be read

    /** Cached Pouch list views. Key is the step index */
    pouchCached: number[];
    pouchViews: Record<number, Result<InvView_PouchList, RuntimeViewError>>;
    setPouchViewInCache: (step: number, view: Result<InvView_PouchList, RuntimeViewError>) => void;
    /** Cached GDT inventory views. Key is the step index */
    gdtCached: number[];
    gdtViews: Record<number, Result<InvView_Gdt, RuntimeViewError>>;
    setGdtViewInCache: (step: number, view: Result<InvView_Gdt, RuntimeViewError>) => void;
    /** Cached Overworld item views. Key is the step index */
    overworldCached: number[];
    overworldViews: Record<number, Result<InvView_Overworld, RuntimeViewError>>;
    setOverworldViewInCache: (
        step: number,
        view: Result<InvView_Overworld, RuntimeViewError>,
    ) => void;

    /** Invalidate all cached inventory views */
    invalidateInventoryCache: () => void;
};

export const useSessionStore = create<SessionStore>()((set) => {
    const { savedScript } = usePersistStore.getState();

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
            const { setSavedScript } = usePersistStore.getState();
            setSavedScript(script);
            set({ hasUnsavedChanges: false });
        },
    });

    return {
        mode: "local",
        initialScript: "",
        setModeToLocal: () => {
            log.info("changing app mode to local");
            set(({ activeScript }) => {
                const { savedScript, setSavedScript } = usePersistStore.getState();
                setSavedScript(activeScript);
                // save backup if user needs
                localStorage.setItem("Skybook.AutoBackupScript", savedScript);
                // clears the embbeded script in the URL
                window.history.pushState({}, "", "/");
                return {
                    mode: "local",
                    hasUnsavedChanges: false,
                };
            });
        },
        setModeToEditOnly: (initialScript) => {
            log.info("changing app mode to edit-only");
            set(({ activeScript }) => {
                if (initialScript === undefined) {
                    return {
                        mode: "edit-only",
                        initialScript: activeScript,
                        hasUnsavedChanges: false,
                    };
                }
                return {
                    mode: "edit-only",
                    initialScript,
                    hasUnsavedChanges: initialScript !== activeScript,
                };
            });
        },
        setModeToReadOnly: (script) => {
            log.info("changing app mode to read-only");
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
            set(({ mode, activeScript, initialScript }) => {
                const newScript = activeScript === script ? undefined : script;
                if (mode === "read-only") {
                    return getSetActiveScriptPayload(activeScript, newScript, false, charPos);
                }
                if (mode === "edit-only") {
                    const hasUnsavedChanges = initialScript !== script;
                    return getSetActiveScriptPayload(
                        activeScript,
                        newScript,
                        hasUnsavedChanges,
                        charPos,
                    );
                }
                const { savedScript } = usePersistStore.getState();
                const hasUnsavedChanges = savedScript !== script;
                setTimeout(() => persistScript(script), 0);
                return getSetActiveScriptPayload(
                    activeScript,
                    newScript,
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
        stepIndex: 0,
        setStepIndex: (stepIndex) => {
            set({ stepIndex });
        },

        inProgressTaskId: "",
        setInProgressTaskId: (id) => {
            set({ inProgressTaskId: id });
        },

        pouchCached: [],
        pouchViews: {},
        setPouchViewInCache: (step, view) => {
            set(({ pouchViews, pouchCached }) => {
                return {
                    pouchViews: {
                        ...pouchViews,
                        [step]: view,
                    },
                    pouchCached: [...pouchCached, step],
                };
            });
        },
        gdtCached: [],
        gdtViews: {},
        setGdtViewInCache: (step, view) => {
            set(({ gdtViews, gdtCached }) => {
                return {
                    gdtViews: {
                        ...gdtViews,
                        [step]: view,
                    },
                    gdtCached: [...gdtCached, step],
                };
            });
        },
        overworldCached: [],
        overworldViews: {},
        setOverworldViewInCache: (step, view) => {
            set(({ overworldViews, overworldCached }) => {
                return {
                    overworldViews: {
                        ...overworldViews,
                        [step]: view,
                    },
                    overworldCached: [...overworldCached, step],
                };
            });
        },
        invalidateInventoryCache: () => {
            set({
                gdtCached: [],
                pouchCached: [],
                overworldCached: [],
            });
        },
    };
});

const getSetActiveScriptPayload = (
    currentScript: string,
    newScript: string | undefined,
    hasUnsavedChanges: boolean,
    charPos: number,
) => {
    if (newScript === undefined) {
        return {
            hasUnsavedChanges,
            bytePos: charPosToBytePos(currentScript, charPos),
        };
    }
    return {
        activeScript: newScript,
        hasUnsavedChanges,
        bytePos: charPosToBytePos(newScript, charPos),
    };
};

/**
 * Get the debounced value of hasUnsavedChanges of the session
 */
export const useDebouncedHasUnsavedChanges = () => {
    const hasUnsavedChanges = useSessionStore((state) => state.hasUnsavedChanges);
    return useDebounce(hasUnsavedChanges, 50);
};
