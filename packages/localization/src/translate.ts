import i18next from "i18next";
import { useTranslation } from "react-i18next";
import { useCallback } from "react";

import type { Category } from "@pistonite/skybook-api";

export type Translator = (key: string, options?: Record<string, unknown>) => string;

export const translateUI = (key: string, options?: Record<string, unknown>) => {
    return i18next.t(`ui:${key}`, options);
};
export const translateGenerated = (key: string, options?: Record<string, unknown>) => {
    const value = i18next.t(`generated:${key}`, options);
    if (value === key) {
        return "";
    }
    return value;
};

export const useUITranslation = (): Translator => {
    const { t } = useTranslation("ui");
    return t;
};

export const useGeneratedTranslation = (): Translator => {
    const { t } = useTranslation("generated", { nsMode: "default" });
    // return empty string if the key is not found, similar to the game
    return useCallback(
        (key: string, options?: Record<string, unknown>) => {
            const value = t(key, options);
            if (value === key) {
                return "";
            }
            return value;
        },
        [t],
    );
};
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
