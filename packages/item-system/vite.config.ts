import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import yaml from "@modyfi/vite-plugin-yaml";

export default defineConfig({
    plugins: [react(), yaml()],
    server: {
        port: 33173,
    },
});
