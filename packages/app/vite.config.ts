/// <reference types="mono-dev/vitest" />
/// <reference types="node" />

import path from "node:path";

import { type Plugin } from "mono-dev/vite";
import serveStatic from "vite-plugin-serve-static";
import intwc from "@pistonite/intwc/vite-plugin";
import { configure } from "mono-dev/app-build-config";

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

export default configure(() => {
    return {
        plugins: [
            intwc({ basicLanguages: [] }),
            staticAssetHeader(),
            serveStatic([
                {
                    pattern: /^\/runtime\/(.*)/,
                    resolve: ([_, capture]) => path.join("..", "runtime-wasm", "dist", capture),
                },
                {
                    pattern: /^\/static\/itemsys\/(.*)/,
                    resolve: ([_, capture]) => path.join("..", "itemsys-build", "public", capture),
                },
            ]),
        ],
        server: {
            port: 23172,
            headers: {
                "Cross-Origin-Embedder-Policy": "require-corp",
                "Cross-Origin-Opener-Policy": "same-origin",
            },
        },
        build: {
            rolldownOptions: {
                input: {
                    index: "index.html",
                    popout: "popout.html",
                },
                output: {
                    codeSplitting: {
                        groups: [
                            // generated translation chunks
                            { name: "gen/de", test: /generated[\\/]de/ },
                            { name: "gen/en", test: /generated[\\/]en/ },
                            { name: "gen/es", test: /generated[\\/]es/ },
                            { name: "gen/fr", test: /generated[\\/]fr/ },
                            { name: "gen/it", test: /generated[\\/]it/ },
                            { name: "gen/ja", test: /generated[\\/]ja/ },
                            { name: "gen/ko", test: /generated[\\/]ko/ },
                            { name: "gen/nl", test: /generated[\\/]nl/ },
                            { name: "gen/ru", test: /generated[\\/]ru/ },
                            { name: "gen/zh", test: /generated[\\/]zh/ },
                            // combined other translation chunks
                            { name: "msg/de", test: "de-DE" },
                            { name: "msg/en", test: "en-US" },
                            { name: "msg/es", test: "es-ES" },
                            { name: "msg/fr", test: "fr-FR" },
                            { name: "msg/it", test: "it-IT" },
                            { name: "msg/ja", test: "ja-JP" },
                            { name: "msg/ko", test: "ko-KR" },
                            { name: "msg/nl", test: "nl-NL" },
                            { name: "msg/ru", test: "ru-RU" },
                            { name: "msg/zhcn", test: "zh-CN" },
                            { name: "msg/zhtw", test: "zh-TW" },
                        ],
                    },
                },
            },
        },
    };
});
