import { create } from "zustand";
import { createSelector } from "reselect";
import { isLessProductive } from "pure-contrib/platform";
import { useNarrow } from "pure-contrib/narrow";

export const ExtensionIds = ["editor", "stub1", "stub2"] as const;
// Use the sort index to display the extensions in a deterministic order
const ExtensionIdToSortIndex: Record<string, number> = (() => {
    const result: Record<string, number> = {};
    ExtensionIds.forEach((id, index) => {
        result[id] = index;
    });
    return result;
})();

export type ExtensionStore = {
    /**
     * Ids of the extensions in the primary slot
     */
    primaryIds: string[];

    /**
     * Ids of the extensions in the secondary slot
     */
    secondaryIds: string[];

    currentPrimary: string;

    currentSecondary: string;

    /**
     * Set the open preference for the extension with the id
     */
    // setExtensionPreference(id: string, preference: "primary" | "secondary" | "none"): void;

    /**
     * Open an extension in the primary or secondary slot, optionally
     * update the open mode for the extension for the future
     */
    open: (
        id: string,
        slot: "primary" | "secondary",
        updateOpenMode?: boolean,
    ) => void;

    /**
     * Update the open mode for the extension with the id
     */
    updateOpenMode: (id: string, openMode: ExtensionOpenMode) => void;

    /**
     * Close the primary extension window
     */
    closePrimary: () => void;

    /**
     * Close the secondary extension window
     */
    closeSecondary: () => void;
};

export type ExtensionOpenMode = "primary" | "secondary" | "popout";

export const useExtensionStore = create<ExtensionStore>()((set) => ({
    primaryIds: ["editor", "stub1"],
    secondaryIds: ["item-explorer"],
    currentPrimary: "editor",
    currentSecondary: "item-explorer",

    open: (
        id: string,
        slot: "primary" | "secondary",
        updateOpenMode = false,
    ) => {
        set(({ primaryIds, secondaryIds }) => {
            const newState: Partial<ExtensionStore> = {};
            if (slot === "primary") {
                newState.currentPrimary = id;
                if (updateOpenMode) {
                    if (secondaryIds.includes(id)) {
                        newState.secondaryIds = secondaryIds.filter(
                            (i) => i !== id,
                        );
                    }
                    if (!primaryIds.includes(id)) {
                        newState.primaryIds = [...primaryIds, id];
                    }
                }
            } else {
                newState.currentSecondary = id;
                if (updateOpenMode) {
                    if (primaryIds.includes(id)) {
                        newState.primaryIds = primaryIds.filter(
                            (i) => i !== id,
                        );
                    }
                    if (!secondaryIds.includes(id)) {
                        newState.secondaryIds = [...secondaryIds, id];
                    }
                }
            }
            return newState;
        });
    },

    updateOpenMode: (id: string, openMode: ExtensionOpenMode) => {
        set(({ primaryIds, secondaryIds }) => {
            const newState: Partial<ExtensionStore> = {};
            if (openMode === "primary") {
                if (secondaryIds.includes(id)) {
                    newState.secondaryIds = secondaryIds.filter(
                        (i) => i !== id,
                    );
                }
                if (!primaryIds.includes(id)) {
                    newState.primaryIds = [...primaryIds, id];
                }
            } else if (openMode === "secondary") {
                newState.currentSecondary = id;
                if (primaryIds.includes(id)) {
                    newState.primaryIds = primaryIds.filter((i) => i !== id);
                }
                if (!secondaryIds.includes(id)) {
                    newState.secondaryIds = [...secondaryIds, id];
                }
            } else {
                if (primaryIds.includes(id)) {
                    newState.primaryIds = primaryIds.filter((i) => i !== id);
                }
                if (secondaryIds.includes(id)) {
                    newState.secondaryIds = secondaryIds.filter(
                        (i) => i !== id,
                    );
                }
            }
            return newState;
        });
    },

    closePrimary: () => {
        set({ currentPrimary: "" });
    },

    closeSecondary: () => {
        set({ currentSecondary: "" });
    },
}));
// ,{
//         name: "Skybook.Extensions",
//         version: 1,
//     }));
//

export const useAllExtensionIds = () => {
    return ExtensionIds;
};

const getAllNonPopoutExtensionIds = createSelector(
    [
        (state: ExtensionStore) => state.primaryIds,
        (state: ExtensionStore) => state.secondaryIds,
    ],
    (p, s) => {
        return sortExtensionIds([...p, ...s]);
    },
);
export const useAllNonPopoutExtensionIds = () => {
    return useExtensionStore(getAllNonPopoutExtensionIds);
};

const getPrimaryExtensionIds = createSelector(
    [(state: ExtensionStore) => state.primaryIds],
    (p) => {
        return sortExtensionIds([...p]);
    },
);
export const usePrimaryExtensionIds = () => {
    return useExtensionStore(getPrimaryExtensionIds);
};

const getSecondaryExtensionIds = createSelector(
    [(state: ExtensionStore) => state.secondaryIds],
    (s) => {
        return sortExtensionIds([...s]);
    },
);
export const useSecondaryExtensionIds = () => {
    return useExtensionStore(getSecondaryExtensionIds);
};

const sortExtensionIds = (ids: string[]): string[] => {
    // toSorted
    return ids.sort((a, b) => {
        const aIndex =
            a in ExtensionIdToSortIndex ? ExtensionIdToSortIndex[a] : 9999;
        const bIndex =
            b in ExtensionIdToSortIndex ? ExtensionIdToSortIndex[b] : 9999;
        return aIndex - bIndex;
    });
};

export const useCurrentPrimaryExtensionId = () => {
    return useExtensionStore((state) => state.currentPrimary);
};

export const useCurrentSecondaryExtensionId = () => {
    const narrow = useNarrow();
    const secondary = useExtensionStore((state) => state.currentSecondary);
    // hide secondary extension window when the screen is narrow
    // or on less productive platforms
    return narrow || isLessProductive ? "" : secondary;
};

export const useIsShowingExtensionPanel = () => {
    const primary = useExtensionStore((state) => state.currentPrimary);
    const secondary = useCurrentSecondaryExtensionId();
    return !!(primary || secondary);
};
