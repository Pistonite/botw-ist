import i18next from "i18next";
import { useTranslation } from "react-i18next";
import { useCallback } from "react";

import type { Category, Translator } from "@pistonite/skybook-api";

/**
 * Translated a key in the generated item translations.
 *
 * Returns empty string if the key is not found.
 * Requires I18next to be initialized with translations from this package.
 */
export const translateGen = (key: string, options?: Record<string, unknown>) => {
    const value = i18next.t(`skybook-itemsys:${key}`, options);
    if (value === key) {
        return "";
    }
    return value;
};

/**
 * Translated a key in the generated item translations.
 *
 * Returns the key as-is if the key is not found.
 * Requires I18next to be initialized with translations from this package.
 */
export const translateUI = (key: string, options?: Record<string, unknown>) => {
    return i18next.t(`skybook-itemsys-ui:${key}`, options);
};

/**
 * React hook for UI translations
 *
 * Requires react-i18next and i18next to be initialized with translations
 * from this package.
 */
export const useUITranslation = (): Translator => {
    const { t } = useTranslation("skybook-itemsys-ui");
    return t;
};

/**
 * React hook for generated item translations.
 *
 * Requires react-i18next and i18next to be initialized with translations
 * from this package.
 */
export const useGenTranslation = (): Translator => {
    const { t } = useTranslation("skybook-itemsys", { nsMode: "default" });
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

/**
 * Translate item category string enum
 *
 * Requires I18next to be initialized with translations from this package.
 */
export const translateCategory = (
    category: Category,
    translator: Translator = translateUI,
): string => {
    return translator(`category.${category}`);
};

/**
 * Translate an actor name, and fall back to the input string if no translation is available.
 *
 * Requires I18next to be initialized with translations from this package.
 */
export const translateActorOrAsIs = (
    actor: string,
    translator: Translator = translateGen,
): string => {
    const translated = translator(`actor.${actor}.name`);
    if (!translated) {
        return actor;
    }
    // since we don't know what the effect is, just return
    // the base actor name
    return translated.replace("{{effect}}", "");
};
