import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import yaml from "@modyfi/vite-plugin-yaml";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// import esbuildImportMetaUrlPlugin from "@codingame/esbuild-import-meta-url-plugin";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), yaml(), wasm(), topLevelAwait()],
    worker: {
        plugins: () => [yaml(), 
            wasm(), topLevelAwait()
        ],
        format: "iife",

    },
    resolve: {
        dedupe: ["@pistonite/pure"]
    }


    // optimizeDeps: {
    //     esbuildOptions: {
    //         plugins: [esbuildImportMetaUrlPlugin]
    //     }
    // }
})

