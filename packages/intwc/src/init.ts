import * as monaco from "monaco-editor";
import { once } from "@pistonite/pure/sync";

import { initPreference } from "./preference.ts";
import type { InitOption } from "./types.ts";
import { initThemes } from "./theme";
import { patchMonacoTypeScript } from "./typescript";
import { setEditorOptions } from "./editor_state.ts";
import { registerDiagnosticProvider } from "./language/diagnostic_provider.ts";
import { log } from "./internal.ts";

const initCodeEditorInternal = ({ preferences, language, editor, theme }: InitOption) => {
    initPreference(preferences || {});

    const { typescript, custom } = language || {};

    initThemes(theme || {});

    // initialize TypeScript options
    if (typescript) {
        if (!import.meta.env.INTWC_TYPESCRIPT) {
            log.warn(
                "TypeScript init options are set, but TypeScript is not loaded using intwc plugin!!!",
            );
        } else {
            const dom = typescript.dom ?? true;
            monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
                target: monaco.languages.typescript.ScriptTarget.ESNext,
                lib: dom ? undefined : ["esnext"],
                noEmit: true,
                strict: true,
                // jsx: "preserve",
                noUnusedLocals: true,
                noUnusedParameters: true,
                noFallthroughCasesInSwitch: true,
            });

            if (typescript.extraLibs) {
                typescript.extraLibs.forEach((lib) => {
                    monaco.languages.typescript.typescriptDefaults.addExtraLib(
                        lib.content,
                        `file:///_lib_${lib.name}.ts`,
                    );
                });
            }

            patchMonacoTypeScript({ semanticTokensMaxLength: -1 });
        }
    }

    if (custom) {
        custom.forEach((client) => {
            const id = client.getId();
            monaco.languages.register({
                id,
                extensions: client.getExtensions?.(),
            });
            const tokenizer = client.getTokenizer?.();
            if (tokenizer) {
                monaco.languages.registerTokensProviderFactory(id, {
                    create: () => tokenizer,
                });
            }
            const configuration = client.getConfiguration?.();
            if (configuration) {
                monaco.languages.setLanguageConfiguration(id, configuration);
            }

            const semanticTokensProvider = client.getSemanticTokensProvider?.();
            if (semanticTokensProvider) {
                const legend = semanticTokensProvider.legend;
                const provideDocumentRangeSemanticTokens =
                    semanticTokensProvider.provideDocumentRangeSemanticTokens.bind(
                        semanticTokensProvider,
                    );
                monaco.languages.registerDocumentRangeSemanticTokensProvider(id, {
                    getLegend: () => legend,
                    provideDocumentRangeSemanticTokens,
                });
            }

            const diagnosticProviders = client.getDiagnosticProviders?.();
            if (diagnosticProviders) {
                diagnosticProviders.forEach((p) => registerDiagnosticProvider(id, p));
            }

            const provideCompletionItems = client.provideCompletionItems?.bind(client);
            if (provideCompletionItems) {
                const completionTriggerCharacters = client.getCompletionTriggerCharacters?.();
                const resolveCompletionItem = client.resolveCompletionItem?.bind(client);
                monaco.languages.registerCompletionItemProvider(id, {
                    triggerCharacters: completionTriggerCharacters,
                    provideCompletionItems,
                    resolveCompletionItem,
                });
            }
        });
    }

    if (editor) {
        setEditorOptions(editor);
    }
};

/** Initialize INTWC code editor service */
export const initCodeEditor = once({ fn: initCodeEditorInternal });
