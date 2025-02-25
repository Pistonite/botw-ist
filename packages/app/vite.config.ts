import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import yaml from "@modyfi/vite-plugin-yaml";
import { spawnSync } from "child_process";
// import topLevelAwait from "vite-plugin-top-level-await";
import tsConfigPaths from "vite-tsconfig-paths";
import { serveStatic } from "vite-proxy-serve-static";

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
        react(),
        tsConfigPaths(),
        yaml(),
        serveStatic({
            routes: [
                {
                    route: "/runtime",
                    dir: "node_modules/skybook-runtime-wasm/dist",
                },
                {
                    route: "/static/item-assets/",
                    dir: "node_modules/botw-item-assets/public",
                },
                {
                    route: "/static/item-system/",
                    dir: "node_modules/skybook-item-system/public",
                },
            ],
        }),
    ],
    resolve: {
        dedupe: ["@pistonite/pure", "@griffel/react", "botw-item-assets"],
    },
    server: {
        port: 23172,
    },

    // optimizeDeps: {
    //     esbuildOptions: {
    //         plugins: [esbuildImportMetaUrlPlugin]
    //     }
    // }
});
