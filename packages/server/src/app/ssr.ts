import { DirectLoad } from "client";
import { ResponsePayload } from "framework";

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
};

export const makeSSR = (options: SSROptions): ResponsePayload => {

    let content = "";
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
