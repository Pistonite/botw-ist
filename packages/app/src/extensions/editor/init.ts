import { initCodeEditor } from "@pistonite/intwc";
import type { ExtensionApp } from "@pistonite/skybook-api";
import { once } from "@pistonite/pure/sync";

import { language, configuration } from "./language.ts";
import { provideParserDiagnostics } from "./marker.ts";
import { legend, provideSemanticTokens } from "./semantic.ts";

let theApp: ExtensionApp | undefined;
/** Set the app the extension is connected to */
export const setApp = (app: ExtensionApp) => {
    theApp = app;
};

export const updateScriptInApp = (script: string) => {
    if (!theApp) {
        return;
    }
    void theApp.setScript(script);
};

export const getScriptFromApp = async () => {
    if (!theApp) {
        return "";
    }
    const script = await theApp.getScript();
    return script.val || "";
};

/** Initialize the code editor framework for this window */
export const init = once({
    fn: () => {
        initCodeEditor({
            language: {
                custom: [
                    {
                        getId: () => "skybook",
                        getExtensions: () => [".skyb"],
                        getTokenizer: () => language,
                        getConfiguration: () => configuration,
                        // the parser and runtime can both produce diagnostics
                        getMarkerOwners: () => ["parser"],
                        provideMarkers: (model) => {
                            if (!theApp) {
                                return undefined;
                            }
                            return provideParserDiagnostics(theApp, model);
                        },
                        getSemanticTokensLegend: () => legend,
                        provideDocumentRangeSemanticTokens: (
                            model,
                            range,
                            token,
                        ) => {
                            if (!theApp) {
                                return undefined;
                            }
                            return provideSemanticTokens(
                                theApp,
                                model,
                                range,
                                token,
                            );
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
                        value: "meta.macro",
                    },
                ],
            },
        });
    },
});
