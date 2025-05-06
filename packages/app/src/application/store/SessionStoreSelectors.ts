import { useEffect } from "react";
import { useDebounce } from "@uidotdev/usehooks";
import type { WxPromise } from "@pistonite/workex";

import type {
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    Runtime,
} from "@pistonite/skybook-api";

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
    data: T | undefined;
    /** If the data is from cache, but not up to date */
    stale: boolean;
    /** If new data is currently being processed in the runtime */
    loading: boolean;
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
    activeScript: string,
    stepIndex: number,
) => {
    return runtime.getPouchList(activeScript, stepIndex);
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
    activeScript: string,
    stepIndex: number,
) => {
    return runtime.getGdtInventory(activeScript, stepIndex);
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
    activeScript: string,
    stepIndex: number,
) => {
    return runtime.getOverworldItems(activeScript, stepIndex);
};

/**
 * Use cached view from the session if possible, otherwise, call the
 * runtime to get the view and store it in the session.
 */
const useStoreCachedRuntimeData = <T>(
    name: string,
    cachedViews: Record<number, T>,
    cacheValidity: number[],
    // must be stable
    runFn: (
        runtime: Runtime,
        activeScript: string,
        stepIndex: number,
    ) => WxPromise<T>,
    setFn: (stepIndex: number, view: T) => void,
): CachedRuntimeData<T> => {
    const activeScript = useDebounce(
        useSessionStore((state) => state.activeScript),
        500,
    );
    const inProgress = useSessionStore((state) => state.executionInProgress);
    const stepIndex = useSessionStore((state) => state.stepIndex);

    const inventory: T | undefined = cachedViews[stepIndex];
    const cacheIsValid = !!(cacheValidity.includes(stepIndex) && inventory);

    const runtime = useRuntime();

    useEffect(() => {
        if (inProgress || cacheIsValid) {
            return;
        }
        let current = true;
        const updateInventory = async () => {
            const view = await runFn(runtime, activeScript, stepIndex);
            const activeScriptNow = useSessionStore.getState().activeScript;
            if (!current || activeScriptNow !== activeScript) {
                return;
            }
            if (view.err) {
                console.error(
                    `useStoreCachedRuntimeData failed for ${name}`,
                    view.err,
                );
                return;
            }
            console.log(
                `useStoreCachedRuntimeData: ${name} updated for step ${stepIndex}`,
            );
            setFn(stepIndex, view.val);
        };

        void updateInventory();

        return () => {
            current = false;
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [inProgress, cacheIsValid, runtime, activeScript, stepIndex, setFn]);

    return {
        data: inventory as T | undefined,
        stale: !cacheIsValid,
        loading: inProgress,
    };
};
