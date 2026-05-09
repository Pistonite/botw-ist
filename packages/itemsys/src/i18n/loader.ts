/** Load translation entries for items, generated from the game */
export const loadItemTranslations = async (language: string): Promise<Record<string, string>> => {
    return (await import(`../generated/${language}.yaml`)).default;
};

/** Load UI translation entries for the item system */
export const loadItemUITranslations = async (language: string): Promise<Record<string, string>> => {
    return (await import(`./ui/${language}.yaml`)).default;
};
