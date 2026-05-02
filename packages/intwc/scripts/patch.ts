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
    lines = fs.readFileSync(file, "utf-8").split("\n");
    currentLine = 0;
    outLines = [];
    fn();
    skipToEnd();
    fs.writeFileSync(file, outLines.join("\n"));
};
/**
 * skip from currentLine until a line matches a condition
 * update currentLine. Throws if not found.
 *
 * new current line will not be pushed, but skipped lines will
 */
const skipUntil = (matches: (line: string) => boolean) => {
    for (; currentLine < lines.length; currentLine++) {
        if (matches(lines[currentLine])) {
            return;
        }
        outLines.push(lines[currentLine]);
    }
    throw new Error("Not found");
};
const skipOne = () => {
    outLines.push(lines[currentLine]);
    currentLine++;
};
const skipToEnd = () => {
    outLines.push(...lines.slice(currentLine));
    currentLine = lines.length;
};
const addPatch = (content: string) => {
    outLines.push(
        ...content
            .split("\n")
            .filter(Boolean)
            .map((line) => line.trimEnd()),
    );
};
const addPatchInlineBefore = (content: string, before: string) => {
    const line = lines[currentLine];
    const index = line.indexOf(before);
    if (index === -1) {
        throw new Error("`before` not found in addPatchInlineBefore");
    }
    lines[currentLine] = line.substring(0, index) + content + line.substring(index);
};

const patchTsWorker = (mode: "esm" | "dev" | "min") => {
    if (mode === "esm") {
        skipUntil((line) => line.trim() === "// src/language/typescript/tsWorker.ts");
        skipUntil((line) => line.trim().includes("class _TypeScriptWorker {"));
    }
    if (mode !== "min") {
        skipUntil((line) => line.trim().startsWith("async provideInlayHints("));
    } else {
        skipUntil((line) => line.includes("async updateExtraLibs("));
    }
    let fileNameIsLibFnName = "fileNameIsLib";
    if (mode === "min") {
        const oldLine = lines[currentLine];
        const provideInlayHintsIdx = oldLine.indexOf("async provideInlayHints(");
        if (provideInlayHintsIdx === -1) {
            throw new Error("provideInlayHints not found");
        }
        const afterProvideInlayHintsPart = oldLine.substring(provideInlayHintsIdx);
        const ifIdx = afterProvideInlayHintsPart.indexOf("){if(");
        if (ifIdx === -1) {
            throw new Error("if not found inside provideInlayHints");
        }
        const afterIfPart = afterProvideInlayHintsPart.substring(ifIdx + 5);
        const closeIdx = afterIfPart.indexOf("(");
        if (closeIdx === -1) {
            throw new Error(") not found after if");
        }
        fileNameIsLibFnName = afterIfPart.substring(0, closeIdx);
        console.log("found minified name of fileNameIsLib=", fileNameIsLibFnName);
    }

    const patchContent = `
  async getEncodedSemanticClassifications(fileName, start, end) {
    if (${fileNameIsLibFnName}(fileName)) { return undefined };
    const span = { start, length: end - start };
    return this._languageService.getEncodedSemanticClassifications(fileName, span, "2020");
  }
`;

    if (mode === "min") {
        addPatchInlineBefore(patchContent, "async updateExtraLibs(");
        skipOne();
    } else {
        addPatch(patchContent);
    }
};

const patchTypeScriptWorkerInterface = () => {
    skipUntil((line) => line.trim() === "export interface TypeScriptWorker {");
    skipUntil((line) => line.trim().startsWith("provideInlayHints("));
    addPatch(
        `getEncodedSemanticClassifications(fileName: string, start: number, end: number): Promise<{spans: number[]}|undefined>;`,
    );
};

patchFile("monaco-editor-patch/esm/vs/language/typescript/ts.worker.js", () =>
    patchTsWorker("esm"),
);
patchFile("monaco-editor-patch/dev/vs/language/typescript/tsWorker.js", () => patchTsWorker("dev"));
patchFile("monaco-editor-patch/min/vs/language/typescript/tsWorker.js", () => patchTsWorker("min"));

patchFile("monaco-editor-patch/monaco.d.ts", patchTypeScriptWorkerInterface);
patchFile("monaco-editor-patch/esm/vs/editor/editor.api.d.ts", patchTypeScriptWorkerInterface);
