const TARGET_PATH = "src/generated";
const SOURCE_FILE = "../parser/src/cir/enum_name.rs";
const HINT = "// @manual-generator-hint";

/** Parse a section starting with the hint and ending with the "end" hint */
const parseHintSection = (lines: string[], hint: string): string[] => {
    console.log(`processing: ${hint}`);
    const output: string[] = [];
    const len = lines.length;
    let i = 0;
    const startHint = `${HINT} ${hint}`;
    const endHint = `${HINT} end`;
    const valueHint = `${HINT} values`;
    // find start hint
    for (; i < len; i++) {
        const l = lines[i];
        if (l.startsWith(startHint)) {
            i++;
            break;
        }
    }
    // parse header
    for (; i < len; i++) {
        const l = lines[i];
        if (l.startsWith(endHint)) {
            i++;
            break;
        }
        if (l.startsWith(valueHint)) {
            i++;
            break;
        }
        let x = l;
        if (l.startsWith("//")) {
            x = x.substring(2).trim();
        }
        output.push(x);
    }
    output.push("| Constant | Description | Internal Value |");
    output.push("|-|-|-|");
    // parse each section (row of the table)
    const sections: string[] = [];
    for (; i < len; i++) {
        // each section starts with comments
        const comments: string[] = [];
        for (; i < len; i++) {
            const l = lines[i];
            if (l.startsWith(endHint)) {
                break;
            }
            if (l.startsWith("/")) {
                comments.push(l);
                continue;
            }
            break;
        }
        // then, lines that don't start with =>
        const variants: string[] = [];
        for (; i < len; i++) {
            const l = lines[i];
            if (l.startsWith(endHint)) {
                break;
            }
            if (l.startsWith("=>")) {
                break;
            }
            const parts = l.split("|").map((x) => x.trim()).filter(Boolean);
            parts.forEach((x) => {
                if (!x.startsWith('"') || !x.endsWith('"')) {
                    console.warn(`warning: unquoted variant syntax, line="${l}"`);
                    return;
                }
                x = x.substring(1, x.length - 1);
                variants.push(x)
            });
        }
        // should be the => line
        const l = lines[i];
        if (l.startsWith(endHint)) {
            break;
        }
        if (!l.startsWith("=>")) {
            console.error("did not find the => line!");
            process.exit(1);
        }
        const value = l.substring(2).trim();

        // table: Name, Description, Value
        //
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

const header = sourceFileLines
    .filter((x) => x.startsWith("//!"))
    .map((x) => x.substring(3).trim());

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

const targetFileContent = header.join("\n") + "\n" + createExportString([
    createPropertyString("commands"),
    createPropertyString("types"),
    createPropertyString("keywords"),
    createPropertyString("annotations", (x) => `":${x}"`),
]);

console.log("writing target file...");
await Bun.file(TARGET_FILE).write(targetFileContent);
console.log("done");
