import type { BackendModule } from "i18next";
import i18next from "i18next";
import { initReactI18next, useTranslation } from "react-i18next";
import { convertToSupportedLocale, detectLocale, initLocale } from "@pistonite/pure/pref";

export const backend: BackendModule = {
    type: "backend",
    init: () => {
        // no init needed
    },
    read: async (language: string, namespace: string) => {
        const locale = convertToSupportedLocale(language);
        let strings;
        try {
            strings  = await import(`./${namespace}/${locale}.yaml`);
        } catch {
            try {
                strings = await import(`./${namespace}/en-US.yaml`);
            } catch {
                return undefined;
            }
            console.warn(`${locale} is not supported for ${namespace} namespace. Falling back to en-US.`);
        }
        return strings.default;
    }
}

export const SupportedLocales = 
    [
        "de-DE", 
        "en-US", 
        "es-ES", 
        "fr-FR", 
        "it-IT", 
        "ja-JP", "ko-KR", 
        "nl-NL", 
        "ru-RU", "zh-CN", "zh-TW"
    ] as const;

export const initI18n = async () => {
    initLocale({
        supported: SupportedLocales,
        default: "en-US",
        persist: true,
    });

    await i18next.use(detectLocale).use(backend).use(initReactI18next).init();
}

export const translateUI = (key: string, options?: Record<string, unknown>) => {
    return i18next.t(`ui:${key}`, options);
}
export const translateGenerated = (key: string, options?: Record<string, unknown>) => {
    return i18next.t(`generated:${key}`, options);
}

export const useUITranslation = () => {
    const {t} = useTranslation("ui");
    return t;
}

export const useGeneratedTranslation = () => {
    const {t} = useTranslation("generated");
    return t;
}
