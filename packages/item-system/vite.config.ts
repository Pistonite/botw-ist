import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import yaml from "@modyfi/vite-plugin-yaml";

// import esbuildImportMetaUrlPlugin from "@codingame/esbuild-import-meta-url-plugin";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), yaml()],

    // optimizeDeps: {
    //     esbuildOptions: {
    //         plugins: [esbuildImportMetaUrlPlugin]
    //     }
    // }
})

