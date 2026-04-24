import type { RuntimeInitParams, RuntimeInitError } from "self::native";

import type { ScriptEnvImage } from "./misc.ts";

/** Diagnostic type for the script */
export interface Diagnostic {
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
}

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
          params: RuntimeInitParams;
          /** Don't load previously stored image, always ask app for new image */
          alwaysAskApp: boolean;
      };

export interface RuntimeWorkerInitOutput {
    /** Image version that was loaded */
    version: ScriptEnvImage | "";

    /**
     * The image version that is stored in the database
     */
    storedVersion: ScriptEnvImage | "" | "not-changed";
}

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
