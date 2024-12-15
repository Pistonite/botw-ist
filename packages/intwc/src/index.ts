import * as monaco from 'monaco-editor';

export type Config = {
    /** */
    createEditorWorker: () => Worker;
    /** Configuration if TypeScript language features should be enabled */
    typescript?: TypeScriptConfig;
}

export type TypeScriptConfig = {
    createTypeScriptWorker: () => Worker;
}

export async function initCodeEditorService({
    createEditorWorker,
    typescript,
}: Config) {
    window.MonacoEnvironment = {
        getWorker: (_, label: string) => {
            if (label === "typescript" || label === "javascript") {
                if (typescript) {
                    return typescript.createTypeScriptWorker();
                }
            }
            return createEditorWorker();
        }
    };
}
