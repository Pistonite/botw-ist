import {
    type Routes,
    make404,
    type RouteBuilder,
    withHeadersOnSuccess,
    makeFile,
} from "self::framework";

import { makeSSR } from "./ssr.ts";
import { makeAsset } from "./assets.ts";
import {
    useDirectLoadFromGitHubRepo,
    useDirectLoadFromHome,
    useDirectLoadFromUrl,
} from "./direct.ts";

const commitFile = Bun.file("app/commit");
const COMMIT = (await commitFile.text()).trim();
console.log("commit: " + COMMIT);

const withCorpHeaders = withHeadersOnSuccess({
    "Cross-Origin-Embedder-Policy": "require-corp",
    "Cross-Origin-Opener-Policy": "same-origin",
});

const withNoCacheHeaders = withHeadersOnSuccess({
    "Cache-Control": "no-cache",
});

const withCacheForeverHeaders = withHeadersOnSuccess({
    "Cache-Control": "public, max-age=31535000, immutable",
});

export const createAppRoutes = (builder: RouteBuilder): Routes => {
    const popoutUrl = `/popout-${COMMIT.substring(0, 8)}`;
    return {
        "/": builder.route({
            handler: (req, url) => {
                const directLoad = useDirectLoadFromHome(url);
                if (directLoad) {
                    return makeSSR(req, {
                        url: url.origin + url.pathname + url.search,
                        directLoad,
                    });
                }

                return makeSSR(req, {
                    url: url.origin + url.pathname,
                });
            },
            outbound: [
                withCorpHeaders,
                // Home page can be cached because the script is embedded in the url
                // but we also don't want to cache it for too long
                // when an update is deployed
                withHeadersOnSuccess({
                    "Cache-Control": "public, max-age=600",
                }),
            ],
        }),
        "/-/*": builder.route({
            handler: async (req, url) => {
                const directLoad = await useDirectLoadFromUrl(url);
                if (directLoad) {
                    return makeSSR(req, {
                        url: url.origin + url.pathname + url.search,
                        directLoad,
                    });
                }
                return make404();
            },
            outbound: [
                withCorpHeaders,
                // direct load urls should not be cached
                // to always load the latest version
                withNoCacheHeaders,
            ],
        }),
        "/github/:user/:repo/:branch/*": builder.route({
            handler: async (req, url) => {
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                const { user, repo, branch } = (req as any).params;
                const directLoad = await useDirectLoadFromGitHubRepo(
                    user.trim(),
                    repo.trim(),
                    branch.trim(),
                    url,
                );
                if (directLoad) {
                    return makeSSR(req, {
                        url: url.origin + url.pathname,
                        directLoad,
                    });
                }
                return make404();
            },
            outbound: [
                withCorpHeaders,
                // github contents are cached for 5 minutes
                // so we also cache that long
                withHeadersOnSuccess({
                    "Cache-Control": "public, max-age=301",
                }),
            ],
        }),
        "/commit": new Response(COMMIT, {
            headers: {
                "Content-Type": "text/plain",
                "Cache-Control": "no-cache",
            },
        }),
        [popoutUrl]: builder.route({
            handler: () =>
                makeFile(`app${popoutUrl}.html`, {
                    headers: {
                        "Cache-Control": "public, max-age=600",
                    },
                }),
        }),
        // bundled assets are hashed and can be cached forever
        "/assets/*": builder.route({
            handler: makeAsset,
            outbound: [withCacheForeverHeaders],
        }),
        // runtime is versioned by commit hash
        "/runtime/*": builder.route({
            handler: makeAsset,
            outbound: [withCacheForeverHeaders],
        }),
        // these static assets are unlikely to change (images)
        // cache for 7 days
        "/static/*": builder.route({
            handler: makeAsset,
            outbound: [
                withHeadersOnSuccess({
                    "Cache-Control": "public, max-age=604800",
                }),
            ],
        }),
        // other assets, cache for a standard 1 day
        "/manifest.json": builder.route({
            handler: makeAsset,
            outbound: [
                withHeadersOnSuccess({
                    "Cache-Control": "public, max-age=86400",
                }),
            ],
        }),
    };
};
