import type { Category } from "@pistonite/skybook-api";

import {
    type Translator,
    translateGenerated,
    translateUI,
} from "./translate.ts";

/** Translate item category string enum */
export const translateCategory = (
    category: Category,
    translator: Translator = translateUI,
): string => {
    return translator(`category.${category}`);
};

/** Translate an actor name, and fall back to the input string if no translation is available */
export const translateActorOrAsIs = (
    actor: string,
    translator: Translator = translateGenerated,
): string => {
    const translated = translator(`actor.${actor}.name`);
    if (!translated) {
        return actor;
    }
    // since we don't know what the effect is, just return
    // the base actor name
    return translated.replace("{{effect}}", "");
};
