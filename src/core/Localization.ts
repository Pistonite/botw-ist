// For now only en_US is supported, but it will be straightforward to add language options later
import display from "config/i18n/en_US.json";
type LanguageMap = Record<string, string>;
const displayMap = display as LanguageMap;

export const getDisplayValue = (displayKey: string, defaultValue?: string) => {
	return displayMap[displayKey] ?? defaultValue ?? displayKey;
};
