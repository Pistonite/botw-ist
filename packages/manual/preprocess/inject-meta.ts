declare const process: any;
const mode = process.argv[2];

// @ts-ignore
import fs from "node:fs/promises";


async function preProcess() {
    let hbs = await fs.readFile("theme/index.hbs", "utf8");
    hbs = hbs.replace(
        `<meta name="theme-color" content="#ffffff">`,
        `<meta name="theme-color" content="#fe8f00">`,
    );
    const headIdx = hbs.indexOf("<head>");
    const beforeHead = hbs.substring(0, headIdx);
    const afterHead = hbs.substring(headIdx + 6);
    const meta = `
<meta name="og:site_name" content="Skybook">
<meta name="og:type" content="website">
<meta name="og:description" content="The IST Simulator Manual">
<meta name="og:image" content="https://ist.pistonite.dev/favicon.png">
<meta name="og:title" content="{{ chatper_title }}">
`;
    hbs = beforeHead + "<head>" + meta + afterHead;
    await fs.writeFile("theme/index.hbs", hbs);
}

async function postProcess() {
    const pathToTitle = await parseToc();
    const files: string[] = await fs.readdir("book", { recursive: true });
    const promises = files.map(async (file: string) => {
        if (!file.endsWith(".html")) {
            return;
        }
        if (file.match(/[/\\]toc.html$/)) {
            return;
        }
        let html = await fs.readFile(`book/${file}`, "utf8");
        const headIdx = html.indexOf("<head>");
        const beforeHead = html.substring(0, headIdx);
        let afterHead = html.substring(headIdx + 6);
        const meta = `<meta name="og:url" content="https://ist.pistonite.dev/${file}">`;

        if (file === "welcome.html" || file === "index.html") {
            // replace the tlte and description of the home page
            afterHead = afterHead.replace(/<title>[^<]*<\/title>/, "<title>The IST Simulator Manual</title>");
            afterHead = afterHead.replace(/<meta name="og:description".*>/, "");
            afterHead = afterHead.replace(/<meta name="og:title" content="[^"]*">/, `<meta name="og:title" content="The IST Simulator Manual">`);
        } else {
            // add parent title to the meta tags
            const title = pathToTitle.get(file.replace(/\\/g, "/"));
            if (title) {
                afterHead = afterHead.replace(/<meta name="og:title" content="[^"]*">/, `<meta name="og:title" content="${title}">`);
            }
        }

        html = beforeHead + "<head>" + meta + afterHead;
        await fs.writeFile(`book/${file}`, html);
    });
    await Promise.all(promises);
}

async function parseToc(): Promise<Map<string, string>> {
    const lines = (await fs.readFile("src/SUMMARY.md", "utf8"))
        .split("\n")
        .map((line: string) => line.trimEnd())
        .filter((x: string) => x.trim().startsWith("-"));
    const pathToTitle = new Map<string, string>();
    const stack = [["",0]];

    const getParent = () => {
        if (stack.length === 0) {
            return "";
        }
        return `${stack[stack.length - 1][0]} &gt; `;
    };
    const getLevel = () => {
        if (stack.length === 0) {
            return 0;
        }
        return stack[stack.length - 1][1];
    }

    const parseLine = (line: string) => {
        const [part1, part2] = line.split("](");
        const nameStart = part1.indexOf("[") + 1;
        const name = part1.substring(nameStart);
        let path = part2.substring(0, part2.length - 1);
        if (path.endsWith(".md")) {
            path = path.substring(0, path.length - 3) + ".html";
        }
        if (path.startsWith("./")) {
            path = path.substring(2);
        }
        return [name, path];
    }

    for (const line of lines) {
        const currLevel = getLevel();
        const nextLevel = line.indexOf("-");
        if (nextLevel === currLevel) {
            stack.pop();
            const [name, path] = parseLine(line);
            if (path) {
                const parent = getParent();
                pathToTitle.set(path, parent + name);
            }
            stack.push([name, nextLevel]);
        } else if (nextLevel > currLevel) {
            const [name, path] = parseLine(line);
            if (path) {
                const parent = getParent();
                pathToTitle.set(path, parent + name);
            }
            stack.push([name, nextLevel]);
        } else if (nextLevel < currLevel) {
            let level = currLevel;
            while (level > nextLevel) {
                stack.pop();
                level = getLevel();
            }
            stack.pop();
            const [name, path] = parseLine(line);
            if (path) {
                const parent = getParent();
                pathToTitle.set(path, parent + name);
            }
            stack.push([name, nextLevel]);
        }
    }

    return pathToTitle;
}

if (mode === "pre") {
    void preProcess();
} else if (mode === "post") {
    void postProcess();
} else {
    console.error("Invalid mode");
    process.exit(1);
}

