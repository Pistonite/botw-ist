import { defineConfig } from "vite";
import monodev from "mono-dev/vite";

const monodevConfig = monodev({});

export default defineConfig(
    monodevConfig({
        server: {
            port: 33173,
        },
    }),
);
