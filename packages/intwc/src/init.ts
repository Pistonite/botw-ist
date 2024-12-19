import * as monaco from 'monaco-editor-contrib';

import { initPreference } from "./preference.ts";
import { InitOption } from "./types.ts";
import { initThemes } from './theme';

import { patchMonacoTypeScript } from "@pistonite/monaco-typescript-contrib";

export async function initCodeEditor({preferences, language, editor}: InitOption) {
    initPreference(preferences || {});

    const {typescript, json, yaml, css, html, toml} = language || {};

    // initialize monaco
    window.MonacoEnvironment = {
        getWorker: async (_, label: string) => {
            if (typescript && (label === "typescript" || label === "javascript" || label==="tsx" || label==="jsx")) {
                const TypeScriptWorker = (await import('monaco-editor-contrib/esm/vs/language/typescript/ts.worker.js?worker')).default;
                return new TypeScriptWorker();
            }
            if (json && (label === "json"|| label === "jsonc")) {
                    const JsonWorker = (await import('monaco-editor-contrib/esm/vs/language/json/json.worker.js?worker')).default;
                    return new JsonWorker();
            }
            if (css && (label === "css"|| label === "scss" || label === "sass" || label === "less")) {
                const CssWorker = (await import('monaco-editor-contrib/esm/vs/language/css/css.worker.js?worker')).default;
                return new CssWorker();
            }
            if (html && (label === "html"|| label === "htm")) {
                const HtmlWorker = (await import('monaco-editor-contrib/esm/vs/language/html/html.worker.js?worker')).default;
                return new HtmlWorker();
            }
            const EditorWorker = (await import('monaco-editor-contrib/esm/vs/editor/editor.worker.js?worker')).default;
            return new EditorWorker();
        }
    };

    initThemes();

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

        patchMonacoTypeScript();
    }
}
