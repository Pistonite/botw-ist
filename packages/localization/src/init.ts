import { initLocale, registerTranslationLoader } from "@pistonite/celera";

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

export const initI18n = async (persist: boolean) => {
    await initLocale({
        supported: SupportedLocales,
        default: "en-US",
        persist,
    });
    registerTranslationLoader("ui", loadUILanguage);
};

const loadUILanguage = async (language: string): Promise<Record<string, string>> => {
    return (await import(`./ui/${language}.yaml`)).default;
};
