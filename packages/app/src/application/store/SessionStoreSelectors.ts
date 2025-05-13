import { useEffect, useRef } from "react";
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
    bytePos: number,
) => {
    return runtime.getPouchList(activeScript, bytePos);
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
    bytePos: number,
) => {
    return runtime.getGdtInventory(activeScript, bytePos);
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
    bytePos: number,
) => {
    return runtime.getOverworldItems(activeScript, bytePos);
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
        bytePos: number,
    ) => WxPromise<T>,
    setFn: (stepIndex: number, view: T) => void,
): CachedRuntimeData<T> => {
    const activeScript = useDebounce(
        useSessionStore((state) => state.activeScript),
        500,
    );
    const inProgress = useSessionStore((state) => state.executionInProgress);
    const stepIndex = useSessionStore((state) => state.stepIndex);
    const bytePos = useSessionStore((state) => state.bytePos);

    const inventory: T | undefined = cachedViews[stepIndex];
    const cacheIsValid = !!(cacheValidity.includes(stepIndex) && inventory);

    const runtime = useRuntime();

    useEffect(() => {
        if (cacheIsValid) {
            return;
        }
        // Note that this hook is executed per-use, meaning if multiple
        // components need to access the pouch view, they are all going
        // to fire the request at the runtime.
        //
        // This introduces extra load on the runtime thread that may
        // slightly hurt the runtime performance. However, they don't
        // result in new simulation runs, since as long as the script
        // stays the same, the runtime will batch the request and cache
        // the results
        //
        // When the first request is done, all the components will rerender,
        // and they will see the cache is now valid, and cancel the previous
        // request to not update the store again.
        //
        // If we want to optimize this, we can store in SessionStore
        // if the run request is made per-inventory (pouch, gdt, overworld) and
        // per-step, and prevent re-firing the same request at the runtime.
        // I don't think this is worth the effort right now.
        //
        // Note that we are not taking the approach where only one component
        // fires the request, or to execute the script in a centralized manner.
        // This is so that we only execute when needed. Although that certainly
        // means "always" right now (because the component is always visible),
        // executing the script is the most expensive operation in the app,
        // and we want to avoid it as much as possible.
        let current = true;
        const isCurrent = () => {
            if (!current) {
                return false;
            }
            const activeScriptNow = useSessionStore.getState().activeScript;
            return activeScriptNow === activeScript;
        };
        const updateInventory = async () => {
            const stepIndex = await runtime.getStepFromPos(
                activeScript,
                bytePos,
            );
            if (!isCurrent() || stepIndex.err) {
                return;
            }
            const view = await runFn(runtime, activeScript, bytePos);
            if (!isCurrent()) {
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
                `useStoreCachedRuntimeData: ${name} updated for bytePos(${bytePos}) step ${stepIndex.val}`,
            );
            setFn(stepIndex.val, view.val);
        };

        void updateInventory();

        return () => {
            current = false;
        };
        // note we don't trigger when stepIndex updates, because
        // it is computed asynchronously from bytePos
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [cacheIsValid, runtime, activeScript, bytePos, setFn]);

    // Create a per-component cache. If the view is not ready when
    // creating new steps, then we can still render the old result,
    // instead of showing empty state (which causes large UI flickering)
    const inventoryViewCache = useRef<T | undefined>(undefined);
    useEffect(() => {
        if (inventory) {
            inventoryViewCache.current = inventory;
        }
    }, [inventory]);

    return {
        // if state for current step is not ready,
        // display the previous step to avoid flickering
        data: (inventory || inventoryViewCache.current) as T | undefined,
        stale: !cacheIsValid,
        loading: inProgress,
    };
};
