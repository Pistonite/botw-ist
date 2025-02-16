import * as monaco from 'monaco-editor';

import { initPreference } from "./preference.ts";
import { InitOption } from "./types.ts";
import { initThemes } from './theme';

import { patchMonacoTypeScript } from "./typescript";
import { registerMarkerProvider } from './language/MarkerProviderRegistry.ts';
import { setEditorOptions } from './EditorState.ts';

export function initCodeEditor({preferences, language, editor, theme}: InitOption) {
    initPreference(preferences || {});

    const {typescript, json, css, html, custom} = language || {};

    // initialize monaco
    window.MonacoEnvironment = {
        getWorker: async (_, label: string) => {
            if (typescript && (label === "typescript" || label === "javascript" || label==="tsx" || label==="jsx")) {
                const TypeScriptWorker = (await import('monaco-editor/esm/vs/language/typescript/ts.worker.js?worker')).default;
                return new TypeScriptWorker();
            }
            if (json && (label === "json"|| label === "jsonc")) {
                    const JsonWorker = (await import('monaco-editor/esm/vs/language/json/json.worker.js?worker')).default;
                    return new JsonWorker();
            }
            if (css && (label === "css"|| label === "scss" || label === "sass" || label === "less")) {
                const CssWorker = (await import('monaco-editor/esm/vs/language/css/css.worker.js?worker')).default;
                return new CssWorker();
            }
            if (html && (label === "html"|| label === "htm")) {
                const HtmlWorker = (await import('monaco-editor/esm/vs/language/html/html.worker.js?worker')).default;
                return new HtmlWorker();
            }
            const EditorWorker = (await import('monaco-editor/esm/vs/editor/editor.worker.js?worker')).default;
            return new EditorWorker();
        }
    };

    initThemes(theme || {});

    // initialize TypeScript options
    if (typescript) {

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
                monaco.languages.typescript.typescriptDefaults.addExtraLib(lib.content, `file:///_lib_${lib.name}.ts`);
            });
        }

        patchMonacoTypeScript({
            semanticTokensMaxLength: -1
        });
    }

    if (custom) {
        custom.forEach((client) => {
            console.log("registering", client.getId(), client);
            const id = client.getId();
            monaco.languages.register({ 
                id ,
                extensions: client.getExtensions?.()
            });
            const tokenizer = client.getTokenizer?.();
            if (tokenizer) {
                monaco.languages.registerTokensProviderFactory(id, {
                    create: () => tokenizer
                    });
                // monaco.languages.setMonarchTokensProvider(id, tokenizer);
            }
            const configuration = client.getConfiguration?.();
            if (configuration) {
                monaco.languages.setLanguageConfiguration(id, configuration);
            }

            const getLegend = client.getSemanticTokensLegend?.bind(client);
            const provideDocumentRangeSemanticTokens = client.provideDocumentRangeSemanticTokens?.bind(client);

            if (getLegend && provideDocumentRangeSemanticTokens) {
                monaco.languages.registerDocumentRangeSemanticTokensProvider(id, {
                    getLegend,
                    provideDocumentRangeSemanticTokens
                });
            }

            const provideMarkers = client.provideMarkers?.bind(client);
            const markerOwners = client.getMarkerOwners?.();
            if (provideMarkers && markerOwners) {
                markerOwners.forEach((owner) => {
                    registerMarkerProvider(id, {
                        owner,
                        provide: (model) => provideMarkers(model, owner)
                    });
                });
                // note: the provider invocation is registered
                // in EditorState using the onDidChangeContent event
            }

            const provideCompletionItems = client.provideCompletionItems?.bind(client);
            if (provideCompletionItems) {
                const completionTriggerCharacters = client.getCompletionTriggerCharacters?.();
                const resolveCompletionItem = client.resolveCompletionItem?.bind(client);
                monaco.languages.registerCompletionItemProvider(id, {
                    triggerCharacters: completionTriggerCharacters,
                    provideCompletionItems,
                    resolveCompletionItem
                });
            }
        });
    }

    if (editor) {
        setEditorOptions(editor);
    }
}
