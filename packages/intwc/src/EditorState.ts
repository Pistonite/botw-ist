import * as monaco from 'monaco-editor';
import { getProvideMarkersCallback } from './language/MarkerProviderRegistry';
import { EditorOption } from './types.ts';


export type CodeEditorApi = {
    /** Get the list of opened files */
    getFiles: () => string[];

    /** Get the current opened file in the editor */
    getCurrentFile: () => string | undefined;
    /** 
     * Switch to the given file in the editor
     *
     * Does nothing if the file doesn't exist in the opened files
     */
    switchToFile: (filename: string) => void;
    /**
     * Close the given file in the editor
     *
     * Does nothing if the file doesn't exist in the opened files
     */
    closeFile: (filename: string) => void;
    /**
     * Open a new file and switch to it in the editor.
     *
     * If the file is already opened, just switch to it.
     */
    openFile(filename: string, content: string, language: string): void;

    /** Get the content of the given file */
    getFileContent: (filename: string) => string;

    /** Set the content of the given file in the editor */
    setFileContent: (filename: string, content: string) => void;

    /** Subscribe to when files are changed */
    subscribe: (callback: (filename: string) => void) => () => void;
}

let editorOptions: EditorOption = {
    options: {},
}

export const setEditorOptions = (options: EditorOption) => {
    editorOptions = options;
}

export class EditorState implements CodeEditorApi {
    private instance: monaco.editor.IStandaloneCodeEditor;
    private models: Map<string, monaco.editor.ITextModel>;
    private extraCleanup: () => void;
    private subscribers: ((filename: string) => void)[];

    constructor(node: HTMLDivElement) {
        this.models = new Map();
        this.subscribers = [];

        this.instance = monaco.editor.create(node, {
            autoDetectHighContrast: true,
            wordBasedSuggestions: "off",
            bracketPairColorization: {
                enabled: false,
                independentColorPoolPerBracketType: false,
            },
            "semanticHighlighting.enabled": true,
            automaticLayout: true,
            tabSize: 2,
            insertSpaces: true,
            ...editorOptions.options,
        });

        this.extraCleanup = () => {
            
        };
    }

    /** Dispose the editor */
    public dispose() {
        this.extraCleanup();
        this.instance.setModel(null);
        for (const model of this.models.values()) {
            model.dispose();
        }
        this.instance.dispose();
    }

    public getFiles(): string[] {
        return Array.from(this.models.keys());
    }

    public getCurrentFile(): string | undefined {
        return this.instance.getModel()?.uri.path;
    }

    public closeFile(filename: string) {
        const model = this.models.get(filename);
        if (model) {
            if (model === this.instance.getModel()) {
                this.instance.setModel(null);
            }
            model.dispose();
            this.models.delete(filename);
        }
    }

    public getFileContent(filename: string): string {
        const model = this.models.get(filename);
        if (model) {
            console.log("get", model.getValue());
            return model.getValue();
        }
        return "";
    }

    public setFileContent(filename: string, content: string) {
        const model = this.models.get(filename);
        if (model && model.getValue() !== content) {
            console.log(model.getValue(), content);
            // model.setValue(content);
        }
    }

    public subscribe(callback: (filename: string) => void): () => void {
        this.subscribers.push(callback);
        return () => {
            const index = this.subscribers.indexOf(callback);
            if (index !== -1) {
                this.subscribers.splice(index, 1);
            }
        };
    }


    public openFile(filename: string, content: string, language: string) {
        const model = this.models.get(filename);
        if (!model) {
            const model = monaco.editor.createModel(content, language, monaco.Uri.file(filename));
            const provideMarkersCallback = getProvideMarkersCallback();
            // there can be only one change event listener, so this is not exposed
            model.onDidChangeContent(() => {
                console.log("changed", model.getValue());
                provideMarkersCallback(model);
                this.subscribers.forEach(subscriber => subscriber(filename));
            });
            model.updateOptions({
                tabSize: 2,
                indentSize: 2,
                insertSpaces: true,
                trimAutoWhitespace: true,
                bracketColorizationOptions: {
                    enabled: false,
                    independentColorPoolPerBracketType: false,
                }
            });
            this.models.set(filename, model);
        }
        this.switchToFile(filename);
    }

    public switchToFile(filename: string) {
        const model = this.models.get(filename);
        if (model) {
            this.instance.setModel(model);
        }
    }
}
