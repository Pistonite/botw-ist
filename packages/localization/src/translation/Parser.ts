import type { ParserError } from "@pistonite/skybook-api";

import { type Translator, translateUI } from "../translate.ts";

import { translateCategory } from "./Category.ts";
import { translateGenericError } from "./Error.ts";

export const translateParserError = (
    error: ParserError,
    translator: Translator = translateUI,
): string => {
    const key = `parser.${error.type}`;
    switch (error.type) {
        case "Unexpected":
            return translateGenericError(error.data, translator);
        case "InvalidMetaValue": {
            const [metaKey, value] = error.data;
            return translator(key, { key: metaKey, value });
        }
        case "InvalidCategory": {
            const category = translateCategory(error.data, translator);
            return translator(key, { arg: category });
        }
        case "InvalidEquipmentSlotNum": {
            const [categoryStr, num] = error.data;
            const category = translateCategory(categoryStr, translator);
            return translator(key, { category, num });
        }
        default: {
            if ("data" in error) {
                return translator(key, { arg: error.data });
            }
            return translator(key);
        }
    }
};
