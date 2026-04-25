/**
 * Language file formatting/editing tool
 *
 * Usage:
 *   mono-langtool <directory> <input file> [--no-confirm]
 *
 * Arguments:
 *   directory:
 *     The directory where the YAML language files are located.
 *     Must have an `en-US.yaml` file as the base.
 *     The language files should be formatted as:
 *       <key1>: <value1>
 *       <key2>: <value2>
 *
 *   input file:
 *     YAML file containing modifications to be applied to the language files.
 *     Should be formatted as:
 *       <lang1>:
 *         <key1>: <value1>
 *         <key2>: <value2>
 *       <lang2>:
 *         <key1>: <value1>
 *         <key2>: <value2>
 *     Use `-` for stdin
 *     If not provided, the program will format the files without changes
 *
 * Effect:
 *   The program will parse the base lang file (en-US) in the directory to
 *   get the structure of the language files. Keys are grouped into sections
 *   separated by empty lines. Comments are preserved. Keys in each section
 *   are sorted alphabetically.
 *
 *   Keys in the base file but not other files will be automatically copied to other files.
 *   Keys not in the base file but in other files will be prompted for deletion.
 *   When making edits, if old entry exists and is different from the base file
 *   (i.e. was not automatically copied), the user will be prompted to pick one.
 */
import { readdir, readFile, writeFile } from "node:fs/promises";
import YAML from "js-yaml";
import path from "node:path";
import readline from "node:readline";

const BASE_LANG = "en-US";
const BASE = `${BASE_LANG}.yaml`;
let inputFile = null;
let inputDirectory = null;
let noConfirm = false;
let position = 0;
for (const arg of process.argv.slice(2)) {
    if (arg === "--no-confirm") {
        noConfirm = true;
        continue;
    }
    switch (position) {
        case 0: {
            inputDirectory = arg;
            break;
        }
        case 1: {
            inputFile = arg;
            break;
        }
    }
    position++;
}
if (!inputDirectory) {
    console.error("Error: No input directory provided.");
    process.exit(1);
}

const main = async (inputDirectory, inputFile) => {
    const baseFile = path.join(inputDirectory, BASE);
    const baseLangContent = YAML.load(await readFile(baseFile, "utf-8"));
    // load other langs
    const otherLangs = (await readdir(inputDirectory))
        .filter((file) => file.endsWith(".yaml") && file !== BASE)
        .map((file) => file.substring(0, file.length - 5));
    const otherLangContent = await Promise.all(
        otherLangs.map(async (lang) => {
            return (
                YAML.load(await readFile(path.join(inputDirectory, `${lang}.yaml`), "utf-8")) ?? {}
            ); // empty object if file is empty
        }),
    );
    // make edits
    if (inputFile) {
        const langToContent = Object.fromEntries(
            otherLangs.map((lang, i) => [lang, otherLangContent[i]]),
        );
        let inputContent /*: Record<string, Record<string, string>>*/;
        if (inputFile === "-") {
            inputContent = YAML.load(await readFile(0, "utf-8"));
        } else {
            inputContent = YAML.load(await readFile(inputFile, "utf-8"));
        }
        for (const language in inputContent) {
            if (language === BASE_LANG) {
                continue;
            }
            const addData = inputContent[language];
            for (const addKey in addData) {
                const newValue = addData[addKey];
                if (
                    addKey in langToContent[language] &&
                    langToContent[language][addKey] !== newValue &&
                    langToContent[language][addKey] !== baseLangContent[addKey]
                ) {
                    await confirm(
                        "edit",
                        [
                            `Replace existing key: "${addKey}" in "${language}"?`,
                            `- Old value: "${langToContent[language][addKey]}"`,
                            `- New value: "${newValue}"`,
                        ],
                        "",
                    );
                }
                langToContent[language][addKey] = addData[addKey];
                console.log(`Added key: "${addKey}" in "${language}"`);
            }
        }
    }

    // delete keys
    for (let i = 0; i < otherLangs.length; i++) {
        const language = otherLangs[i];
        if (language === BASE_LANG) {
            continue;
        }
        const langContent = otherLangContent[i];
        const toDelete = new Set();
        for (const key in langContent) {
            if (!(key in baseLangContent)) {
                await confirm("delete", [`Delete key: "${key}" in "${language}"?`], "");
                toDelete.add(key);
            }
        }
        for (const key of toDelete) {
            delete langContent[key];
            console.log(`Deleted key: "${key}" in "${language}"`);
        }
    }

    console.log("Saving changes...");

    const baseLangFile = await parseLangFile(baseFile);
    await saveLangFile(baseFile, baseLangFile);
    for (let i = 0; i < otherLangs.length; i++) {
        const newFile = makeLangFile(baseLangFile, otherLangContent[i]);
        await saveLangFile(path.join(inputDirectory, `${otherLangs[i]}.yaml`), newFile);
    }
};

