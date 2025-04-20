import { create } from "zustand";
import { persist } from "zustand/middleware";

import type {
    GameDataInventory,
    InventoryListView,
} from "@pistonite/skybook-api";

type InventoryViewType = "list" | "tab" | "graph";

export type UIStore = {
    /** Percentage size of the extension panel */
    extensionPanelPercentage: number;
    setExtensionPanelPercentage: (percentage: number) => void;

    /** Percentage size of the primary extension window */
    primaryExtensionWindowPercentage: number;
    setPrimaryExtensionWindowPercentage: (percentage: number) => void;

    /** Inventory in the GameData */
    gdtInventory: GameDataInventory;
    /** Current visible inventory (PMDM) view type */
    inventoryViewType: InventoryViewType;
    /** List view of visible inventory (PMDM) */
    inventoryListView: InventoryListView;
    setInventoryListView: (inventoryListView: InventoryListView) => void;
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

            gdtInventory: {
                items: [],
            },
            inventoryViewType: "list",
            inventoryListView: {
                info: {
                    numTabs: 0,
                    tabs: [],
                },
                items: [],
            },
            setInventoryListView: (inventoryListView) => {
                set({ inventoryListView });
            },
        }),
        {
            name: "Skybook.UI",
            version: 1,
        },
    ),
);
