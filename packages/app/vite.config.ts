// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import { spawnSync } from "child_process";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import path from "path";

import { type Plugin, defineConfig, type UserConfig } from "vite";
import serveStatic from "vite-plugin-serve-static";
import intwc from "@pistonite/vite-plugin-intwc";
import monodev from "mono-dev/vite";

const commit = spawnSync("git", ["rev-parse", "HEAD"], {
    encoding: "utf-8",
}).stdout.trim();
console.log(commit);


const staticAssetHeader = (): Plugin => {
    return {
        name: "static-asset-header",
        apply: "serve",
        configureServer(server) {
            server.middlewares.use((req, res, next) => {
                if (req.url?.startsWith("/runtime/")) {
                    res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
                    res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
                }
                next();
            });
        }
    }
};

// https://vitejs.dev/config/
export default defineConfig( ({command}) => {
    const monodevConfig = monodev({
        https: command === "serve",
    });
    return monodevConfig<UserConfig>({
        define: {
            "import.meta.env.COMMIT": JSON.stringify(commit),
        },
        plugins: [
            intwc({ basicLanguages: [] }),
            staticAssetHeader(),
            serveStatic([
                {
                    pattern: /^\/runtime\/(.*)/,
                    resolve: ([_, capture]) =>
                        path.join("..", "runtime-wasm", "dist", capture),
                },
                {
                    pattern: /^\/static\/item-assets\/(.*)/,
                    resolve: ([_, capture]) =>
                        path.join("..", "item-assets", "public", capture),
                },
                {
                    pattern: /^\/static\/item-system\/(.*)/,
                    resolve: ([_, capture]) =>
                        path.join("..", "item-system", "public", capture),
                },
            ]),
        ],
        resolve: {
            dedupe: ["botw-item-assets"],
        },
        server: {
            port: 23172,
            headers: {
                "Cross-Origin-Embedder-Policy": "require-corp",
                "Cross-Origin-Opener-Policy": "same-origin",
            },
        },
        build: {
            rollupOptions: {
                output: {
                    chunkFileNames: (info) => {
                        for (let i = 0; i < info.moduleIds.length; i++) {
                            if (
                                info.moduleIds[i].match(
                                    /localization[/\\]src[/\\]ui/,
                                )
                            ) {
                                return `assets/strings/ui-${info.name}-[hash].js`;
                            }
                            if (
                                info.moduleIds[i].match(
                                    /localization[/\\]src[/\\]generated/,
                                )
                            ) {
                                return `assets/strings/gen-${info.name}-[hash].js`;
                            }
                            if (
                                info.moduleIds[i].match(
                                    /app[/\\]src[/\\]extensions/,
                                )
                            ) {
                                return `assets/exts/${info.name}-[hash].js`;
                            }
                        }
                        return `assets/${info.name}-[hash].js`;
                    },
                    manualChunks: {
                        strings: ["skybook-localization"],
                    },
                },
            },
        },
    });
});
