import { type Translator, translateUI } from "../translate.ts";

/** Localize a generic error message */
export const translateGenericError = (error: string | undefined, translator:Translator = translateUI): string => {
    if (error) {
        return translator("error.internal", { error });
    }
    return translator("error.unknown");
};
