// @ts-ignore
import fs from "node:fs";
// @ts-ignore
import subprocess from "node:child_process";
import YAML from "js-yaml";

const LANG_TO_CHECK = ["en-US"];

// Check completeness of some lang entries using TypeScript
const TSCONFIG = {
    extends: "../../../../mono-dev/tsconfig/browser.json",
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
    const parserErrorKeyInLang: string[] = [];
    for (const key in langData) {
        if (key.startsWith("parser.")) {
            const parserError = key.split(".")[1];
            parserErrorKeyInLang.push(parserError);
        }
    }
    const outFile = `${SRC_DIR}/parser_${lang}.ts`;
    const toSwitchCase = (key: string) => `case "${key}": return true;`;
    const outContent = `
import type { ParserError } from "@pistonite/skybook-api";
function checkParserErrorKey(key: ParserError): boolean {
switch (key.type) {
case "Unexpected": return true; // this is handled as a generic error
${parserErrorKeyInLang.map(toSwitchCase).join("\n")}
}
console.log(key); // Check the type of key with LS to see what's missing
}
`;
    fs.writeFileSync(outFile, outContent);
});

const result = subprocess.spawnSync("pnpm", ["exec", "tsc", "-p", DIR], {
    stdio: "inherit",
    encoding: "utf8",
});
if (result.status !== 0) {
    console.error("Lang entry check failed!!!");
    console.error("Please ensure these keys are defined in lang files:");
    console.error("  - One `parser.<ParserError>` for each ParserError type");
    // @ts-ignore
    process.exit(2);
}
