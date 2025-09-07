import i18next from "i18next";
import { useTranslation } from "react-i18next";

import type { Translator } from "@pistonite/skybook-api";

export const translateUI = (key: string, options?: Record<string, unknown>) => {
    return i18next.t(`ui:${key}`, options);
};
export const useUITranslation = (): Translator => {
    const { t } = useTranslation("ui");
    return t;
};
