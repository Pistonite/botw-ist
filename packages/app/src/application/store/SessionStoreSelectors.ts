import { useEffect, useRef, useState } from "react";
import { useDebounce } from "@uidotdev/usehooks";
import type { WxPromise } from "@pistonite/workex";
import type { Result } from "@pistonite/pure/result";
import { v4 as makeUUID } from "uuid";

import type {
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    MaybeAborted,
    Runtime,
    RuntimeViewError,
} from "@pistonite/skybook-api";
import { translateGenericError } from "skybook-localization";

import { useRuntime } from "self::application/runtime";

import { useSessionStore } from "./SessionStore.ts";

/**
 * Get the debounced value of hasUnsavedChanges of the session
 */
export const useDebouncedHasUnsavedChanges = (delay: number) => {
    const hasUnsavedChanges = useSessionStore(
        (state) => state.hasUnsavedChanges,
    );
    return useDebounce(hasUnsavedChanges, delay);
};

export type CachedRuntimeData<T> = {
    data: Result<T, RuntimeViewError> | undefined;
    /** If new data is currently being processed in the runtime */
    loading: boolean;
    /** Error in runtime. Empty strings means no error */
    error: string;
};

/** Get the list view of the visible inventory of the current script and step */
export const usePouchListView = (): CachedRuntimeData<InvView_PouchList> => {
    const cachedViews = useSessionStore((state) => state.pouchViews);
    const cacheValidity = useSessionStore((state) => state.pouchCached);
    const setPouchViewInCache = useSessionStore(
        (state) => state.setPouchViewInCache,
    );
    return useStoreCachedRuntimeData(
        "PouchList",
        cachedViews,
        cacheValidity,
        getPouchListShim,
        setPouchViewInCache,
    );
};
const getPouchListShim = (
    runtime: Runtime,
    taskId: string,
    activeScript: string,
    bytePos: number,
) => {
    return runtime.getPouchList(activeScript, taskId, bytePos);
};

/** Get the list view of the GDT inventory of the current script and step */
export const useGdtInventoryView = (): CachedRuntimeData<InvView_Gdt> => {
    const cachedViews = useSessionStore((state) => state.gdtViews);
    const cacheValidity = useSessionStore((state) => state.gdtCached);
    const setGdtViewInCache = useSessionStore(
        (state) => state.setGdtViewInCache,
    );
    return useStoreCachedRuntimeData(
        "GdtInventory",
        cachedViews,
        cacheValidity,
        getGdtInventoryShim,
        setGdtViewInCache,
    );
};
const getGdtInventoryShim = (
    runtime: Runtime,
    taskId: string,
    activeScript: string,
    bytePos: number,
) => {
    return runtime.getGdtInventory(activeScript, taskId, bytePos);
};

/** Get the view of the overworld items of the current script and step */
export const useOverworldItemsView =
    (): CachedRuntimeData<InvView_Overworld> => {
        const cachedViews = useSessionStore((state) => state.overworldViews);
        const cacheValidity = useSessionStore((state) => state.overworldCached);
        const setOverworldViewInCache = useSessionStore(
            (state) => state.setOverworldViewInCache,
        );
        return useStoreCachedRuntimeData(
            "OverworldItems",
            cachedViews,
            cacheValidity,
            getOverworldItemsShim,
            setOverworldViewInCache,
        );
    };
const getOverworldItemsShim = (
    runtime: Runtime,
    taskId: string,
    activeScript: string,
    bytePos: number,
) => {
    return runtime.getOverworldItems(activeScript, taskId, bytePos);
};

/**
 * Use cached view from the session if possible, otherwise, call the
 * runtime to get the view and store it in the session.
 */
const useStoreCachedRuntimeData = <T>(
    name: string,
    cachedViews: Record<number, Result<T, RuntimeViewError>>,
    cacheValidity: number[],
    // must be stable
    runFn: (
        runtime: Runtime,
        taskId: string,
        activeScript: string,
        bytePos: number,
    ) => WxPromise<MaybeAborted<Result<T, RuntimeViewError>>>,
    setFn: (stepIndex: number, view: Result<T, RuntimeViewError>) => void,
): CachedRuntimeData<T> => {
    const activeScript = useDebounce(
        useSessionStore((state) => state.activeScript),
        100,
    );
    const inProgress = useSessionStore((state) => state.executionInProgress);
    const stepIndex = useSessionStore((state) => state.stepIndex);
    const bytePos = useSessionStore((state) => state.bytePos);

    const cacheIsValid = !!(cacheValidity.includes(stepIndex) && cachedViews[stepIndex]);
    // only use the inventory from state if the cache is valid
    const inventory = cacheIsValid ? cachedViews[stepIndex] : undefined;

    const runtime = useRuntime();

    const [errorMessage, setErrorMessage] = useState("");

    useEffect(() => {
        if (cacheIsValid) {
            return;
        }
        let taskId: string | undefined = undefined;
        const isCurrent = () => {
            const activeScriptNow = useSessionStore.getState().activeScript;
            return activeScriptNow === activeScript;
        };
        const updateInventory = async () => {
            const stepIndex = await runtime.getStepFromPos(
                activeScript,
                bytePos,
            );
            if (stepIndex.err) {
                console.error(
                    `[rtux] ${name} failed. cannot get step index.`,
                    stepIndex.err,
                );
                setErrorMessage(translateGenericError(stepIndex.err.message));
                return;
            }
            if (!isCurrent()) {
                return;
            }
            // we only need task id once we request the run
            taskId = makeUUID();
            console.log(
                `[rtux] starting task ${taskId} for ${name}, bytepos=${bytePos}`,
            );
            const view = await runFn(runtime, taskId, activeScript, bytePos);
            // IPC error
            if (view.err) {
                console.error(
                    `[rtux] task ${taskId} for ${name} failed. IPC error.`,
                    view.err,
                );
                setErrorMessage(translateGenericError(view.err.message));
                return;
            }
            if (view.val.type === "Aborted") {
                console.warn(`[rtux] task ${taskId} for ${name} aborted`);
                return;
            }
            if (!isCurrent()) {
                return;
            }
            // TODO: when getting certain view, the runtime may return error
            // if cannot get that view
            console.log(`[rtux] task ${taskId} for ${name} succeeded`);
            const viewVal = view.val.value;
            setFn(stepIndex.val, viewVal);
        };

        void updateInventory();

        return () => {
            // we only want to abort the task if the script
            // has changed. Otherwise, if we are the only one running,
            // we are aborting the old task and starting over, which is not helpful
            if (taskId && !isCurrent()) {
                void runtime.abortTask(taskId);
            }
        };
        // note we don't trigger when stepIndex updates, because
        // it is computed asynchronously from bytePos
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [cacheIsValid, runtime, activeScript, bytePos, setFn]);

    // Create a per-component cache. If the view is not ready when
    // creating new steps, then we can still render the old result,
    // instead of showing empty state (which causes large UI flickering)
    const inventoryViewCache = useRef<Result<T, RuntimeViewError> | undefined>(
        undefined,
    );
    useEffect(() => {
        if (inventory) {
            inventoryViewCache.current = inventory;
        }
    }, [inventory]);

    return {
        // if state for current step is not ready,
        // display the per-component cache to avoid flickering
        data: inventory || inventoryViewCache.current,
        loading: !cacheIsValid || inProgress,
        error: errorMessage,
    };
};
