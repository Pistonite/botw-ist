import { wxWorker, wxWrapHandler } from "@pistonite/workex";
import { serial } from "@pistonite/pure/sync";
import { v4 as makeUUID } from "uuid";

import type { ItemSearchResult, RuntimeApp } from "@pistonite/skybook-api";
import { skybookRuntime } from "@pistonite/skybook-api/interfaces/Runtime.bus";
import { searchItemLocalized } from "skybook-localization";

import { crashApp, useSessionStore } from "self::application/store";

let customImage: Uint8Array | undefined;

/**
 * Create the runtime worker, but do not initialize it yet
 */
export async function createRuntime() {
    const appHost = createRuntimeAppHost();
    // create the runtime worker
    let url: string;
    if (import.meta.env.DEV) {
        console.log("[dev] using local runtime worker");
        url = "/runtime/worker.js";
    } else {
        const commitShort = import.meta.env.COMMIT.substring(0, 8);
        url = `/runtime/worker-${commitShort}.min.js`;
    }
    const worker = new Worker(url);
    const result = await wxWorker(worker)({
        runtime: skybookRuntime(appHost),
    });

    if (result.err) {
        console.error("[boot] failed to connect to runtime worker", result.err);
        throw new Error(
            "fatal boot failure: failed to connect to runtime worker",
        );
    }

    const runtime = result.val.protocols.runtime;

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
                // setInitiallyExecuted,
            } = useSessionStore.getState();
            // setInitiallyExecuted();
            console.log("triggering background execution");

            if (scriptChanged) {
                invalidateInventoryCache();
            }
            const checkCancel = () => {
                checkTaskCancel();
                const { activeScript: activeScriptNow } =
                    useSessionStore.getState();
                if (activeScriptNow !== activeScript) {
                    // script changed while waiting for result
                    throw new Error("cancelled");
                }
            };
            // this only requires parsing, and should be fast
            const stepIndex = await runtime.getStepFromPos(
                activeScript,
                bytePos,
            );
            checkCancel();
            if (stepIndex.err) {
                console.error("failed to get step index:", stepIndex.err);
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
            console.log(taskId);
            setInProgressTaskId(taskId);

            await runtime.executeScript(activeScript, taskId);
            if (useSessionStore.getState().inProgressTaskId === taskId) {
                setInProgressTaskId("");
            }
        },
    });

    // register a subscriber to reliably trigger the script execution
    // for the first time, then unregisters itself
    const unsubcribeInitialExecution = useSessionStore.subscribe((curr) => {
        if (curr.activeScript) {
            console.log("triggering script execution for the first time");
            unsubcribeInitialExecution();
            setTimeout(() => void triggerSimulationAndUpdateState(true), 0);
        }
    });

    useSessionStore.subscribe((curr, prev) => {
        const scriptChanged = curr.activeScript !== prev.activeScript;
        // console.log(curr.initiallyExecuted);
        if (
            // !curr.initiallyExecuted ||
            scriptChanged ||
            curr.bytePos !== prev.bytePos
        ) {
            void triggerSimulationAndUpdateState(scriptChanged);
        }
    });

    return runtime;
}

/** Create the API for runtime to call the app */
const createRuntimeAppHost = (): RuntimeApp => {
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

        // updatePerfData: wxWrapHandler((data: PerformanceData) => {
        //     const { setPerfData } = useSessionStore.getState();
        //     setPerfData(data.ips, data.sps);
        // }),

        crashApplication: wxWrapHandler(() => {
            crashApp();
        }),
    };
};

/**
 * Set the custom image to provide to the runtime if the runtime asks for it
 */
export const setCustomImageToProvide = (image: Uint8Array | undefined) => {
    customImage = image;
};
