// @ts-ignore
import fs from "node:fs";

const ROOT_PACKAGE = "../../package.json";
let rootPackageJson: Record<string, any> = {};
try {
rootPackageJson = JSON.parse(fs.readFileSync(ROOT_PACKAGE, "utf-8"));
} catch{
    console.log("No root package.json found, will create one");
}
if (!rootPackageJson.pnpm) {
    rootPackageJson.pnpm = {};
}
if (!rootPackageJson.pnpm.patchedDependencies) {
    rootPackageJson.pnpm.patchedDependencies = {};
}
const packageJson = JSON.parse(fs.readFileSync("package.json", "utf-8"));
const monacoEditorVersion = packageJson.dependencies["monaco-editor"];
const patchedDependencies = rootPackageJson.pnpm.patchedDependencies;

let found = false

for (const key in patchedDependencies) {
    if (key === `monaco-editor@${monacoEditorVersion}`) {
        found = true;
        const file = patchedDependencies[key];
        patchedDependencies[key] = "packages/intwc/" + file;
        console.log("Patched", key, "to", patchedDependencies[key]);
        continue;
    }
    if (key.startsWith("monaco-editor@")) {
        console.log("Removing", key);
        delete patchedDependencies[key];
    }
}

if (!found) {
    console.log("Adding patch");
    patchedDependencies[`monaco-editor@${monacoEditorVersion}`] = `packages/intwc/patches/monaco-editor@${monacoEditorVersion}.patch`;
}

fs.writeFileSync(ROOT_PACKAGE, JSON.stringify(rootPackageJson, null, 4));
