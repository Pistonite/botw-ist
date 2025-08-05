// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import path from "path";

import { defineConfig } from "vite";
import serveStatic from "vite-plugin-serve-static";
import monodev from "mono-dev/vite";

const monodevConfig = monodev({});

export default defineConfig(
    monodevConfig({
        server: {
            port: 33173,
        },
        plugins: [
            serveStatic([
                {
                    pattern: /^\/static\/item-assets\/(.*)/,
                    resolve: ([_, capture]) => path.join("..", "item-assets", "public", capture),
                },
            ]),
        ],
    }),
);
