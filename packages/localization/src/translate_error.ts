import type {
    ParserError,
    RuntimeError,
    RuntimeViewError,
    RuntimeWorkerInitError,
} from "@pistonite/skybook-api";

import { type Translator, translateUI } from "./translate.ts";
import { translateActorOrAsIs, translateCategory } from "./translate_name.ts";

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
        case "InvalidCategoryName": {
            return translator(key, { arg: error.data });
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
