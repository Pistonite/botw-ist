import { translate, useTranslation, type TranslatorFn } from "@pistonite/celera";

export const translateUI = (key: string, options?: Record<string, unknown>) => {
    return translate(`ui:${key}`, options);
};
export const useUITranslation = (): TranslatorFn => {
    return useTranslation("ui");
};
