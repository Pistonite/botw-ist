/** Load translation entries for items, generated from the game */
export const loadItemTranslations = async (language: string): Promise<Record<string, string>> => {
    return (await import(`../generated/${language}.yaml`)).default;
};

/** Load UI translation entries for the item system */
export const loadItemUITranslations = async (language: string): Promise<Record<string, string>> => {
    return (await import(`./ui/${language}.yaml`)).default;
};

/** Get the loader config for working with the pure/i18next framework */
export const getPureI18nextLoaderConfig = (): {
    "skybook-itemsys": (language: string) => Promise<Record<string, string>>;
    "skybook-itemsys-ui": (language: string) => Promise<Record<string, string>>;
} => {
    return {
        "skybook-itemsys": loadItemTranslations,
        "skybook-itemsys-ui": loadItemUITranslations,
    };
};
