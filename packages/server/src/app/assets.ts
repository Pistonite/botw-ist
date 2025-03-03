import { readdir } from "node:fs/promises";

import {
    make404,
    makeFile,
    type ResponsePayload,
    useAcceptsGzip,
} from "util/framework";

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
    const path = new URL(req.url).pathname;
    if (!path.startsWith("/") || path.endsWith("/") || path.endsWith(".")) {
        return make404();
    }
    const localPath = "app" + path;
    if ((await gzipPaths).has(path) && useAcceptsGzip(req)) {
        const gzipPath = localPath + ".gz";
        return makeFile(gzipPath);
    }
    return makeFile(localPath);
};
