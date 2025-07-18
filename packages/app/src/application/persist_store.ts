import { create } from "zustand";
import { persist } from "zustand/middleware";

import {
    parseEnvFromScript,
    type ScriptEnvImage,
} from "@pistonite/skybook-api";
import type { ActorSpriteProps } from "botw-item-assets";

import { STARTER_SCRIPT } from "self::util";

/** Persistent state for the application, like settings and locally saved script */
export type PersistStore = {
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
     * The previously stored custom image version ("1.5" or "1.6"). Empty if no custom image
     */
    customImageVersion: ScriptEnvImage | "";
    setCustomImageVersion: (version: ScriptEnvImage | "") => void;
    /**
     * If custom image should be used by default on sessions with editing
     * enabled (from local script or direct load with the edit flag set)
     */
    isUseCustomImageByDefault: boolean;
    setUseCustomImageByDefault: (value: boolean) => void;

    /** Enable high-res icons. Disabling this will also disable animation */
    enableHighQualityIcons: boolean;
    setEnableHighQualityIcons: (value: boolean) => void;

    /** Enable animations. */
    enableAnimations: boolean;
    setEnableAnimations: (value: boolean) => void;
};

export const usePersistStore = create<PersistStore>()(
    persist(
        (set) => {
            return {
                savedScript: STARTER_SCRIPT,
                setSavedScript: (savedScript) => {
                    set({ savedScript });
                    const env = parseEnvFromScript(savedScript);
                    // Set a separate local storage key for the boot flow
                    // to quickly display the logo
                    localStorage.setItem(
                        "Skybook.EarlyCI",
                        env.image ? "1" : "",
                    );
                },
                customImageVersion: "",
                setCustomImageVersion: (version) => {
                    if (version) {
                        set({ customImageVersion: version });
                    } else {
                        set({
                            customImageVersion: "",
                            isUseCustomImageByDefault: false,
                        });
                    }
                },
                isUseCustomImageByDefault: false as boolean,
                setUseCustomImageByDefault: (value) => {
                    set({ isUseCustomImageByDefault: value });
                },

                enableHighQualityIcons: true,
                setEnableHighQualityIcons: (value) => {
                    set({ enableHighQualityIcons: value });
                },

                enableAnimations: true,
                setEnableAnimations: (value) => {
                    set({ enableAnimations: value });
                },
            };
        },
        {
            name: "Skybook.Application",
            version: 2,
        },
    ),
);

export const useItemSlotPropsFromSettings = () => {
    const enableHighRes = usePersistStore(
        (state) => state.enableHighQualityIcons,
    );
    const enableAnimations = usePersistStore((state) => state.enableAnimations);
    return {
        cheap: !enableHighRes,
        disableAnimation: !enableAnimations,
    } satisfies Omit<ActorSpriteProps, "actor">;
};
