/**
 * Handles the app's interaction with runtime worker
 */

import { serial } from "@pistonite/pure/sync";
import { wxWrapHandler } from "@pistonite/workex";
import type { Void } from "@pistonite/pure/result";
import { v4 as makeUUID } from "uuid";
import { createContext, useContext } from "react";

import type {
    ItemSearchResult,
    Runtime,
    RuntimeApp,
    RuntimeWorkerInitArgs,
    Translator,
} from "@pistonite/skybook-api";
import {
    searchItemLocalized,
    translateGenericError,
    translateRuntimeInitError,
} from "skybook-localization";

import { bootLog, log } from "self::util";

import { useSessionStore } from "./session_store.ts";
import { crashApp } from "./crash_handler.ts";
import { usePersistStore } from "./persist_store.ts";

/** Storage for the selected custom image */
let customImage: Uint8Array | undefined;

/**
 * Set the custom image to provide to the runtime if the runtime asks for it
 */
export const setCustomImageToProvide = (image: Uint8Array | undefined) => {
    customImage = image;
};

/** Create the API for runtime to call the app */
export const createRuntimeAppHost = (): RuntimeApp => {
    return {
        resolveQuotedItem: wxWrapHandler(
            async (query: string): Promise<ItemSearchResult | undefined> => {
                const result = await searchItemLocalized(query, 1);
                if ("err" in result || !result.val.length) {
                    return undefined;
                }
                const item = result.val[0];
                return item;
            },
        ),

        getCustomBlueFlameImage: wxWrapHandler(() => {
            return customImage;
        }),

        crashApplication: wxWrapHandler(() => {
            crashApp();
        }),
    };
};

/** Shared (app-side) initialization after runtime has been created and connected */
export const bootstrapAppWithRuntime = (runtime: Runtime): void => {
    // create a serial event for triggering simulation when state change
    const triggerSimulationAndUpdateState = serial({
        fn: (checkTaskCancel) => async (scriptChanged: boolean) => {
            const {
                invalidateInventoryCache,
                inProgressTaskId,
                setInProgressTaskId,
                activeScript,
                bytePos,
                setStepIndex,
            } = useSessionStore.getState();

            if (scriptChanged) {
                invalidateInventoryCache();
            }
            const checkCancel = () => {
                checkTaskCancel();
                const { activeScript: activeScriptNow } = useSessionStore.getState();
                if (activeScriptNow !== activeScript) {
                    // script changed while waiting for result
                    throw new Error("cancelled");
                }
            };
            // this only requires parsing, and should be fast
            const stepIndex = await runtime.getStepFromPos(activeScript, bytePos);
            checkCancel();
            if (stepIndex.err) {
                log.error("failed to get step index");
                log.error(stepIndex.err);
                return;
            }
            setStepIndex(stepIndex.val);
            if (!scriptChanged) {
                return;
            }
            // trigger a run to keep the script execution in the background
            // even if all other tasks are finished
            if (inProgressTaskId) {
                // abort previous run
                await runtime.abortTask(inProgressTaskId);
            }
            checkCancel();
            const taskId = makeUUID();
            log.debug(`${taskId}\nbackground execution start`);
            setInProgressTaskId(taskId);

            await runtime.executeScript(activeScript, taskId);
            if (useSessionStore.getState().inProgressTaskId === taskId) {
                log.debug(`${taskId}\nbackground idle`);
                setInProgressTaskId("");
            }
        },
    });

    // register a subscriber to reliably trigger the script execution
    // for the first time, then unregisters itself
    const unsubcribeInitialExecution = useSessionStore.subscribe((curr) => {
        if (curr.activeScript) {
            log.info("triggering script execution for the first time");
            unsubcribeInitialExecution();
            setTimeout(() => void triggerSimulationAndUpdateState(true), 0);
        }
    });

    useSessionStore.subscribe((curr, prev) => {
        const scriptChanged = curr.activeScript !== prev.activeScript;
        if (scriptChanged || curr.bytePos !== prev.bytePos) {
            void triggerSimulationAndUpdateState(scriptChanged);
        }
    });
};

/** (runtime-side) initialization with the given arguments, return localized error message on error */
export async function initRuntime(
    runtime: Runtime,
    args: RuntimeWorkerInitArgs,
): Promise<Void<(translator: Translator) => string>> {
    /* The reason why ^ is a function, is because when the error occured,
     * the localization package might not have been loaded. So we delay
     * translating the message to the UI, where react-i18next will handle
     * the display with react updates
     */
    const isCustomImage = args.isCustomImage;
    updateLogo(isCustomImage);
    bootLog.info(`initializing runtime, is_custom_image=${isCustomImage}`);

    const initWorkerResult = await runtime.initialize(args);

    // IPC error
    if (initWorkerResult.err) {
        bootLog.error("runtime initialization failed: IPC error");
        bootLog.error(initWorkerResult.err);
        if (isCustomImage) {
            usePersistStore.getState().setCustomImageVersion("");
        }
        return {
            err: (t) => translateGenericError(initWorkerResult.err.message, t),
        };
    }

    const initResult = initWorkerResult.val;
    if (initResult.err) {
        bootLog.error("runtime initialization failed: runtime error");
        bootLog.error(initResult.err);
        if (isCustomImage) {
            usePersistStore.getState().setCustomImageVersion("");
        }
        return { err: (t) => translateRuntimeInitError(initResult.err, t) };
    }

    const { version, storedVersion } = initResult.val;
    if (storedVersion !== "not-changed") {
        bootLog.info(`updating stored image version to: ${version}`);
        usePersistStore.getState().setCustomImageVersion(version);
    }

    useSessionStore.getState().setRunningCustomImageVersion(version);

    bootLog.info(`runtime initialized successfully, version=${version}`);
    return {};
}

/** Update the logo in the boot screen and the favicon */
const updateLogo = (customImage: boolean) => {
    const image = customImage ? "/static/icon-purple.svg" : "/static/icon.svg";
    const linkIconTag = document.head.querySelector("link[rel='icon']");
    if (!linkIconTag) {
        const link = document.createElement("link");
        link.rel = "icon";
        link.type = "image/svg+xml";
        link.href = image;
        document.head.appendChild(link);
    } else {
        (linkIconTag as HTMLLinkElement).href = image;
    }

    const bootLogo = document.querySelector(".-boot-logo- img");
    if (!bootLogo || (bootLogo as HTMLImageElement).src === image) {
        return;
    }
    (bootLogo as HTMLImageElement).src = image;
};

/** Provide the runtime to the React Tree */
export const RuntimeContext = createContext<Runtime>(undefined as unknown as Runtime);

/** Hook to access the runtime from React */
export const useRuntime = () => {
    return useContext(RuntimeContext);
};
