import { initLocaleWithI18next } from "@pistonite/pure-i18next";
import { getPureI18nextLoaderConfig as sharedControlsLoader } from "@pistonite/shared-controls";

import { getPureI18nextLoaderConfig as itemsysLoader } from "@pistonite/skybook-itemsys";

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
            ...itemsysLoader(),
            ...sharedControlsLoader(),
            ui: loadUILanguage,
        },
    });
};

const loadUILanguage = async (language: string): Promise<Record<string, string>> => {
    return (await import(`./ui/${language}.yaml`)).default;
};
