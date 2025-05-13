import { initLocaleWithI18next } from "@pistonite/pure-i18next";

import {
    loadSharedControlLanguage,
    namespace as sharedControlsNamespace,
} from "@pistonite/shared-controls";

export const SupportedLocales = [
    "de-DE",
    "en-US",
    "es-ES",
    "fr-FR",
    "it-IT",
    "ja-JP",
    "ko-KR",
    "nl-NL",
    "ru-RU",
    "zh-CN",
    "zh-TW",
] as const;

export const initI18n = (persist: boolean) => {
    return initLocaleWithI18next({
        supported: SupportedLocales,
        default: "en-US",
        persist,
        loader: {
            ui: (language) => loadLanguage("ui", language),
            generated: (language) => loadLanguage("generated", language),
            [sharedControlsNamespace]: loadSharedControlLanguage,
        },
    });
};

const loadLanguage = async (namespace: string, language: string) => {
    return (await import(`./${namespace}/${language}.yaml`)).default;
};
