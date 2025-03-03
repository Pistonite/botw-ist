import { DirectLoad } from "client";
import { ResponsePayload, useAcceptLanguage } from "framework";

import Strings from "strings.json" with { type: "json" };

const languages = Object.keys(Strings.title);

/** Server side rendering utilities */
console.log("loading entry point for server-side rendering");
const loadEntryPointHtml = async (): Promise<[string, string]> => {
    const indexHtml = await Bun.file("app/index.html").text();
    const headStartIndex = indexHtml.indexOf("<head>");
    if (headStartIndex == -1) {
        throw new Error("No <head> tag found in index.html");
    }
    const i = headStartIndex + "<head>".length;
    return [indexHtml.substring(0, i), indexHtml.substring(i)];
}
const [htmlHead, htmlTail] = await loadEntryPointHtml();

export type SSROptions = {
    /** Direct load payload to inject into the page */
    directLoad?: DirectLoad,
    /** File reference to appear in the meta tags */
    file?: {
        /** File name */
        name: string,
        /** Short version of the file name */
        short?: string,
    },
};

export const makeSSR = (req: Request, options: SSROptions): ResponsePayload => {

    const language = useAcceptLanguage(req, languages, "en-US");
    const title = `<title>${Strings.title[language]}</title>`;
    let longTitle = Strings.title[language];
    if (options.file) {
        longTitle += ` - ${options.file.name}`;
    }
    const ogtitle = `<meta name="og:title" content="${longTitle}">`;

    let content 
    if (options.directLoad) {
        const json = JSON.stringify(options.directLoad);
        const jsonString = JSON.stringify(json);
        const payload = `<script data-skybook-direct-load="1">var __skybook_direct_load=JSON.parse(${jsonString})</script>`;
        content += payload;
    }

    return {
        body: htmlHead + content + htmlTail,
        options: {
            headers: {
                "Content-Type": "text/html",
                "Cross-Origin-Embedder-Policy": "require-corp",
                "Cross-Origin-Opener-Policy": "same-origin",
            }
        }
    };
}
