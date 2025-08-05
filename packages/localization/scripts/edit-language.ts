import { readdir } from "node:fs/promises";
import YAML from "js-yaml";

async function main() {
    // load en-US as base
    const enUsFile = await parseLangFile("src/ui/en-US.yaml");
    // load other langs
    const otherLangs = (await readdir("src/ui"))
        .filter((file) => file.endsWith(".yaml") && file !== "en-US.yaml")
        .map((file) => file.substring(0, file.length - 5));
    const otherLangContent = await Promise.all(
        otherLangs.map(async (lang) => {
            return YAML.load(await Bun.file(`src/ui/${lang}.yaml`).text()) as Record<
                string,
                string
            >;
        }),
    );
    const [_execPath, _scriptPath, inputFile] = process.argv;
    if (inputFile) {
        const langToContent = Object.fromEntries(
            otherLangs.map((lang, i) => [lang, otherLangContent[i]]),
        );
        let inputContent: Record<string, Record<string, string>>;
        if (inputFile === "-") {
            inputContent = YAML.load(await Bun.stdin.text()) as Record<
                string,
                Record<string, string>
            >;
        } else {
            inputContent = YAML.load(await Bun.file(inputFile).text()) as Record<
                string,
                Record<string, string>
            >;
        }
        for (const language in inputContent) {
            if (language === "en-US") {
                continue;
            }
            console.log(`-- language: ${language}`);
            const addData = inputContent[language];
            for (const addKey in addData) {
                console.log(`---- key: ${addKey}`);
                langToContent[language][addKey] = addData[addKey];
            }
        }
    }

    const otherLangFiles: LangFile[] = [];
    for (let i = 0; i < otherLangs.length; i++) {
        const newContent = makeLangFile(enUsFile, otherLangContent[i]);
        otherLangFiles.push(newContent);
    }

    console.log("Saving files...");
    await saveLangFile("src/ui/en-US.yaml", enUsFile);
    for (let i = 0; i < otherLangs.length; i++) {
        await saveLangFile(`src/ui/${otherLangs[i]}.yaml`, otherLangFiles[i]);
    }
}

type LangFile = LangBlock[];
type LangBlock = {
    before: string[];
    entries: Record<string, string>;
};
const parseLangFile = async (file: string): Promise<LangFile> => {
    console.log(`Parsing lang file: ${file}`);
    const content = (await Bun.file(file).text()).split("\n");
    const blocks: LangBlock[] = [];
    let currentBlockEntryLines: string[] = [];
    let currentBlockLines: string[] = [];
    const addAndResetCurrentBlock = () => {
        if (currentBlockLines.length || currentBlockEntryLines.length) {
            const entryContent = currentBlockEntryLines.join("\n");
            blocks.push({
                before: currentBlockLines,
                entries: YAML.load(entryContent) as Record<string, string>,
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
const makeLangFile = (basedOnLang: LangFile, content: Record<string, string>) => {
    const newBlocks: LangFile = [];
    for (const block of basedOnLang) {
        const newBlock: LangBlock = {
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

const saveLangFile = async (file: string, content: LangFile) => {
    const lines: string[] = [];
    for (const block of content) {
        lines.push(...block.before);
        const entries = Object.entries(block.entries);
        entries.sort(([a], [b]) => a.localeCompare(b));
        for (const [key, value] of entries) {
            lines.push(`${key}: ${JSON.stringify(value)}`);
        }
    }
    await Bun.file(file).write(lines.join("\n"));
};

void main();
