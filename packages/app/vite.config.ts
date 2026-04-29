/// <reference types="mono-dev/vitest" />
/// <reference types="node" />

import { spawnSync } from "node:child_process";
import path from "node:path";
import fs from "node:fs";

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

// https://vitejs.dev/config/
export default configure(() => {
    const commit = spawnSync("git", ["rev-parse", "HEAD"], {
        encoding: "utf-8",
    }).stdout.trim();
    console.log(`commit: ${commit}`);

    const packageJson = JSON.parse(fs.readFileSync("../../package.json", "utf-8"));
    const version = packageJson.version;
    console.log(`version: ${version}`);

    return {
        define: {
            "import.meta.env.COMMIT": JSON.stringify(commit),
            "import.meta.env.VERSION": JSON.stringify(version),
            "import.meta.vitest": "undefined",
        },
        optimizeDeps: {
            // exclude the ones that requires building
            exclude: [
                "@pistonite/skybook-itemsys",
                "@pistonite/skybook-api",
                "skybook-runtime-worker",
                "skybook-localization",
            ],
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
                    pattern: /^\/static\/itemsys\/(.*)/,
                    resolve: ([_, capture]) => path.join("..", "itemsys-build", "public", capture),
                },
            ]),
        ],
        resolve: {
          dedupe: [
                "react",
    "react-dom",
    "@fluentui/react-components",
    "@fluentui/react-icons",
            ],
        },
        server: {
            port: 23172,
            hmr: {
                protocol: "wss"
            },
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
            },
        },
        test: {
            includeSource: ["src/**/*.{js,ts}"],
            environment: "jsdom",
        },
    };
});
