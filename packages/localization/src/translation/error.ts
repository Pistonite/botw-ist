import { translateUI } from "../translate.ts";

/** Localize a generic error message */
export const translateGenericError = (error?: string): string => {
    if (error) {
        return translateUI("error.internal", { error });
    }
    return translateUI("error.unknown");
};
