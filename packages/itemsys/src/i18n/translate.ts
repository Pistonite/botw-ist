import { useCallback } from "react";
import { translate, useTranslation, type TranslatorFn } from "@pistonite/celera";
import type { Category } from "@pistonite/skybook-api";

/**
 * Translated a key in the generated item translations.
 *
 * Returns empty string if the key is not found.
 * Requires I18next to be initialized with translations from this package.
 */
export const translateGen = (key: string, options?: Record<string, unknown>) => {
    const value = translate(`skybook-itemsys:${key}`, options);
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
    return translate(`skybook-itemsys-ui:${key}`, options);
};

/**
 * React hook for UI translations
 *
 * Requires react-i18next and i18next to be initialized with translations
 * from this package.
 */
export const useUITranslation = (): TranslatorFn => {
    return useTranslation("skybook-itemsys-ui");
};

/**
 * React hook for generated item translations.
 */
export const useGenTranslation = (): TranslatorFn => {
    const t = useTranslation("skybook-itemsys", { nsMode: "default" });
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
    translator: TranslatorFn = translateUI,
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
    translator: TranslatorFn = translateGen,
): string => {
    const translated = translator(`actor.${actor}.name`);
    if (!translated) {
        return actor;
    }
    // since we don't know what the effect is, just return
    // the base actor name
    return translated.replace("{{effect}}", "");
};
