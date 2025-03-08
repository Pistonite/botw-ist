import { create } from "zustand";
import { persist } from "zustand/middleware";

/** Persistent state for the application */
export type ApplicationStore = {
    /** 
     * The SAVED simulator script
     *
     * Saving script into the application store is unchecked. The app
     * should always use setActiveScript from the session, which
     * throttles the script change and checks the mode of the session
     */
    savedScript: string;
    setSavedScript: (script: string) => void;

    /** 
     * The previously stored custom image version. Empty if no custom image 
     */
    customImageVersion: string;
    setCustomImageVersion: (version: string) => void;
    /**
     * If custom image should be used by default on sessions with editing
     * enabled (from local script or direct load with the edit flag set)
     */
    isUseCustomImageByDefault: boolean;
    setUseCustomImageByDefault: (value: boolean) => void;
};

export const useApplicationStore = create<ApplicationStore>()(
    persist(
        (set) => {
            return {
                savedScript: "",
                setSavedScript: (savedScript) => set({ savedScript}),
                customImageVersion: "",
                setCustomImageVersion: (version) => set({ customImageVersion: version }),
                isUseCustomImageByDefault: false as boolean,
                setUseCustomImageByDefault: (value) => set({ isUseCustomImageByDefault: value }),
            }
        },
        {
            name: "Skybook.Application",
            version: 2,
        },
    ),
);

