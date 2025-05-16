import type { WorkerError } from "skybook-runtime-worker";

import { translateUI } from "../translate.ts";

export const translateWorkerError = (error: WorkerError): string => {
    const key = `worker.${error.type}`;
    switch (error.type) {
        case "Aborted":
            return translateUI("error.aborted");
        default: {
            let message = translateUI(key);
            if ("message" in error) {
                // the message is not localized, but better than nothing
                message += ": " +error.message;
            }
            return message;
        }
    }
};
