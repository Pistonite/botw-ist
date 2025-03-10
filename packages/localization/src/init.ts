import { initLocaleWithI18next } from "@pistonite/pure-i18next";

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

export const NativelyMaintainedLocales = ["en-US", "zh-CN"];

export const initI18n = () => {
    return initLocaleWithI18next({
        supported: SupportedLocales,
        default: "en-US",
        persist: true,
        loader: {
            ui: (language) => loadLanguage("ui", language),
            generated: (language) => loadLanguage("generated", language),
        },
    });
};

const loadLanguage = async (namespace: string, language: string) => {
    const strings = (await import(`./${namespace}/${language}.yaml`)).default;
    if (namespace === "ui" && language !== "en-US") {
        const enStrings = (await import(`./${namespace}/en-US.yaml`)).default;
        let countMissing = 0;
        for (const key in enStrings) {
            if (!(key in strings)) {
                strings[key] = enStrings[key];
                countMissing++;
            }
        }
        if (countMissing > 0) {
            console.warn(
                `Missing ${countMissing} strings in ${language} UI, falling back to en-US`,
            );
        }
    }
    return strings;
};
