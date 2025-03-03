import { make404, RouteBuilder, } from "framework";

import { makeSSR } from "./ssr.ts";
import { makeAsset } from "./assets.ts";
import { useDirectLoadFromGitHubRepo, useDirectLoadFromHome, useDirectLoadFromUrl } from "./direct.ts";

export const createAppRoutes = async (builder: RouteBuilder) => {
    const commitFile = Bun.file("app/commit");
    const commit = (await commitFile.text()).trim();
    console.log("commit: " + commit);

    const assetRoute = builder.route({
        handler: makeAsset,
    })

    return {
        "/": builder.route({
            handler: (req, url) => {
                const directLoad = useDirectLoadFromHome(url);
                if (directLoad) {
                    return makeSSR(req, {
                        directLoad,
                    });
                }

                return makeSSR(req, {});
            },
        }),
        "/-/*": builder.route({
            handler: async (req, url) => {
                const directLoad = await useDirectLoadFromUrl(url);
                if (directLoad) {
                    return makeSSR(req, {
                        directLoad,
                    });
                }
                return make404();
            }
        }),
        "/github/:user/:repo/:branch/*": builder.route({
            handler: async (req, url) => {
                const { user, repo, branch } = (req as any).params;
                const directLoad = await useDirectLoadFromGitHubRepo(
                    user.trim(), 
                    repo.trim(), 
                    branch.trim(), 
                    url
                );
                if (directLoad) {
                    return makeSSR(req, {
                        directLoad,
                    });
                }
                return make404();
            }
        }),
        "/commit": new Response(commit),
        "/manifest.json": assetRoute,
        "/assets/*": assetRoute,
        "/static/*": assetRoute,
        "/runtime/*": assetRoute,
    };
}
