import { create } from "zustand";
import { persist } from "zustand/middleware";

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

const DefaultPrimaryIds: string[] = [
    "editor",
    "stub1",
] satisfies (typeof BuiltinExtensionIds)[number][];
const DefaultSecondaryIds: string[] = [
    "item-explorer",
] satisfies (typeof BuiltinExtensionIds)[number][];

export type ExtensionStore = {
    /** Built-in Ids stored locally to track store version */
    builtinIds: string[];
    /** Update the store when built-in extensions are added or removed */
    updateBuiltinExtensions: (newIds: string[]) => void;
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
};

export const useExtensionStore = create<ExtensionStore>()(
    persist(
        (set) => ({
            builtinIds: [...BuiltinExtensionIds],
            updateBuiltinExtensions: (newIds: string[]) => {
                console.log("updating built-in extensions");
                const newPrimary = newIds.filter((id) =>
                    DefaultPrimaryIds.includes(id),
                );
                const newSecondary = newIds.filter((id) =>
                    DefaultSecondaryIds.includes(id),
                );
                const toFiltered = (ids: string[]) => {
                    return ids.filter((id) =>
                        (BuiltinExtensionIds as readonly string[]).includes(id),
                    );
                };
                set(
                    ({
                        primaryIds,
                        secondaryIds,
                        recentIds,
                        pinnedIds,
                        currentPrimary,
                        currentSecondary,
                    }) => ({
                        // typescript issue
                        // eslint-disable-next-line @typescript-eslint/no-explicit-any
                        builtinIds: [...BuiltinExtensionIds] as any,
                        primaryIds: toFiltered([
                            ...new Set(primaryIds.concat(newPrimary)),
                        ]),
                        secondaryIds: toFiltered([
                            ...new Set(secondaryIds.concat(newSecondary)),
                        ]),
                        recentIds: toFiltered(recentIds),
                        pinnedIds: toFiltered(pinnedIds),
                        currentPrimary: (
                            BuiltinExtensionIds as readonly string[]
                        ).includes(currentPrimary)
                            ? currentPrimary
                            : "",
                        currentSecondary: (
                            BuiltinExtensionIds as readonly string[]
                        ).includes(currentSecondary)
                            ? currentSecondary
                            : "",
                    }),
                );
            },
            custom: [],
            pinnedIds: [],
            setPinnedIds: (ids) => {
                set(({ custom }) => {
                    const customIds = custom.map((e) =>
                        getCustomExtensionId(e.url),
                    );
                    return {
                        pinnedIds: filterInvalidCustomIds(ids, customIds),
                    };
                });
            },
            recentIds: [],
            updateRecency: (id: string) => {
                set(({ recentIds, custom }) => {
                    const customIds = custom.map((e) =>
                        getCustomExtensionId(e.url),
                    );
                    return {
                        recentIds: filterInvalidCustomIds(
                            [id, ...recentIds.filter((i) => i !== id)],
                            customIds,
                        ),
                    };
                });
            },

            primaryIds: [...DefaultPrimaryIds],
            secondaryIds: [...DefaultSecondaryIds],
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
                            newState.primaryIds = primaryIds.filter(
                                (i) => i !== id,
                            );
                        }
                        if (!secondaryIds.includes(id)) {
                            newState.secondaryIds = [...secondaryIds, id];
                        }
                    } else {
                        if (primaryIds.includes(id)) {
                            newState.primaryIds = primaryIds.filter(
                                (i) => i !== id,
                            );
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
                    const newCustomIds = extensions.map((e) =>
                        getCustomExtensionId(e.url),
                    );
                    return {
                        custom: extensions,
                        pinnedIds: filterInvalidCustomIds(
                            pinnedIds,
                            newCustomIds,
                        ),
                        recentIds: filterInvalidCustomIds(
                            recentIds,
                            newCustomIds,
                        ),
                    };
                });
            },
        }),
        {
            name: "Skybook.Extensions",
            version: 1,
            onRehydrateStorage: () => {
                return (state) => {
                    if (!state) {
                        return;
                    }
                    const oldSet = new Set(state.builtinIds);
                    const newIds: string[] = [];
                    for (const id of BuiltinExtensionIds) {
                        // new built-in extension added
                        if (!oldSet.has(id)) {
                            newIds.push(id);
                        }
                    }
                    if (newIds.length > 0) {
                        state.updateBuiltinExtensions(newIds);
                        return;
                    }
                    const newSet = new Set<string>(BuiltinExtensionIds);
                    for (const id of state.builtinIds) {
                        // old built-in extension removed
                        if (!newSet.has(id)) {
                            state.updateBuiltinExtensions([]);
                            return;
                        }
                    }
                };
            },
        },
    ),
);

const filterInvalidCustomIds = (
    ids: string[],
    customIds: string[],
): string[] => {
    return ids.filter((id) => {
        return !id.startsWith("custom-") || customIds.includes(id);
    });
};
