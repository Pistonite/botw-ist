import { create } from "zustand";
import { persist } from "zustand/middleware";
import { openExtensionPopup } from "./extensionManager";

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
     * Set the extension with the id as the current primary extension 
     *
     * If the extension is not currently in primaryIds, it will be added,
     * and removed from other slots
     */
    setPrimary: (id: string) => void;

    /**
     * Set the extension with the id as the current secondary extension
     *
     * If the extension is not currently in secondaryIds, it will be added,
     * and removed from other slots
     */
    setSecondary: (id: string) => void;

    /** 
     * Close the primary extension window
     */
    closePrimary: () => void;

    /**
     * Close the secondary extension window
     */
    closeSecondary: () => void;
}

export type ExtensionOpenMode = "primary" | "secondary" | "popout";

export const useExtensionStore = create<ExtensionStore>()((set) => ({
    primaryIds: ["editor", "stub2"],
    secondaryIds: ["stub1"],
    currentPrimary: "editor",
    currentSecondary: "stub1",

    setPrimary: (id: string) => {
        set(({primaryIds, secondaryIds}) => {
            const newState: Partial<ExtensionStore> = {
                currentPrimary: id,
            };
            if (secondaryIds.includes(id)) {
                newState.secondaryIds = secondaryIds.filter((i) => i !== id);
            }
            if (!primaryIds.includes(id)) {
                newState.primaryIds = [...primaryIds, id];
            }
            return newState;
        });
    },

    setSecondary: (id: string) => {
        set(({primaryIds, secondaryIds}) => {
            const newState: Partial<ExtensionStore> = {
                currentSecondary: id,
            };
            if (primaryIds.includes(id)) {
                newState.primaryIds = primaryIds.filter((i) => i !== id);
            }
            if (!secondaryIds.includes(id)) {
                newState.secondaryIds = [...secondaryIds, id];
            }
            return newState;
        });
    },

    closePrimary: () => {
        set({currentPrimary: ""});
    },

    closeSecondary: () => {
        set({currentSecondary: ""});
    }

}));
// ,{
//         name: "Skybook.Extensions",
//         version: 1,
//     }));
//

export const useIsShowingExtensionPanel = () => {
    const x=  useExtensionStore((state) => state.currentPrimary || state.currentSecondary);
    return !!x;
}
