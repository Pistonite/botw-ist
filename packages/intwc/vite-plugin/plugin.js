// vite-plugin/plugin.ts
var createBasicLanguagesMonacoContributionModule = (basicLanguages) => {
    const lines = ["import '../editor/editor.api.js';"];
    basicLanguages.forEach((lang) => {
        lines.push(`import '../basic-languages/${lang}/${lang}.contribution.js';`);
    });
    return lines.join(`
`);
};
var createEditorMain = (options) => {
    const lines = [createBasicLanguagesMonacoContributionModule(options.basicLanguages)];
    if (options.typescript) {
        lines.push("import '../language/typescript/monaco.contribution';");
    }
    if (options.json) {
        lines.push("import '../language/json/monaco.contribution';");
    }
    if (options.css) {
        lines.push("import '../language/css/monaco.contribution';");
    }
    if (options.html) {
        lines.push("import '../language/html/monaco.contribution';");
    }
    lines.push("self['MonacoEnvironment'] = { getWorker: async (_, label) => {");
    if (options.typescript) {
        lines.push(`if (label === 'typescript' || label === 'javascript') {`);
        lines.push(
            `const W = (await import('monaco-editor/esm/vs/language/typescript/ts.worker.js?worker')).default;`,
        );
        lines.push(`return new W();`);
        lines.push(`}`);
    }
    if (options.json) {
        lines.push(`if (label === 'json' || label === 'jsonc') {`);
        lines.push(
            `const W = (await import('monaco-editor/esm/vs/language/json/json.worker.js?worker')).default;`,
        );
        lines.push(`return new W();`);
        lines.push(`}`);
    }
    if (options.css) {
        lines.push(
            `if (label === 'css' || label === 'scss' || label === 'sass' || label === 'less') {`,
        );
        lines.push(
            `const W = (await import('monaco-editor/esm/vs/language/css/css.worker.js?worker')).default;`,
        );
        lines.push(`return new W();`);
        lines.push(`}`);
    }
    if (options.html) {
        lines.push(`if (label === 'html' || label === 'htm') {`);
        lines.push(
            `const W = (await import('monaco-editor/esm/vs/language/html/html.worker.js?worker')).default;`,
        );
        lines.push(`return new W();`);
        lines.push(`}`);
    }
    lines.push(
        `const W = (await import('monaco-editor/esm/vs/editor/editor.worker.js?worker')).default;`,
    );
    lines.push("return new W();");
    lines.push("}};");
    lines.push("export * from './edcore.main';");
    return lines.join(`
`);
};
var intwcChunks = { intwc: ["@pistonite/intwc", "monaco-editor"] };
var updateRollupOutputConfig = (output) => {
    const chunkFileNamesOriginal = output?.chunkFileNames;
    const assetFileNamesOriginal = output?.assetFileNames;
    const manualChunksOriginal = output?.manualChunks;
    const manualChunks =
        typeof manualChunksOriginal === "function"
            ? manualChunksOriginal
            : { ...manualChunksOriginal, ...intwcChunks };
    const chunkFileNames = (info) => {
        for (let i = 0; i < info.moduleIds.length; i++) {
            if (info.moduleIds[i].match(/esm[/\\]vs[/\\]basic-languages/)) {
                return `assets/intwc/basic-${info.name}-[hash].js`;
            }
            if (info.moduleIds[i].match(/esm[/\\]vs[/\\]language/)) {
                return `assets/intwc/lang-${info.name}-[hash].js`;
            }
            if (info.moduleIds[i].match(/esm[/\\]vs[/\\]editor/)) {
                return `assets/intwc/editor-${info.name}-[hash].js`;
            }
        }
        if (!chunkFileNamesOriginal) {
            return `assets/${info.name}-[hash].js`;
        }
        if (typeof chunkFileNamesOriginal === "function") {
            return chunkFileNamesOriginal(info);
        }
        return chunkFileNamesOriginal;
    };
    const assetFileNames = (info) => {
        if (!info.originalFileNames) {
            if (info.name && info.name.match(/intwc/)) {
                return "assets/intwc/[name]-[hash][extname]";
            }
            return `assets/[name]-[hash][extname]`;
        }
        for (let i = 0; i < info.originalFileNames.length; i++) {
            if (
                info.originalFileNames[i].match(/esm[/\\]vs[/\\]language/) ||
                info.originalFileNames[i].match(/esm[/\\]vs[/\\]editor/) ||
                info.originalFileNames[i].match(/esm[/\\]vs[/\\]base/)
            ) {
                return "assets/intwc/[name]-[hash][extname]";
            }
            if (info.names[i].match("intwc")) {
                return "assets/intwc/[name]-[hash][extname]";
            }
        }
        if (!assetFileNamesOriginal) {
            return `assets/[name]-[hash][extname]`;
        }
        if (typeof assetFileNamesOriginal === "function") {
            return assetFileNamesOriginal(info);
        }
        return assetFileNamesOriginal;
    };
    if (output) {
        return { ...output, chunkFileNames, assetFileNames, manualChunks };
    }
    return { chunkFileNames, assetFileNames, manualChunks };
};
function plugin(options) {
    return {
        name: "vite-plugin-intwc",
        enforce: "pre",
        config(config) {
            if (!config.define) {
                config.define = {};
            }
            config.define["import.meta.env.INTWC_TYPESCRIPT"] = !!options.typescript;
            if (!config.build) {
                config.build = {};
            }
            if (!config.build.rollupOptions) {
                config.build.rollupOptions = {};
            }
            const rollupOptions = config.build.rollupOptions;
            const output = rollupOptions.output;
            if (output && Array.isArray(output)) {
                for (let i = 0; i < output.length; i++) {
                    output[i] = updateRollupOutputConfig(output[i]);
                }
            } else {
                rollupOptions.output = updateRollupOutputConfig(output);
            }
            if (!config.optimizeDeps) {
                config.optimizeDeps = {};
            }
            if (config.optimizeDeps.exclude) {
                if (!config.optimizeDeps.exclude.includes("monaco-editor")) {
                    config.optimizeDeps.exclude.push("monaco-editor");
                }
            } else {
                config.optimizeDeps.exclude = ["monaco-editor"];
            }
        },
        load(id) {
            if (id.match(/esm[/\\]vs[/\\]editor[/\\]editor.main.js/)) {
                return createEditorMain(options);
            }
            return null;
        },
    };
}
export { intwcChunks, plugin as default };
