import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// import esbuildImportMetaUrlPlugin from "@codingame/esbuild-import-meta-url-plugin";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],

    // optimizeDeps: {
    //     esbuildOptions: {
    //         plugins: [esbuildImportMetaUrlPlugin]
    //     }
    // }
})
