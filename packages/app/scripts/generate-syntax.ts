// Generate syntax files for editor extension based on skybook-parser
// we could use a rust parser but this is faster to implement

const TARGET_FILE = "src/extensions/editor/syntax.gen.ts";
const SOURCE_FILE = "../parser/src/syn/token.rs";
const HINT = "// @syntax-generator-hint";

/** Parse a section starting with the hint and ending with the "end" hint */
const parseHintSection = (lines: string[], hint: string): string[] => {
    console.log(`processing: ${hint}`);
    const output = [];
    const len = lines.length;
    let i = 0;
    const startHint = `${HINT} ${hint}`;
    const endHint = `${HINT} end`;
    for (; i < len; i++) {
        const l = lines[i];
        if (l.startsWith(startHint)) {
            i++;
            break;
        }
    }
    for (; i < len; i++) {
        const l = lines[i];
        if (l.startsWith(endHint)) {
            break;
        }
        // skip comments
        if (l.startsWith("/")) {
            continue;
        }
        const parts = l.split("=");
        if (parts.length !== 2) {
            continue;
        }
        let word = parts[1].trim();
        // clean up the line
        while (word.endsWith(",")) {
            word = word.substring(0, word.length - 1);
        }
        if (!word.startsWith('"') || !word.endsWith('"')) {
            console.warn(`warning: unknown syntax, line="${l}"`);
            continue;
        }
        word = word.substring(1, word.length - 1);
        if (word.trim() !== word) {
            console.warn(
                `warning: word has leading or trailing spaces, word="${word}"`,
            );
        }
        if (word.includes('"')) {
            console.error(`error: word cannot contain quote, word="${word}"`);
            process.exit(1);
        }
        output.push(word);
    }
    console.log(`done. found ${output.length} words`);
    return output;
};

const createExportString = (propStrings: string[]): string => {
    const middleLines = propStrings.join("\n");
    return `export const GenSyntax = {\n${middleLines}\n} as const;`;
};

console.log("parsing source file...");
const sourceFileLines = (await Bun.file(SOURCE_FILE).text())
    .split("\n")
    .map((x) => x.trim())
    .filter(Boolean);

/** Create a string containing the code `[prop]: [...words]` */
const createPropertyString = (
    prop: string,
    mapFn?: (x: string) => string,
): string => {
    mapFn ??= (x) => `"${x}"`;
    const words = parseHintSection(sourceFileLines, prop);
    const array = words.map(mapFn).join(", ");
    return `${prop}: [${array}],`;
};

const targetFileContent = createExportString([
    createPropertyString("commands"),
    createPropertyString("types"),
    createPropertyString("keywords"),
    createPropertyString("annotations", (x) => `":${x}"`),
]);

console.log("writing target file...");
await Bun.file(TARGET_FILE).write(targetFileContent);
console.log("done");
