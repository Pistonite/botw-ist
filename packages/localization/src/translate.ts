import i18next from "i18next";
import { useTranslation } from "react-i18next";
import { useCallback } from "react";

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
