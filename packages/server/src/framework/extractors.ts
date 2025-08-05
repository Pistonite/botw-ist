import type { Result } from "@pistonite/pure/result";

/** Get if the request accepts Gzip encoding */
export const useAcceptsGzip = (req: Request): boolean => {
    const acceptEncoding = req.headers.get("Accept-Encoding");
    if (!acceptEncoding) {
        return false;
    }
    const parts = acceptEncoding.split(",");
    const len = parts.length;
    for (let i = 0; i < len; i++) {
        const part = parts[i].trimStart();
        if (part.match(/^[gG][zZ][iI][pP](;)?/)) {
            return true;
        }
    }
    return false;
};

export const useStringBody = async (req: Request): Promise<Result<string, unknown>> => {
    try {
        return { val: await req.text() };
    } catch (e) {
        return { err: e };
    }
};

/**
 * Parses the Accept-Language header and returns the best match
 * within the languages array. If no match is found, the fallback is returned
 */
export const useAcceptLanguage = <T extends string[]>(
    req: Request,
    languages: T,
    fallback: T[number],
): T[number] => {
    const acceptLanguage = req.headers.get("Accept-Language");
    if (!acceptLanguage || acceptLanguage.trim() === "*") {
        return fallback;
    }
    const parts = acceptLanguage.split(",");
    const choices = parts.map((part) => {
        let lang,
            q = 1;
        if (part.includes(";")) {
            const parts = part.split(";");
            lang = parts[0].trim().toLowerCase();
            const qPart = parts[1].trim();
            if (qPart.startsWith("q=")) {
                q = Number(qPart.substring(2).trim());
                if (isNaN(q)) {
                    q = 1;
                }
            }
        } else {
            lang = part.trim().toLowerCase();
            q = 1;
        }
        return { lang, q };
    });
    choices.sort((a, b) => b.q - a.q);
    const supportedLanguages = languages.map((lang) => lang.toLowerCase());
    const supportedLen = supportedLanguages.length;
    const choicesLen = choices.length;
    for (let i = 0; i < choicesLen; i++) {
        const { lang } = choices[i];
        if (!lang || lang.length < 2) {
            continue;
        }
        // first use exact match
        for (let j = 0; j < supportedLen; j++) {
            if (lang === supportedLanguages[j]) {
                return languages[j];
            }
        }
        // if not, use prefix match
        for (let j = 0; j < supportedLen; j++) {
            const langPrefix = lang.substring(0, 2);
            const supportedPrefix = supportedLanguages[j].substring(0, 2);
            if (langPrefix === supportedPrefix) {
                return languages[j];
            }
        }
    }
    return fallback;
};
