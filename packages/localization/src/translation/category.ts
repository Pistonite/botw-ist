import type { Category } from "@pistonite/skybook-api";

import { translateUI } from "../translate.ts";

/** Translate item category string enum */
export const translateCategory = (category: Category): string => {
    return translateUI(`category.${category}`);
}
