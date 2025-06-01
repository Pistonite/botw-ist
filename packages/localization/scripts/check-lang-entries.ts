import fs from "node:fs";
import subprocess from "node:child_process";
import YAML from "js-yaml";

// just checking en-US is fine, since missing keys are duplicated to other language files
// automatically
const LANG_TO_CHECK = ["en-US"];

// Check completeness of some lang entries using TypeScript
const TSCONFIG = {
    extends: "../../../../mono-dev/toolsets/mono-lint/default-tsconfig.json",
    compilerOptions: {
        lib: ["esnext", "dom"],
    },
    include: ["src"],
};
const DIR = "node_modules/.cache/check-lang-entries";
const SRC_DIR = `${DIR}/src`;
if (!fs.existsSync(SRC_DIR)) {
    fs.mkdirSync(SRC_DIR, { recursive: true });
}
fs.writeFileSync(`${DIR}/tsconfig.json`, JSON.stringify(TSCONFIG, null, 2));

LANG_TO_CHECK.forEach((lang) => {
    const langFile = `src/ui/${lang}.yaml`;
    const langData = YAML.load(fs.readFileSync(langFile, "utf8"));

    const generateCheckerFile = (
        keysPrefix: string,
        checkType: string,
        importFrom: string,
        ignore: string[],
    ) => {
        const keysInLang: string[] = [];

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        for (const key in langData as any) {
            if (key.startsWith(keysPrefix)) {
                keysInLang.push(key.split(".")[1]);
            }
        }

        return `
import type { ${checkType} } from "${importFrom}";
function __if_you_see_this_add_missing_translation_for_${checkType}(key: ${checkType}): boolean {
switch (key.type) {
${ignore.map(toSwitchCase).join("\n")}
${keysInLang.map(toSwitchCase).join("\n")}
}
console.log(key); // Check the type of key with LS to see what's missing
}
`;
    };
    const toSwitchCase = (key: string) => `case "${key}": return true;`;
    const makeOutFilePath = (prefix: string) =>
        `${SRC_DIR}/${prefix}_${lang}.ts`;

    fs.writeFileSync(
        makeOutFilePath("parser"),
        generateCheckerFile(
            "parser.",
            "ParserError",
            "@pistonite/skybook-api",
            ["Unexpected"],
        ),
    );
    fs.writeFileSync(
        makeOutFilePath("runtime_init"),
        generateCheckerFile(
            "runtime_init.",
            "RuntimeWorkerInitError",
            "@pistonite/skybook-api",
            [],
        ),
    );
});

const result = subprocess.spawnSync("pnpm", ["exec", "tsc", "-p", DIR], {
    stdio: "inherit",
    encoding: "utf8",
});
if (result.status !== 0) {
    console.error("Lang entry check failed!!!");
    console.error("Please ensure these keys are defined in lang files:");
    console.error("  - One `parser.<ParserError>` for each ParserError type");
    console.error("  - One `worker.<WorkerError>` for each WorkerError type");
    process.exit(2);
}
