import { create } from "zustand";
import { persist } from "zustand/middleware";

export type UIStore = {
    /** Percentage size of the extension panel */
    extensionPanelPercentage: number;
    setExtensionPanelPercentage: (percentage: number) => void;

    /** Percentage size of the primary extension window */
    primaryExtensionWindowPercentage: number;
    setPrimaryExtensionWindowPercentage: (percentage: number) => void;
};

export const useUIStore = create<UIStore>()(
    persist(
        (set) => ({
            extensionPanelPercentage: 40,
            setExtensionPanelPercentage: (percentage) =>
                set({ extensionPanelPercentage: percentage }),

            primaryExtensionWindowPercentage: 50,
            setPrimaryExtensionWindowPercentage: (percentage) =>
                set({ primaryExtensionWindowPercentage: percentage }),
        }),
        {
            name: "Skybook.UI",
            version: 1,
        },
    ),
);
