import { wxWorker, wxWrapHandler } from "@pistonite/workex";
import { serial } from "@pistonite/pure/sync";

import type {
    ItemSearchResult,
    PerformanceData,
    RuntimeApp,
} from "@pistonite/skybook-api";
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
    const triggerSimulation = serial({
        fn: (checkCancel) => async (scriptChanged: boolean) => {
            const {
                invalidateInventoryCache,
                setExecutionInProgress,
                activeScript,
                bytePos,
                setStepIndex,
            } = useSessionStore.getState();
            setExecutionInProgress(true);
            if (scriptChanged) {
                invalidateInventoryCache();
            }
            const stepIndex = await runtime.getStepFromPos(
                activeScript,
                bytePos,
            );
            checkCancel();
            const { activeScript: activeScriptNow } =
                useSessionStore.getState();
            if (activeScriptNow !== activeScript) {
                // script changed while waiting for result
                return;
            }
            setExecutionInProgress(false);
            if (stepIndex.err) {
                console.error("failed to get step index:", stepIndex.err);
                return;
            }
            setStepIndex(stepIndex.val);
        },
    });

    useSessionStore.subscribe((curr, prev) => {
        const scriptChanged = curr.activeScript !== prev.activeScript;
        if (
            !curr.initiallyExecuted ||
            scriptChanged ||
            curr.bytePos !== prev.bytePos
        ) {
            void triggerSimulation(scriptChanged);
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

        updatePerfData: wxWrapHandler((data: PerformanceData) => {
            const { setPerfData } = useSessionStore.getState();
            setPerfData(data.ips, data.sps);
        }),

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
