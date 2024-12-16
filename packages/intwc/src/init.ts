import * as monaco from 'monaco-editor';

import { initPreference } from "./preference.ts";
import { InitOption } from "./types.ts";
import { initThemes } from './theme';
import { legend } from './semanticTokens.ts';

export async function initCodeEditor({preferences, language, editor}: InitOption) {
    initPreference(preferences || {});

    const {typescript, json, yaml, css, html, toml} = language || {};

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
            if (yaml && (label === "yaml"|| label === "yml")) {
                const YamlWorker = (await import('monaco-editor/esm/vs/basic-languages/yaml/yaml.js?worker')).default;
                return new YamlWorker();
            }
            const EditorWorker = (await import('monaco-editor/esm/vs/editor/editor.worker.js?worker')).default;
            return new EditorWorker();
        }
    };

    initThemes();

    // initialize TypeScript options
    if (typescript) {

        const tsLibs = (typescript.lib || ["esnext"]).filter((lib) => typeof lib === "string");
        // monaco.languages.typescript.typescriptDefaults.inlayHintsOptions
        monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
            target: monaco.languages.typescript.ScriptTarget.ESNext,
            lib: tsLibs,
            noEmit: true,
            strict: true,
            jsx: "preserve",
            noUnusedLocals: true,
            noUnusedParameters: true,
            noFallthroughCasesInSwitch: true,
        });

        if (typescript.lib) {
            typescript.lib.forEach((lib) => {
                if (typeof lib === "string") {
                    return;
                }

                monaco.languages.typescript.typescriptDefaults.addExtraLib(lib.content, `file:///_lib_${lib.name}.ts`);
            });
        }

        monaco.languages.registerDocumentSemanticTokensProvider("typescript", {
            getLegend: () => legend,
            releaseDocumentSemanticTokens: () => {},
            provideDocumentSemanticTokens: (model, lastResultId, token) => {
                const data = [];
                data.push(0, 0, 15, 0, 0);

                return {  
                    data: new Uint32Array(data),
                };
            }
        });
    }
}
