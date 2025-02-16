import type { ParserError } from "@pistonite/skybook-api";

import { translateUI } from "../translate.ts";

import { translateCategory } from "./category.ts";

export const translateParserError = (error: ParserError): string => {
    const key = `parser.${error.type}`;
    switch (error.type) {
        case "Unexpected": 
            return translateUI("generic.error.internal", { error: error.data });
        case "InvalidMetaValue": {
            const [metaKey, value] = error.data;
            return translateUI(key, { key: metaKey, value });
        }
        case "InvalidCategory": {
            const category = translateCategory(error.data);
            return translateUI(key, { arg: category });
        }
        default: {
            if ("data" in error) {
                return translateUI(key, { arg: error.data });
            }
            return translateUI(key);
        }
    }
};
