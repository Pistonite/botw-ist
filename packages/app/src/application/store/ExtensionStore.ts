import { create } from "zustand";
import { createSelector } from "reselect";

import { isLessProductive, useNarrow } from "self::pure-contrib";

/**
 * List of builtin extension IDs
 *
 * The builtin extensions will be displayed in this order
 * in dropdowns. Custom extensions will be displayed in the order
 * they are configured in.
 *
 * Custom extension IDs will be in the format of `custom-<url>`.
 */
export const BuiltinExtensionIds = [
    "editor",
    "item-explorer",
    "stub1",
] as const;

export type ExtensionStore = {
    /** 
     * Configured custom extensions
     * (note that custom extensions can only be popups)
     */
    custom: CustomExtension[];

    /** Pinned extensions (could include custom) */
    pinnedIds: string[];
    setPinnedIds: (ids: string[]) => void;

    /** Recently opened extensions (could include custom) */
    recentIds: string[];
    /** Update id to be the most recently used extension */
    updateRecency: (id: string) => void;

    /** Ids of the (built-in) extensions configured to open in the primary slot */
    primaryIds: string[];
    /** Same as primaryIds, but for the secondary slot */
    secondaryIds: string[];
    /** Current (built-in) extension id in the primary slot */
    currentPrimary: string;
    /** Current (built-in) extension id in the secondary slot */
    currentSecondary: string;

    /**
     * Open an extension in the primary or secondary slot, optionally
     * update the open mode for the extension for the future
     */
    open: (
        id: string,
        slot: "primary" | "secondary",
        updateOpenMode?: boolean,
    ) => void;

    /** Update the open mode for the extension with the id */
    updateOpenMode: (id: string, openMode: ExtensionOpenMode) => void;
    /** Close the primary extension window */
    closePrimary: () => void;
    /** Close the secondary extension window */
    closeSecondary: () => void;
    /** Set custom extension configuration */
    setCustomExtensions: (extensions: CustomExtension[]) => void;
};

export type ExtensionOpenMode = "primary" | "secondary" | "popout";

export type CustomExtension = {
    name: string;
    url: string;
};

export const getCustomExtensionId = (url: string) => {
    return `custom-${url}`;
}

export const useExtensionStore = create<ExtensionStore>()((set) => ({
    custom: [{
        name: "Test",
        url: "https://skybook.pistonite.dev",
    }],
    pinnedIds: [],
    setPinnedIds: (ids) => {
        set(({ custom }) => {
            const customIds = custom.map((e) => getCustomExtensionId(e.url));
            return {
                pinnedIds: filterInvalidCustomIds(ids, customIds),
            }
        });
    },
    recentIds: [],
    updateRecency: (id: string) => {
        set(({ recentIds, custom }) => {
            const customIds = custom.map((e) => getCustomExtensionId(e.url));
            return {
                recentIds: filterInvalidCustomIds([id, ...recentIds.filter((i) => i !== id)], customIds),
            }
        });
    },


    primaryIds: ["editor", "stub1"],
    secondaryIds: ["item-explorer"],
    currentPrimary: "editor",
    currentSecondary: "item-explorer",

    open: (
        id: string,
        slot: "primary" | "secondary",
        updateOpenMode = false,
    ) => {
        // make editor only openable in the primary slot
        // because issue with the monaco instance
        if (id === "editor") {
            slot = "primary";
        }
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
        if (id === "editor") {
            openMode = "primary";
        }
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

    setCustomExtensions: (extensions) => {
        // here we need clean the state and delete old ids that aren't 
        // in the new list
        set(({ pinnedIds, recentIds }) => {
            const newCustomIds = extensions.map((e) => getCustomExtensionId(e.url));
            return {
                custom: extensions,
                pinnedIds: filterInvalidCustomIds(pinnedIds, newCustomIds),
                recentIds: filterInvalidCustomIds(recentIds, newCustomIds)
            }
        });
    },
}));
// ,{
//         name: "Skybook.Extensions",
//         version: 1,
//     }));
//
//
const filterInvalidCustomIds = (ids: string[], customIds: string[]): string[] => {
    return ids.filter((id) => {
        return !id.startsWith("custom-") || customIds.includes(id);
    });
}

const getAllNonPopoutExtensionIds = createSelector(
    [
        (state: ExtensionStore) => state.primaryIds,
        (state: ExtensionStore) => state.secondaryIds,
    ],
    (p, s) => {
        return toSortedExtensionIds([...p, ...s]);
    },
);
export const useAllNonPopoutExtensionIds = () => {
    return useExtensionStore(getAllNonPopoutExtensionIds);
};

// const getPrimaryExtensionIds = createSelector(
//     [(state: ExtensionStore) => state.primaryIds],
//     (p) => {
//         return toSortedExtensionIds(p);
//     },
// );
// export const usePrimaryExtensionIds = () => {
//     return useExtensionStore(getPrimaryExtensionIds);
// };
//
// const getSecondaryExtensionIds = createSelector(
//     [(state: ExtensionStore) => state.secondaryIds],
//     (s) => {
//         return toSortedExtensionIds(s);
//     },
// );
// export const useSecondaryExtensionIds = () => {
//     return useExtensionStore(getSecondaryExtensionIds);
// };

const toSortedExtensionIds = (ids: string[]): string[] => {
    const customs = ids.filter(
        (id) => !(BuiltinExtensionIds as readonly string[]).includes(id),
    );
    customs.sort();
    const builtins = BuiltinExtensionIds.filter((id) => ids.includes(id));
    return [...builtins, ...customs];
};

export const useCurrentPrimaryExtensionId = () => {
    return useExtensionStore((state) => state.currentPrimary);
};



