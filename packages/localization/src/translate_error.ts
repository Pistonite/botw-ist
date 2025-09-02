import type {
    ParserError,
    RuntimeError,
    RuntimeViewError,
    RuntimeWorkerInitError,
} from "@pistonite/skybook-api";

import { type Translator, translateUI , translateActorOrAsIs, translateCategory 
} from "./translate.ts";

/** Localize a generic error message */
export const translateGenericError = (
    error: string | undefined,
    translator: Translator = translateUI,
): string => {
    if (error) {
        return translator("error.internal", { error });
    }
    return translator("error.unknown");
};

export const translateParserError = (
    error: ParserError,
    translator: Translator = translateUI,
): string => {
    const errorKey = `parser.${error.type}`;
    switch (error.type) {
        case "Unexpected":
            return translateGenericError(error.data, translator);
        case "InvalidMetaValue": {
            const [metaKey, value] = error.data;
            return translator(errorKey, { key: metaKey, value });
        }
        case "InvalidCategory": {
            const category = translateCategory(error.data, translator);
            return translator(errorKey, { arg: category });
        }
        case "InvalidCategoryName": {
            return translator(errorKey, { arg: error.data });
        }
        case "InvalidEquipmentSlotNum": {
            const [categoryStr, num] = error.data;
            const category = translateCategory(categoryStr, translator);
            return translator(errorKey, { category, num });
        }
        case "InvalidSystemCommand": {
            const [key, value] = error.data;
            return translator(errorKey, { key: key, value });
        }
        case "GdtInvalidIndex": {
            const index = error.data;
            return translator(errorKey, { index });
        }
        default: {
            if ("data" in error) {
                return translator(errorKey, { arg: error.data });
            }
            return translator(errorKey);
        }
    }
};

export const translateRuntimeInitError = (
    error: RuntimeWorkerInitError,
    translator: Translator = translateUI,
): string => {
    const key = `runtime_init.${error.type}`;
    switch (error.type) {
        case "BadDlcVersion":
            return translator(key, { version: error.data });
        case "ProgramStartMismatch": {
            const [addr_ci, addr_script] = error.data;
            return translator(key, { addr_ci, addr_script });
        }
        default:
            return translator(key);
    }
};

export const translateRuntimeError = (
    error: RuntimeError,
    translator: Translator = translateUI,
): string => {
    const key = `runtime_error.${error.type}`;
    switch (error.type) {
        case "CannotFindItemNeedMore":
        case "CannotFindGroundItemNeedMore": {
            const more = error.data;
            return translator(key, { more });
        }
        case "ItemMismatch": {
            const [actual, expected] = error.data;
            return translator(key, {
                actual_item: translateActorOrAsIs(actual),
                expected_item: translateActorOrAsIs(expected),
            });
        }
        case "ItemMismatchCategory": {
            const [actual, expected] = error.data;
            return translator(key, {
                actual_item: translateActorOrAsIs(actual),
                expected_category: translateCategory(expected),
            });
        }
        case "NotEnoughForAllBut": {
            const [need, actual] = error.data;
            return translator(key, { need, actual });
        }
        case "SaveNotFound": {
            const name = error.data;
            return translator(key, { name });
        }
        case "CannotUseMore": {
            const time = error.data;
            return translator(key, { time });
        }
        case "CannotFindGdtFlag": {
            const [flag, type] = error.data;
            return translator(key, { flag, type });
        }
        case "InvalidGdtArrayIndex": {
            const [flag, type, index] = error.data;
            return translator(key, { flag, type, index });
        }
        default:
            return translator(key);
    }
};

export const translateRuntimeViewError = (
    error: RuntimeViewError,
    translator: Translator = translateUI,
): string => {
    const key = `runtime_view_error.${error.type}`;
    return translator(key);
};
