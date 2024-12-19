// import { isDark } from '@pistonite/pure/pref';
import * as monaco from 'monaco-editor-contrib';


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
}

export class EditorState {
    private node: HTMLDivElement;
    // private currentFile: string | undefined;
    private instance: monaco.editor.IStandaloneCodeEditor;
    private models: Map<string, monaco.editor.ITextModel>;

    private extraCleanup: () => void;

    constructor(node: HTMLDivElement) {
        this.node = node;
        this.models = new Map();
        // const dark = isDark();


        this.instance = monaco.editor.create(this.node, {
            wordBasedSuggestions: "off",
            bracketPairColorization: {
                enabled: false,
                independentColorPoolPerBracketType: false,
            },
            "semanticHighlighting.enabled": true,
        });

        this.extraCleanup = () => {
            
        };
    }

    public dispose() {
        this.extraCleanup();
        this.instance.setModel(null);
        for (const model of this.models.values()) {
            model.dispose();
        }
        this.instance.dispose();
    }

    public openFile(filename: string, content: string, language: string) {
        const model = this.models.get(filename);
        if (!model) {
            const model = monaco.editor.createModel(content, language, monaco.Uri.file(filename));
            model.updateOptions({

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
            this.currentFile = filename;
            this.instance.setModel(model);
        }
    }
}
