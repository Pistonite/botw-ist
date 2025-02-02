import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import yaml from "@modyfi/vite-plugin-yaml";
// import topLevelAwait from "vite-plugin-top-level-await";
import tsConfigPaths from "vite-tsconfig-paths";
import { serveStatic } from "vite-proxy-serve-static";

// import esbuildImportMetaUrlPlugin from "@codingame/esbuild-import-meta-url-plugin";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        react(),
        tsConfigPaths(),
        yaml(),
        serveStatic({
            routes: [
                {
                    route: "/runtime",
                    dir: "node_modules/skybook-runtime-wasm/dist",
                }, // Serving images from the public/images directory
            ],
        }),
    ],
    resolve: {
        dedupe: ["@pistonite/pure", "@griffel/react"],
    },

    // optimizeDeps: {
    //     esbuildOptions: {
    //         plugins: [esbuildImportMetaUrlPlugin]
    //     }
    // }
});
