import { config } from "mono-dev/eslint";

export default config({
    ignores: ["dist"],
    tsconfigRootDir: import.meta.dirname,
});
