import { initCodeEditor, type LanguageClient } from "@pistonite/intwc";
import { once } from "@pistonite/pure/sync";

import type { ExtensionApp } from "@pistonite/skybook-api";

import { language, configuration } from "./language.ts";
import { provideDiagnostics } from "./marker.ts";
import { legend, provideSemanticTokens } from "./semantic.ts";

let theApp: ExtensionApp | undefined;
/** Set the app the extension is connected to */
export const setApp = (app: ExtensionApp) => {
    theApp = app;
};

export const updateScriptInApp = (script: string, charPos: number) => {
    if (!theApp) {
        return;
    }
    void theApp.setScript(script, charPos);
};

export const getScriptFromApp = async () => {
    if (!theApp) {
        return "";
    }
    const script = await theApp.getScript();
    return script.val || "";
};

const CustomLanguageOptions: LanguageClient = {
    getId: () => "skybook",
    getExtensions: () => [".skyb"],
    getTokenizer: () => language,
    getConfiguration: () => configuration,
    // the parser and runtime can both produce diagnostics
    getMarkerOwners: () => ["parser", "runtime"],
    provideMarkers: (model, owner) => {
        if (!theApp) {
            return undefined;
        }
        return provideDiagnostics(theApp, model, owner);
    },
    getSemanticTokensLegend: () => legend,
    provideDocumentRangeSemanticTokens: (model, range, token) => {
        if (!theApp) {
            return undefined;
        }
        return provideSemanticTokens(theApp, model, range, token);
    },
};

/** Token colors for special tokens in skybook script */
const CustomTokenColors = [
    {
        token: "string.item.quoted",
        value: "string.regexp",
    },
    {
        token: "string.item.literal",
        value: "string.regexp",
    },
    {
        token: "string.blockliteral",
        value: "tag",
    },
    {
        token: "function.command.super",
        value: "meta.macro",
    },
];

/** Initialize the code editor framework for this window */
export const init = once({
    fn: () => {
        initCodeEditor({
            language: {
                custom: [CustomLanguageOptions],
            },
            theme: {
                customTokenColors: CustomTokenColors,
            },
        });
    },
});
