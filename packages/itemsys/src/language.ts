
export const loadTranslations = async (language: string): Promise<Record<string, string>> => {
    return (await import(`./generated/${language}.yaml`)).default;
};
