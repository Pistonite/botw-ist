/// <reference types="vitest" />

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import { spawnSync } from "child_process";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import path from "path";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import fs from "fs";

import { type Plugin, defineConfig, type UserConfig } from "vite";
import serveStatic from "vite-plugin-serve-static";
import intwc from "@pistonite/vite-plugin-intwc";
import monodev from "mono-dev/vite";

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
        },
    };
};

// https://vitejs.dev/config/
export default defineConfig(({ command }) => {
    const commit = spawnSync("git", ["rev-parse", "HEAD"], {
        encoding: "utf-8",
    }).stdout.trim();
    console.log(`commit: ${commit}`);

    const packageJson = JSON.parse(fs.readFileSync("../../package.json", "utf-8"));
    const version = packageJson.version;
    console.log(`version: ${version}`);

    const monodevConfig = monodev({
        https: command === "serve",
    });
    return monodevConfig<UserConfig>({
        define: {
            "import.meta.env.COMMIT": JSON.stringify(commit),
            "import.meta.env.VERSION": JSON.stringify(version),
            "import.meta.vitest": "undefined",
        },
        plugins: [
            intwc({ basicLanguages: [] }),
            staticAssetHeader(),
            serveStatic([
                {
                    pattern: /^\/runtime\/(.*)/,
                    resolve: ([_, capture]) => path.join("..", "runtime-wasm", "dist", capture),
                },
                {
                    pattern: /^\/static\/item-assets\/(.*)/,
                    resolve: ([_, capture]) => path.join("..", "item-assets", "public", capture),
                },
                {
                    pattern: /^\/static\/item-system\/(.*)/,
                    resolve: ([_, capture]) => path.join("..", "item-system", "public", capture),
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
                input: {
                    index: "index.html",
                    popout: "popout.html",
                },
                output: {
                    chunkFileNames: (info) => {
                        for (let i = 0; i < info.moduleIds.length; i++) {
                            if (info.moduleIds[i].match(/localization[/\\]src[/\\]ui/)) {
                                return `assets/strings/ui-${info.name}-[hash].js`;
                            }
                            if (info.moduleIds[i].match(/localization[/\\]src[/\\]generated/)) {
                                return `assets/strings/gen-${info.name}-[hash].js`;
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
        test: {
            includeSource: ["src/**/*.{js,ts}"],
            environment: "jsdom",
        },
    });
});
