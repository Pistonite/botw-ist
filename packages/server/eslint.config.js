import { config } from "mono-dev/eslint";

export default config({
    react: false,
    ignores: ["dist"],
    tsconfigRootDir: import.meta.dirname,
});
