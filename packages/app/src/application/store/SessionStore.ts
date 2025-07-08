import { create } from "zustand";
import { debounce } from "@pistonite/pure/sync";
import type { Result } from "@pistonite/pure/result";
import { charPosToBytePos } from "@pistonite/intwc";

import type {
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    RuntimeViewError,
    ScriptEnvImage,
} from "@pistonite/skybook-api";
import { translateUI } from "skybook-localization";

import { useApplicationStore } from "./ApplicationStore.ts";

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
    setActiveScript: (script: string, charPos: number) => void;

    /** The version of the custom image that is currently running */
    runningCustomImageVersion: ScriptEnvImage | "";
    setRunningCustomImageVersion: (value: ScriptEnvImage | "") => void;

    /** Current byte position of the active selection (caret) in the script */
    bytePos: number;
    /** Current step index of the active selection */
    stepIndex: number;
    setStepIndex: (stepIndex: number) => void;
    initiallyExecuted: boolean;
    inProgressTaskId: string;
    setInProgressTaskId: (id: string) => void;

    // The inventory view errors (RuntimeViewError) is not an error in the app
    // It's expected if the simulation corrupted the game in some way
    // so the inventory cannot be read

    /** Cached Pouch list views. Key is the step index */
    pouchCached: number[];
    pouchViews: Record<number, Result<InvView_PouchList, RuntimeViewError>>;
    setPouchViewInCache: (
        step: number,
        view: Result<InvView_PouchList, RuntimeViewError>,
    ) => void;
    /** Cached GDT inventory views. Key is the step index */
    gdtCached: number[];
    gdtViews: Record<number, Result<InvView_Gdt, RuntimeViewError>>;
    setGdtViewInCache: (
        step: number,
        view: Result<InvView_Gdt, RuntimeViewError>,
    ) => void;
    /** Cached Overworld item views. Key is the step index */
    overworldCached: number[];
    overworldViews: Record<number, Result<InvView_Overworld, RuntimeViewError>>;
    setOverworldViewInCache: (
        step: number,
        view: Result<InvView_Overworld, RuntimeViewError>,
    ) => void;

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
            set(({ mode, activeScript, initialScript }) => {
                const newScript = activeScript === script ? undefined : script;
                if (mode === "read-only") {
                    return getSetActiveScriptPayload(
                        activeScript,
                        newScript,
                        false,
                        charPos,
                    );
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
                const { savedScript } = useApplicationStore.getState();
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

        initiallyExecuted: false,
        inProgressTaskId: "",
        setInProgressTaskId: (id) => {
            if (id) {
                set({ inProgressTaskId: id, initiallyExecuted: true });
            } else {
                set({ inProgressTaskId: id });
            }
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

        perfIps: -1,
        perfSps: -1,
        setPerfData: (ips, sps) => {
            set({ perfIps: ips, perfSps: sps });
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
