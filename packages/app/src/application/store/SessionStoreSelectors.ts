import { useEffect } from "react";
import { useDebounce } from "@uidotdev/usehooks";

import type { InvView_Gdt, InvView_PouchList } from "@pistonite/skybook-api";

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
    const activeScript = useDebounce(
        useSessionStore((state) => state.activeScript),
        500,
    );
    const inProgress = useSessionStore((state) => state.executionInProgress);
    const stepIndex = useSessionStore((state) => state.stepIndex);
    const cachedViews = useSessionStore((state) => state.pouchViews);
    const cacheValidity = useSessionStore((state) => state.upToDatePouchSteps);
    const setPouchViewInCache = useSessionStore(
        (state) => state.setPouchViewInCache,
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
            const view = await runtime.getPouchList(activeScript, stepIndex);
            const activeScriptNow = useSessionStore.getState().activeScript;
            if (!current || activeScriptNow !== activeScript) {
                return;
            }
            if (view.err) {
                console.error("failed to get inventory list view:", view.err);
                return;
            }
            console.log("updating inventory view");
            setPouchViewInCache(stepIndex, view.val);
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
        setPouchViewInCache,
    ]);

    return {
        inventory: inventory as InvView_PouchList | undefined,
        stale: !cacheIsValid,
        loading: inProgress,
    };
};

/** Get the list view of the GDT inventory of the current script and step */
export const useGdtInventoryView = () => {
    const activeScript = useDebounce(
        useSessionStore((state) => state.activeScript),
        500,
    );
    const inProgress = useSessionStore((state) => state.executionInProgress);
    const stepIndex = useSessionStore((state) => state.stepIndex);
    const cachedViews = useSessionStore((state) => state.gdtViews);
    const cacheValidity = useSessionStore((state) => state.upToDateGdtSteps);
    const setGdtViewInCache = useSessionStore(
        (state) => state.setGdtViewInCache,
    );

    const inventory: InvView_Gdt | undefined = cachedViews[stepIndex];
    const cacheIsValid = !!(cacheValidity.includes(stepIndex) && inventory);

    const runtime = useRuntime();

    useEffect(() => {
        if (inProgress || cacheIsValid) {
            return;
        }
        let current = true;
        const updateInventory = async () => {
            const view = await runtime.getGdtInventory(activeScript, stepIndex);
            const activeScriptNow = useSessionStore.getState().activeScript;
            if (!current || activeScriptNow !== activeScript) {
                return;
            }
            if (view.err) {
                console.error("failed to get inventory list view:", view.err);
                return;
            }
            console.log("updating gdt inventory view");
            setGdtViewInCache(stepIndex, view.val);
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
        setGdtViewInCache,
    ]);

    return {
        inventory: inventory as InvView_Gdt | undefined,
        stale: !cacheIsValid,
        loading: inProgress,
    };
};
