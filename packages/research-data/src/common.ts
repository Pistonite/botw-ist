/**
 * Common types in schema
 */
/** Locals supported by the game */
export type Locale = 
    "en-US"|
    "ja-JP"|
    "de-DE"|
    "es-ES"|
    "it-IT"|
    "fr-FR"|
    "ru-RU"|
    "zh-CN"|
    "zh-TW"|
    "ko-KR"|
    "nl-NL";

/** Locale names used by N */
export type LocaleNin =
    "USen" |
    "JPja" |
    "EUde" |
    "EUes" |
    "EUit" |
    "EUfr" |
    "EUru" |
    "CNzh" |
    "TWzh" |
    "KRko" |
    "EUnl";

export type ToLocaleNin<L extends Locale> = 
    L extends "en-US" ? "USen" :
    L extends "ja-JP" ? "JPja" :
    L extends "de-DE" ? "EUde" :
    L extends "es-ES" ? "EUes" :
    L extends "it-IT" ? "EUit" :
    L extends "fr-FR" ? "EUfr" :
    L extends "ru-RU" ? "EUru" :
    L extends "zh-CN" ? "CNzh" :
    L extends "zh-TW" ? "TWzh" :
    L extends "ko-KR" ? "KRko" :
    L extends "nl-NL" ? "EUnl" :
    never;

export type ToLocale<L extends LocaleNin> =
    L extends "USen" ? "en-US" :
    L extends "JPja" ? "ja-JP" :
    L extends "EUde" ? "de-DE" :
    L extends "EUes" ? "es-ES" :
    L extends "EUit" ? "it-IT" :
    L extends "EUfr" ? "fr-FR" :
    L extends "EUru" ? "ru-RU" :
    L extends "CNzh" ? "zh-CN" :
    L extends "TWzh" ? "zh-TW" :
    L extends "KRko" ? "ko-KR" :
    L extends "EUnl" ? "nl-NL" :
    never;
    
