import { parseEnvFromScript, type DirectLoad } from "@pistonite/skybook-api";

import { type ResponsePayload, safeParseUrl, useAcceptLanguage } from "self::framework";
import { VERSION } from "self::util";

import Strings from "./strings.json" with { type: "json" };

type Language = keyof typeof Strings.title;
const languages = Object.keys(Strings.title) as Language[];

const loadEntryPointHtml = async (): Promise<[string, string]> => {
    console.log("loading entry point for server-side rendering");
    const indexHtml = await Bun.file("app/index.html").text();
    const headStartIndex = indexHtml.indexOf("<head>");
    if (headStartIndex == -1) {
        throw new Error("No <head> tag found in index.html");
    }
    const i = headStartIndex + "<head>".length;
    return [indexHtml.substring(0, i), indexHtml.substring(i)];
};
const entryPointHtml = await loadEntryPointHtml();

const getVersion = () => {
    return VERSION.replace("0.", "v");
};

export type SSROptions = {
    /** URL to put in meta */
    url: string;
    /** Direct load payload to inject into the page */
    directLoad?: DirectLoad;
    /** File reference to appear in the meta tags */
    file?: {
        /** File name */
        name: string;
        /** Short version of the file name */
        short?: string;
    };
};

/**
 * Renders the HTML on the server-side
 *
 * This handles injecting the direct load script and meta tags into the HTML
 */
export const makeSSR = async (req: Request, options: SSROptions): Promise<ResponsePayload> => {
    const language = useAcceptLanguage(req, languages, "en-US");

    let origin: string;
    const url = safeParseUrl(options.url);
    if ("err" in url) {
        origin = "https://ist.pistonite.app";
    } else {
        origin = url.val.origin || "https://ist.pistonite.app";
    }

    const titleTag = `<title>${Strings.title[language]}</title>`;

    // <script> block for direct load
    let directLoadScript: string;
    let customImage = "";
    // site name, if direct load specifies an image, also pull out the image
    // and display it
    if (options.directLoad) {
        const json = JSON.stringify(options.directLoad);
        const jsonString = JSON.stringify(json);
        directLoadScript = `<script data-skybook-direct-load="1">var __skybook_direct_load=JSON.parse(${jsonString})</script>`;
        const scriptEnv = parseEnvFromScript(options.directLoad.content);
        if (scriptEnv.image) {
            customImage = ` (custom-image:ver${scriptEnv.image}`;
            switch (scriptEnv.params.dlc) {
                case 0: {
                    customImage += "-nodlc)";
                    break;
                }
                case 1: {
                    customImage += "-dlc-1)";
                    break;
                }
                case 2: {
                    customImage += "-dlc-2)";
                    break;
                }
                default: {
                    customImage += ")";
                    break;
                }
            }
        }
    } else {
        directLoadScript = "";
    }

    const urlTag = `<meta name="og:url" content="${options.url}">`;

    // In discord, this is the small grey text on top of the card
    const siteNameTag = `<meta name="og:site_name" content="Skybook ${getVersion()}${customImage}">`;
    // In discord, this is the vertical color bar of the card
    const themeColor = customImage ? "#EE15F4" : "#73FBFD";
    const themeColorTag = `<meta name="theme-color" content="${themeColor}">`;

    // In discord, this is the title (big text) of the card
    let longTitle = Strings.title[language];
    if (options.file?.short) {
        longTitle += ` - ${options.file.short}`;
    }
    const titleMetaTag = `<meta name="og:title" content="${longTitle}">`;

    // In discord, this is the small text below the title
    let description = "";
    if (options.file) {
        const text = Strings["description-file"][language];
        description = text.replace("{{file}}", options.file.name);
    } else if (options.directLoad) {
        if (options.directLoad.type === "v3") {
            description = Strings["description-legacy"][language];
        } else {
            description = Strings.description[language];
        }
    }
    const descriptionTag = description
        ? `<meta name="og:description" content="${description}">`
        : "";

    // In discord, this is the image that appears on the card
    const icon = customImage ? "icon-purple" : "icon";
    const imageTag = `<meta name="og:image" content="${origin}/static/${icon}.png">`;

    // also set the favicon
    const faviconTag = `<link rel="icon" type="image/svg+xml" href="/static/${icon}.svg" />`;

    const content =
        faviconTag +
        titleTag +
        directLoadScript +
        urlTag +
        siteNameTag +
        themeColorTag +
        titleMetaTag +
        descriptionTag +
        imageTag;

    const [htmlHead, originalhtmlTail] = entryPointHtml;
    let htmlTail = originalhtmlTail;

    // replace boot logo
    if (options.directLoad) {
        if (customImage) {
            htmlTail = htmlTail.replace(
                /<img data-ssr-boot-logo [^>]*>/,
                `<img class="start" src="/static/${icon}.svg" />`,
            );
        }
    } else {
        // use a script to render the logo early
        const script = `<script> (function (){
let i = "icon";
try { if (localStorage.getItem("Skybook.EarlyCI")) { i += "-purple"; } } catch {}
document.write('<img class="start" src="/static/'+i+'.svg" />');
})() </script>`;
        htmlTail = htmlTail.replace(/<img data-ssr-boot-logo [^>]*>/, script);
    }

    return {
        body: htmlHead + content + htmlTail,
        options: {
            headers: {
                "Content-Type": "text/html",
            },
        },
    };
};
