// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import { spawnSync } from "child_process";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import path from "path";

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import yaml from "@modyfi/vite-plugin-yaml";
// import topLevelAwait from "vite-plugin-top-level-await";
import tsConfigPaths from "vite-tsconfig-paths";
import serveStatic from "vite-plugin-serve-static";
import intwc from "@pistonite/vite-plugin-intwc";

const commit = spawnSync("git", ["rev-parse", "HEAD"], {
    encoding: "utf-8",
}).stdout.trim();
console.log(commit);

// import esbuildImportMetaUrlPlugin from "@codingame/esbuild-import-meta-url-plugin";

// https://vitejs.dev/config/
export default defineConfig({
    define: {
        "import.meta.env.COMMIT": JSON.stringify(commit),
    },
    plugins: [
        intwc({
            basicLanguages: ["typescript"],
            typescript: true,
        }),
        react(),
        tsConfigPaths(),
        yaml(),
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
        dedupe: ["@pistonite/pure", "@griffel/react", "botw-item-assets"],
    },
    server: {
        port: 23172,
        headers: {
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin",
        },
    },
    build: {
        chunkSizeWarningLimit: 4096,
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
                    react: ["react", "react-dom", "@fluentui/react-components"],
                    strings: ["skybook-localization"],
                },
            },
        },
    },

    // optimizeDeps: {
    // esbuildOptions: {
    //     plugins: [esbuildImportMetaUrlPlugin]
    // }
    // }
});
