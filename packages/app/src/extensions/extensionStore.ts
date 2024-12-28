import { create } from "zustand";
import { persist } from "zustand/middleware";

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
}

export const useExtensionStore = create<ExtensionStore>()(persist((_set) => ({
    primaryIds: ["editor"],
    secondaryIds: [],
    currentPrimary: "editor",
    currentSecondary: "",
}),{
        name: "Skybook.Extensions",
        version: 1,
    }));

export const setPrimary = (id: string) => useExtensionStore.setState({ currentPrimary: id });
