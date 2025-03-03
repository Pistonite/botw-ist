declare const process: any;
const mode = process.argv[2];

// @ts-ignore
import fs from "node:fs/promises";

const ogDescription = `<meta name="og:description" content="The IST Simulator Manual">`;

if (mode === "pre") {
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
<meta name="og:title" content="{{ chapter_title }}">
${ogDescription}
<meta name="og:image" content="https://ist.pistonite.dev/favicon.png">
`;
    hbs = beforeHead + "<head>" + meta + afterHead;
    await fs.writeFile("theme/index.hbs", hbs);
} else if (mode === "post") {
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
        const meta = `
<meta name="og:url" content="https://ist.pistonite.dev/${file}">
`;

        // replace the title of the home page
        if (file === "welcome.html" || file === "index.html") {
            afterHead = afterHead.replace(/<title>[^<]*<\/title>/, "<title>IST Simulator Manual</title>");
            afterHead = afterHead.replace(/<meta name="og:title" content="[^"]*">/, `<meta name="og:title" content="Skybook - The IST Simulator">`);
        }

        html = beforeHead + "<head>" + meta + afterHead;
        await fs.writeFile(`book/${file}`, html);
    });
    await Promise.all(promises);
} else {
    console.error("Invalid mode");
    process.exit(1);
}

