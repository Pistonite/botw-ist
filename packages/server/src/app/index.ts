import { RouteBuilder, } from "framework";

import { convertLegacyScript } from "skybook-parser-legacy";

import { makeSSR } from "./ssr.ts";
import { makeAsset } from "./assets.ts";
import { decodeCompressedParam } from "./decode.ts";

console.log("initializing app routes");





export const createAppRoutes = async (builder: RouteBuilder) => {
    const commitFile = Bun.file("app/commit");
    const commit = (await commitFile.text()).trim();
    console.log("commit: " + commit);

    const assetRoute = builder.route({
        handler: makeAsset,
    })

    return {
        "/": builder.route({
            handler: (_req, url) => {
                // handle direct load through param
                // ?r=<v3 raw script>
                // ?c=<v3 compressed script>
                // ?v4=<v4 compressed script>
                const v4Param = url.searchParams.get("v4");
                if (v4Param) {
                    console.log("--- direct load v4");
                    const script = decodeCompressedParam(v4Param);
                    if (script.val) {
                        return makeSSR({
                            directLoad: {
                                type: "v4",
                                content: script.val,
                            }
                        });
                    }
                }
                const v3cParam = url.searchParams.get("c");
                if (v3cParam) {
                    console.log("--- direct load v3 compressed");
                    const script = decodeCompressedParam(v3cParam);
                    if (script.val) {
                        return makeSSR({
                            directLoad: {
                                type: "v3",
                                content: convertLegacyScript(script.val),
                            }
                        });
                    }
                }
                const v3rParam = url.searchParams.get("r");
                if (v3rParam) {
                    console.log("--- direct load v3 raw");
                    return makeSSR({
                        directLoad: {
                            type: "v3",
                            content: convertLegacyScript(v3rParam),
                        }
                    });
                }

                return makeSSR({});
            },
        }),
        "/commit": new Response(commit),
        "/manifest.json": assetRoute,
        "/assets/*": assetRoute,
        "/static/*": assetRoute,
        "/runtime/*": assetRoute,
    };
}
