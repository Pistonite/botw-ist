import { create } from "zustand";
import { createSelector } from "reselect";

import { isLessProductive, useNarrow } from "self::pure-contrib";

/**
 * List of builtin extension IDs
 *
 * The builtin extensions will be displayed in this order
 * in dropdowns. Custom extensions will be sorted by id
 */
const BuiltinExtensionIds = [
    "editor",
    "item-explorer",
    "stub1",
    "stub2",
] as const;

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

    custom: CustomExtension[];
    setCustomExtension: (extension: CustomExtension) => void;
    removeCustomExtension: (id: string) => void;
};

export type ExtensionOpenMode = "primary" | "secondary" | "popout";

export type CustomExtension = {
    id: string;
    name: string;
    url: string;
};

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

    custom: [],
    setCustomExtension: (extension) => {
        set(({ custom }) => {
            return {
                custom: custom.map((c) =>
                    c.id === extension.id ? extension : c,
                ),
            };
        });
    },
    removeCustomExtension: (id) => {
        set(({ custom }) => {
            return { custom: custom.filter((c) => c.id !== id) };
        });
    },
}));
// ,{
//         name: "Skybook.Extensions",
//         version: 1,
//     }));
//

export const useAllExtensionIds = () => {
    return BuiltinExtensionIds;
};

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

const getPrimaryExtensionIds = createSelector(
    [(state: ExtensionStore) => state.primaryIds],
    (p) => {
        return toSortedExtensionIds(p);
    },
);
export const usePrimaryExtensionIds = () => {
    return useExtensionStore(getPrimaryExtensionIds);
};

const getSecondaryExtensionIds = createSelector(
    [(state: ExtensionStore) => state.secondaryIds],
    (s) => {
        return toSortedExtensionIds(s);
    },
);
export const useSecondaryExtensionIds = () => {
    return useExtensionStore(getSecondaryExtensionIds);
};

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

const getCustomExtensions = createSelector(
    [(state: ExtensionStore) => state.custom],
    (custom) => {
        const map = new Map<string, CustomExtension>();
        custom.forEach((c) => map.set(c.id, c));
        return map;
    },
);

export const useCustomExtensions = () => {
    return useExtensionStore(getCustomExtensions);
};