const noConfirmTypes = new Set();
const confirm = async (type, pre, message) => {
    if (noConfirm) {
        return;
    }
    if (noConfirmTypes.has(type)) {
        return;
    }

    if (typeof pre === "string") {
        console.log(pre);
    } else if (Array.isArray(pre)) {
        console.log(pre.join("\n"));
    }

    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    while (true) {
        const ok = await new Promise((resolve) => {
            rl.question(`${message} [Y]es [N]o [A]ll: `, (answer) => {
                rl.close();
                switch (answer.toLowerCase()) {
                    case "y": {
                        resolve(true);
                        break;
                    }
                    case "n": {
                        rl.close();
                        process.exit(102);
                    }
                    case "a": {
                        noConfirmTypes.add(type);
                        resolve(true);
                        break;
                    }
                    default:
                        resolve(false);
                }
            });
        });
        if (ok) {
            rl.close();
            return;
        }
        console.log("Invalid input. Please try again.");
    }
};

// type LangFile = LangBlock[];
// type LangBlock = {
//     before: string[];
//     entries: Record<string, string>;
// };
const parseLangFile = async (file /*: string*/) /*: Promise<LangFile>*/ => {
    const content = (await readFile(file, "utf-8")).split("\n");
    const blocks = [];
    let currentBlockEntryLines = [];
    let currentBlockLines = [];
    const addAndResetCurrentBlock = () => {
        if (currentBlockLines.length || currentBlockEntryLines.length) {
            const entryContent = currentBlockEntryLines.join("\n");
            blocks.push({
                before: currentBlockLines,
                entries: YAML.load(entryContent),
            });
        }
        currentBlockLines = [];
        currentBlockEntryLines = [];
    };
    for (const line of content) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith("#")) {
            if (currentBlockEntryLines.length) {
                addAndResetCurrentBlock();
            }
            currentBlockLines.push(line);
            continue;
        }
        currentBlockEntryLines.push(line);
    }
    if (currentBlockEntryLines.length) {
        addAndResetCurrentBlock();
    }
    return blocks;
};
/** Make sure the content and blocks are the same in the lang file to fix */
const makeLangFile = (basedOnLang /*: LangFile*/, content /*: Record<string, string>*/) => {
    const newBlocks /*: LangFile*/ = [];
    for (const block of basedOnLang) {
        const newBlock /*: LangBlock*/ = {
            before: [...block.before],
            entries: {},
        };
        Object.keys(block.entries).forEach((key) => {
            if (content[key]) {
                newBlock.entries[key] = content[key];
                return;
            }
            newBlock.entries[key] = block.entries[key];
        });
        newBlocks.push(newBlock);
    }
    return newBlocks;
};

const saveLangFile = async (file /*: string*/, content /*: LangFile*/) => {
    const lines /*: string[]*/ = [];
    for (const block of content) {
        lines.push(...block.before);
        const entries = Object.entries(block.entries);
        entries.sort(([a], [b]) => a.localeCompare(b));
        for (const [key, value] of entries) {
            lines.push(`${key}: ${JSON.stringify(value)}`);
        }
    }
    await writeFile(file, lines.join("\n"));
};

void main(inputDirectory, inputFile, noConfirm);
