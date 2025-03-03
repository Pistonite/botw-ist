import { config } from "mono-dev/eslint";

export default config({
    ignores: ["src/interfaces", "src/sides"],
    tsconfigRootDir: import.meta.dirname,
});
