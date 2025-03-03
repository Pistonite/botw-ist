import { errstr, type Result } from "@pistonite/pure/result";

import type { DirectLoad } from "@pistonite/skybook-api";
import { convertLegacyScript } from "skybook-parser-legacy";

import type { URL } from "server/framework";

const decodeCompressedParam = (param: string): Result<string, string> => {
    try {
        const compressedBytes = Buffer.from(param, "base64");
        const uncompressedBytes = Bun.gunzipSync(compressedBytes);
        return { val: new TextDecoder().decode(uncompressedBytes) };
    } catch (e) {
        console.error("error decoding compressed param: ", e);
        return { val: errstr(e) };
    }
};

/** Handle DirectLoad from the home page (/) */
export const useDirectLoadFromHome = (url: URL): DirectLoad | undefined => {
    // handle direct load through param
    // ?r=<v3 raw script>
    // ?c=<v3 compressed script>
    // ?v4=<v4 compressed script>
    const v4Param = url.searchParams.get("v4");
    if (v4Param) {
        const script = decodeCompressedParam(v4Param);
        if (script.val) {
            console.log("--- direct load v4");
            return {
                type: "v4",
                content: script.val,
            };
        }
    }
    const v3cParam = url.searchParams.get("c");
    if (v3cParam) {
        const script = decodeCompressedParam(v3cParam);
        if (script.val) {
            console.log("--- direct load v3 compressed");
            return {
                type: "v3",
                content: convertLegacyScript(script.val),
            };
        }
    }
    const v3rParam = url.searchParams.get("r");
    if (v3rParam) {
        console.log("--- direct load v3 raw");
        return {
            type: "v3",
            content: convertLegacyScript(v3rParam),
        };
    }

    return undefined;
};

/** Handle DirectLoad from any URL (/-/) */
export const useDirectLoadFromUrl = async (
    url: URL,
): Promise<DirectLoad | undefined> => {
    const pathname = url.pathname.trim();
    if (!pathname.startsWith("/-/")) {
        return undefined;
    }
    const directURL = pathname.replace(/^\/-\//, "https://") + url.search;
    console.log("--- direct load from: " + directURL);
    const content = await fetchContent(directURL, true);
    if ("err" in content) {
        console.error(`error fetching direct load: ${errstr(content.err)}`);
        return undefined;
    }
    return {
        type: "v4",
        content: content.val,
    };
};

export const useDirectLoadFromGitHubRepo = async (
    user: string,
    repo: string,
    branch: string,
    url: URL,
): Promise<DirectLoad | undefined> => {
    const pathname = url.pathname.trim();
    const prefix = `/github/${user}/${repo}/${branch}/`;
    if (!pathname.startsWith(prefix)) {
        return undefined;
    }
    const filePath = pathname.substring(prefix.length).trim();
    const directURL = `https://raw.githubusercontent.com/${user}/${repo}/${branch}/${filePath}`;
    const content = await fetchContent(directURL);
    if ("err" in content) {
        console.error(`error fetching direct load: ${errstr(content.err)}`);
        return undefined;
    }
    return {
        type: "v4",
        content: content.val,
    };
};

const fetchContent = async (
    url: string,
    enforceContentType?: boolean,
): Promise<Result<string, unknown>> => {
    try {
        const response = await fetch(url);
        if (!response.ok) {
            return { err: response.status };
        }
        if (enforceContentType) {
            if (!isTextPlain(response.headers.get("Content-Type"))) {
                return { err: "content type is not text/plain" };
            }
        }
        return { val: await response.text() };
    } catch (e) {
        return { err: e };
    }
};

const isTextPlain = (contentType: string | null): boolean => {
    if (!contentType) {
        return false;
    }
    return contentType.startsWith("text/plain");
};
