import { initCodeEditor, type LanguageClient } from "@pistonite/intwc";

import type { ExtensionApp } from "@pistonite/skybook-api";

import { language, configuration } from "./language.ts";
import {
    mergeDataByReplace,
    provideParserDiagnostics,
    type Provider,
    provideRuntimeDiagnostics,
} from "./marker.ts";
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
    getDiagnosticProviders: () => [ParserDiagnosticProvider, RuntimeDiagnosticProvider],
    getSemanticTokensProvider: () => {
        return {
            legend,
            provideDocumentRangeSemanticTokens: (model, range, token) => {
                if (!theApp) {
                    return undefined;
                }
                return provideSemanticTokens(theApp, model, range, token);
            },
        };
    },
};

export const ParserDiagnosticProvider: Provider = {
    ownerId: "parser",
    newRequest: async (_filename, _model, script) => {
        if (!theApp) {
            return [];
        }
        return await provideParserDiagnostics(theApp, script);
    },
    mergeData: mergeDataByReplace,
};

export const RuntimeDiagnosticProvider: Provider = {
    ownerId: "runtime",
    newRequest: async (_filename, _model, script, charPos) => {
        if (!theApp) {
            return [];
        }
        return await provideRuntimeDiagnostics(theApp, script, charPos);
    },
    mergeData: mergeDataByReplace,
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
export const init = () => {
    return initCodeEditor({
        language: {
            custom: [CustomLanguageOptions],
        },
        theme: {
            customTokenColors: CustomTokenColors,
        },
        editor: {
            options: {
                wordWrap: "on"
            }
        }
    });
};
