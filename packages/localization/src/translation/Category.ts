import type { Category } from "@pistonite/skybook-api";

import { type Translator, translateUI } from "../translate.ts";

/** Translate item category string enum */
export const translateCategory = (category: Category, translator: Translator = translateUI): string => {
    return translator(`category.${category}`);
};
