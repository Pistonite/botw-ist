/**
 * Patches TS Worker to expose getEncodedSemanticClassifcations
 * according to the POC here:
 * https://github.com/Pistonight/monaco-editor/commit/ac884678bc17c0eafe174a9cab84510f3b68b4ed
 */

import fs from "node:fs";

let lines: string[] = [];
let currentLine = 0;
let outLines: string[] = [];

const patchFile = (file: string, fn: () => void) => {
    const oldFile = file + ".old";
    if (fs.existsSync(oldFile)) {
        if (fs.existsSync(file)) {
            fs.rmSync(file);
        }
    } else if (fs.existsSync(file)) {
        fs.copyFileSync(file, oldFile);
    } else {
        throw new Error("File not found: " + file);
    }
    console.log("patching", file);
    lines = fs.readFileSync(oldFile, "utf-8").split("\n");
    currentLine = 0;
    outLines = [];
    fn();
    skipToEnd();
    fs.writeFileSync(file, outLines.join("\n"));
}
/**
 * skip from currentLine until a line matches a condition
 * update currentLine. Throws if not found.
 *
 * new current line will not be pushed, but skipped lines will
 */
const skipUntil = (matches: (line: string) => boolean) => {
    for (; currentLine< lines.length; currentLine++) {
        if (matches(lines[currentLine])) {
            return;
        }
        outLines.push(lines[currentLine]);
    }
    throw new Error("Not found");
}
const skipToEnd = () => {
    outLines.push(...lines.slice(currentLine));
    currentLine = lines.length;
}
const addPatch = (content: string) => {
    outLines.push(...content.split("\n").filter(Boolean).map(line => line.trimEnd()));
}

patchFile("monaco-editor-patch/esm/vs/language/typescript/ts.worker.js", () => {
    skipUntil(line => line.trim() === "// src/language/typescript/tsWorker.ts");
    skipUntil(line => line.trim().includes("class _TypeScriptWorker {"))
    skipUntil(line => line.trim().startsWith("async provideInlayHints("));
    // add our function here
    addPatch(`
  async getEncodedSemanticClassifications(fileName, start, end) {
    if (fileNameIsLib(fileName)) { return undefined };
    const span = { start, length: end - start };
    return this._languageService.getEncodedSemanticClassifications(fileName, span, "2020");
  }
`);
});

patchFile("monaco-editor-patch/monaco.d.ts", patchTypeScriptWorkerInterface);
patchFile("monaco-editor-patch/esm/vs/editor/editor.api.d.ts", patchTypeScriptWorkerInterface);

function patchTypeScriptWorkerInterface() {
    skipUntil(line => line.trim() === "export interface TypeScriptWorker {");
    skipUntil(line => line.trim().startsWith("provideInlayHints("));
    addPatch(`getEncodedSemanticClassifications(fileName: string, start: number, end: number): Promise<{spans: number[]}|undefined>;`);
}
