import type { ParserError } from "@pistonite/skybook-api";

import { translateUI } from "../translate.ts";

import { translateCategory } from "./category.ts";
import { translateGenericError } from "./error.ts";

export const translateParserError = (error: ParserError): string => {
    const key = `parser.${error.type}`;
    switch (error.type) {
        case "Unexpected":
            return translateGenericError(error.data);
        case "InvalidMetaValue": {
            const [metaKey, value] = error.data;
            return translateUI(key, { key: metaKey, value });
        }
        case "InvalidCategory": {
            const category = translateCategory(error.data);
            return translateUI(key, { arg: category });
        }
        case "InvalidEquipmentSlotNum": {
            const [categoryStr, num] = error.data;
            const category = translateCategory(categoryStr);
            return translateUI(key, { category, num });
        }
        default: {
            if ("data" in error) {
                return translateUI(key, { arg: error.data });
            }
            return translateUI(key);
        }
    }
};
