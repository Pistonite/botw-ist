/* eslint-disable @typescript-eslint/no-explicit-any */
import fs from "node:fs";

const ROOT_PACKAGE = "../../package.json";
let rootPackageJson: Record<string, any> = {};
let needFix = true;
try {
    rootPackageJson = JSON.parse(fs.readFileSync(ROOT_PACKAGE, "utf-8"));
} catch {
    console.log("No root package.json found, skipping pre-patch");
    needFix = false;
}
if (needFix && !rootPackageJson.pnpm) {
    console.log("No `pnpm` key in root package.json, skipping pre-patch");
    needFix = false;
}
if (needFix && !rootPackageJson.pnpm.patchedDependencies) {
    console.log("No `patchedDependencies` key in root package.json, skipping pre-patch");
    needFix = false;
}

let modified = false;
if (needFix) {
    const patchedDependencies = rootPackageJson.pnpm.patchedDependencies;
    for (const key in patchedDependencies) {
        if (key.startsWith("monaco-editor@")) {
            console.log("Removing", key);
            // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
            delete patchedDependencies[key];
            modified = true;
        }
    }
}

if (modified) {
    console.log("Writing root package.json");
    fs.writeFileSync(ROOT_PACKAGE, JSON.stringify(rootPackageJson, null, 4));
} else {
    console.log("No changes needed");
}
