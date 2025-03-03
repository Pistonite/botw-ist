import { readdir } from "node:fs/promises";

import {
    make404,
    makeFile,
    type ResponsePayload,
    useAcceptsGzip,
} from "util/framework";

console.log("loading gzipped assets paths");
const gzipPaths = new Set<string>();
for (const path of await readdir("app")) {
    if (path.endsWith(".gz")) {
        gzipPaths.add("/" + path.slice(0, -3));
    }
}

/** Make an asset response */
export const makeAsset = (req: Request): ResponsePayload => {
    const path = new URL(req.url).pathname;
    if (!path.startsWith("/") || path.endsWith("/") || path.endsWith(".")) {
        return make404();
    }
    const localPath = "app" + path;
    if (gzipPaths.has(path) && useAcceptsGzip(req)) {
        const gzipPath = localPath + ".gz";
        return makeFile(gzipPath);
    }
    return makeFile(localPath);
};
