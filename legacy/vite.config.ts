import path from "path";
import fs from "fs";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import yaml from "@modyfi/vite-plugin-yaml";

const createHttpsConfig = () => {
    try {
        const key = path.join(__dirname, "cert/cert-key.pem");
        const cert = path.join(__dirname, "cert/cert.pem");
        if (fs.existsSync(key) && fs.existsSync(cert)) {
            return { key, cert };
        }
    } catch (e) {}
    return undefined;
};

const https = createHttpsConfig();

export default defineConfig({
    plugins: [react(), tsconfigPaths(), yaml()],
    server: { https },
    base: "/legacy/",
});
