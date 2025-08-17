import type { Result } from "@pistonite/pure/result";

export type UrlParts = {
    origin: string;
    pathname: string;
    search: string;
};
export const safeParseUrl = (urlOrPath: string): Result<UrlParts, unknown> => {
    if (urlOrPath.startsWith("/")) {
        const queryStart = urlOrPath.indexOf("?");
        if (queryStart === -1) {
            return {
                val: {
                    origin: "",
                    pathname: urlOrPath,
                    search: "",
                },
            };
        }
        return {
            val: {
                origin: "",
                pathname: urlOrPath.substring(0, queryStart),
                search: urlOrPath.substring(queryStart),
            },
        };
    }
    try {
        const url = new URL(urlOrPath);
        return {
            val: {
                origin: url.origin,
                pathname: url.pathname,
                search: url.search,
            },
        };
    } catch (e) {
        if (urlOrPath.startsWith("http") || urlOrPath.startsWith("https")) {
            return { err: e };
        }
        const queryStart = urlOrPath.indexOf("?");
        if (queryStart === -1) {
            return {
                val: {
                    origin: "",
                    pathname: "/" + urlOrPath,
                    search: "",
                },
            };
        }
        return {
            val: {
                origin: "",
                pathname: "/" + urlOrPath.substring(0, queryStart),
                search: urlOrPath.substring(queryStart),
            },
        };
    }
};
