import { create } from "zustand";
import { persist } from "zustand/middleware";

export type DialogId = "extension-launch" | "custom-extension";

export type UIStore = {
    /** Percentage size of the extension panel */
    extensionPanelPercentage: number;
    setExtensionPanelPercentage: (percentage: number) => void;

    /** Percentage size of the primary extension window */
    primaryExtensionWindowPercentage: number;
    setPrimaryExtensionWindowPercentage: (percentage: number) => void;

    /** Percentage size of the gamedata inventory panel */
    gamedataInventoryPercentage: number;
    setGamedataInventoryPercentage: (percentage: number) => void;

    /** Id of the currently open dialog */
    openedDialogId: DialogId | undefined;
    setOpenedDialog: (id: DialogId | undefined) => void;
};

/**
 * Properties that are not persisted
 *
 * Technically, these should be in SessionStore as they are not persisted.
 * However, these are mostly purely-UI-related properties so it makes
 * more sense for them to be in the UIStore
 */
const ExcludedKeys: (keyof UIStore)[] = ["openedDialogId"];

export const useUIStore = create<UIStore>()(
    persist(
        (set) => ({
            extensionPanelPercentage: 40,
            setExtensionPanelPercentage: (percentage) => {
                set({ extensionPanelPercentage: percentage });
            },

            primaryExtensionWindowPercentage: 50,
            setPrimaryExtensionWindowPercentage: (percentage) => {
                set({ primaryExtensionWindowPercentage: percentage });
            },

            gamedataInventoryPercentage: 40,
            setGamedataInventoryPercentage: (percentage) => {
                set({ gamedataInventoryPercentage: percentage });
            },

            openedDialogId: undefined,
            setOpenedDialog: (id) => {
                set({ openedDialogId: id });
            },
        }),
        {
            name: "Skybook.UI",
            version: 1,
            partialize: (state) => {
                return Object.fromEntries(
                    Object.entries(state).filter(([key]) => {
                        return !ExcludedKeys.includes(key as keyof UIStore);
                    }),
                );
            },
        },
    ),
);
