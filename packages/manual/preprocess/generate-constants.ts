const TARGET_FILE = "src/generated/constants.md";
const SOURCE_FILE = "../parser/src/cir/enum_name.rs";
const HINT = "// @manual-generator-hint";

/** 
 * Parse a section starting with the hint and ending with the "end" hint
 * into a markdown section
 */
const parseHintSection = (lines: string[], hint: string): string => {
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
    output.push(`<div class="skybook--wide-table">\n`);
    output.push("| Constant | Description |");
    output.push("|-|-|");
    // parse each section (row of the table)
    const tableRowByName: Record<string, string> = {};
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
        let value = l.substring(2).trim();
        if (value.endsWith(",")) {
            value = value.substring(0, value.length - 1);
        }
        const description = comments.map((x) =>{
            x = x.trim();
            if (x.startsWith("//")) {
                x = x.substring(2).trim();
            }
            return x
        }).join("<br>");

        const vLen = variants.length;
        const first = variants[0];
        const firstLine = "| `"+first+"` | " + description + "<br><br>Internal Value: `"+value+"`";
        tableRowByName[first] = firstLine;

        // The first one is the main one, the rest
        // are aliases to it
        for (let i = 1;i<vLen;i++) {
            const v = variants[i];
            const line = "| `"+v+"` | Alias for `"+first+"` |";
            tableRowByName[v] = line;
        }
    }

    const keys = Object.keys(tableRowByName);
    keys.sort();
    for (const k of keys) {
        output.push(tableRowByName[k]);
    }

    output.push("\n</div>");
    return "\n" + output.join("\n") + "\n";
};

console.log("parsing source file...");
const sourceFileLines = (await Bun.file(SOURCE_FILE).text())
    .split("\n")
    .map((x) => x.trim())
    .filter(Boolean);

const header = sourceFileLines
    .filter((x) => x.startsWith("//!"))
    .map((x) => x.substring(3).trim());

const targetFileContent = 
    header.join("\n") + 
    parseHintSection(sourceFileLines, "cook-effects") +
    parseHintSection(sourceFileLines, "weapon-modifiers");


console.log("writing target file...");
await Bun.file(TARGET_FILE).write(targetFileContent);
console.log("done");
