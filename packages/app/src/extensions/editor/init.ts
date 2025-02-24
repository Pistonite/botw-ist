import { initCodeEditor } from "@pistonite/intwc";
import type { ExtensionApp } from "@pistonite/skybook-api";

import { language, configuration } from "./language.ts";
import { provideParserDiagnostics } from "./marker.ts";
import { legend, provideSemanticTokens } from "./semantic.ts";

let initialized = false;

export const init = (app: ExtensionApp) => {
    if (initialized) {
        return;
    }
    initialized = true;
    initCodeEditor({
        language: {
            custom: [
                {
                    getId: () => "skyb",
                    getExtensions: () => [".skyb"],
                    getTokenizer: () => language,
                    getConfiguration: () => configuration,
                    // the parser and runtime can both produce diagnostics
                    getMarkerOwners: () => ["parser"],
                    provideMarkers: (model) => {
                        return provideParserDiagnostics(app, model);
                    },
                    getSemanticTokensLegend: () => legend,
                    provideDocumentRangeSemanticTokens: (
                        model,
                        range,
                        token,
                    ) => {
                        return provideSemanticTokens(app, model, range, token);
                    },
                },
            ],
        },
        theme: {
            customTokenColors: [
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
                    value: "constant",
                },
            ],
        },
    });
};
