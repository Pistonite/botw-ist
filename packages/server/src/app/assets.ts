import { readdir } from "node:fs/promises";

import { make404, makeFile, type ResponsePayload, useAcceptsGzip, useUrl } from "self::framework";

const gzipPaths = (async () => {
    console.log("loading gzipped assets paths");
    const gzipPaths = new Set<string>();
    for (const path of await readdir("app")) {
        if (path.endsWith(".gz")) {
            gzipPaths.add("/" + path.slice(0, -3));
        }
    }
    return gzipPaths;
})();

/** Make an asset response */
export const makeAsset = async (req: Request): Promise<ResponsePayload> => {
    const path = useUrl(req).pathname;
    if (!path.startsWith("/") || path.endsWith("/") || path.endsWith(".")) {
        return make404();
    }
    const localPath = "app" + path;
    let filePath = localPath;
    if ((await gzipPaths).has(path) && useAcceptsGzip(req)) {
        filePath += ".gz";
    }
    // Workers are frames, so COEP=require-corp is needed
    // to be embedded in the main frame.
    return makeFile(
        filePath,
        isWorker(path) && {
            headers: {
                "Cross-Origin-Embedder-Policy": "require-corp",
                "Cross-Origin-Opener-Policy": "same-origin",
            },
        },
    );
};

const isWorker = (path: string): boolean => {
    if (path.startsWith("/assets/editor.worker")) {
        return true;
    }
    if (path.startsWith("/runtime/") && path.endsWith(".js")) {
        return true;
    }
    return false;
};
