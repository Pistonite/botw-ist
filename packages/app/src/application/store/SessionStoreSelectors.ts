import { useEffect } from "react";
import { useDebounce } from "@uidotdev/usehooks";

import type { InvView_PouchList } from "@pistonite/skybook-api";

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

/** Get the list view of the visible inventory of the current script and step */
export const usePouchListView = () => {
    const activeScript = useSessionStore((state) => state.activeScript);
    const inProgress = useSessionStore((state) => state.executionInProgress);
    const stepIndex = useSessionStore((state) => state.stepIndex);
    const cachedViews = useSessionStore((state) => state.inventoryListViews);
    const cacheValidity = useSessionStore((state) => state.upToDateSteps);
    const setInventoryListViewInCache = useSessionStore(
        (state) => state.setInventoryListViewInCache,
    );

    const inventory: InvView_PouchList | undefined = cachedViews[stepIndex];
    const cacheIsValid = !!(cacheValidity.includes(stepIndex) && inventory);

    const runtime = useRuntime();

    useEffect(() => {
        if (inProgress || cacheIsValid) {
            return;
        }
        let current = true;
        const updateInventory = async () => {
            const view = await runtime.getPouchList(
                activeScript,
                stepIndex,
            );
            const activeScriptNow = useSessionStore.getState().activeScript;
            if (!current || activeScriptNow !== activeScript) {
                return;
            }
            if (view.err) {
                console.error("failed to get inventory list view:", view.err);
                return;
            }
            setInventoryListViewInCache(stepIndex, view.val);
        };

        void updateInventory();

        return () => {
            current = false;
        };
    }, [
        inProgress,
        cacheIsValid,
        runtime,
        activeScript,
        stepIndex,
        setInventoryListViewInCache,
    ]);

    return {
        inventory: inventory as InvView_PouchList | undefined,
        stale: !cacheIsValid,
        loading: inProgress,
    };
};
