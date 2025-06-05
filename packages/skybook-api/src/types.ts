/**
 * Common types
 *
 * @module
 */

import type { ScriptEnvImage } from "./envParser.ts";
import type { CustomImageInitParams, RuntimeInitError } from "./native";

/**
 * Type used for JS side item search queries
 */
export type ItemSearchResult = {
    /** The item actor name (for example Weapon_Sword_502 */
    actor: string;
    /**
     * The cook effect of the item.
     *
     * The number is the game's representation (the CookEffect enum in decomp project).
     * If the item should not have an effect, the value should be 0 (instead of -1)
     */
    cookEffect: number;
};

/** Diagnostic type for the script */
export type Diagnostic = {
    /** (Localized) message to display */
    message: string;
    /** Start character position of the diagnostic (inclusive) */
    start: number;
    /** End character position of the diagnostic (exclusive) */
    end: number;
    /**
     * Whether this diagnostic is only a warning. If false, it should be treated as an error
     */
    isWarning: boolean;
};

/** Args for initializing the runtime */
export type RuntimeWorkerInitArgs =
    | {
          /** If a stored custom image should be loaded */
          isCustomImage: false;
          /** If previously stored images should be deleted */
          deleteCustomImage: boolean;
      }
    | {
          /** If a stored custom image should be loaded */
          isCustomImage: true;
          params: CustomImageInitParams;
          /** Don't load previously stored image, always ask app for new image */
          alwaysAskApp: boolean;
      };

export type RuntimeWorkerInitOutput = {
    /** Image version that was loaded */
    version: ScriptEnvImage | "";

    /**
     * The image version that is stored in the database
     */
    storedVersion: ScriptEnvImage | "" | "not-changed";
};

export type RuntimeWorkerInitError =
    | {
          /** Failed to get custom image from app */
          type: "NoImageFromApp";
      }
    | {
          /** Failed to save custom image */
          type: "SaveImage";
      }
    | RuntimeInitError;

export type PerformanceData = {
    /** Instructions per second */
    ips: number;
    /** Steps per second */
    sps: number;
};
