import * as monaco from "monaco-editor";

import type { EditorOption } from "./types.ts";
import type { Position } from "./monaco_types.ts";
import { getFileUri, getNormalizedPath } from "./utils.ts";
import { provideMarkers } from "./language/diagnostic_provider.ts";

export type CodeEditorEvent = "content-changed" | "cursor-position-changed";

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

    /**
     * Get the current primary cursor position (line number and column) in the editor.
     *
     * Note that both line number and column are 1-based.
     */
    getCursorPosition: () => Position | undefined;

    /**
     * Set the primary cursor position in the editor
     *
     * Note that both line number and column are 1-based.
     */
    setCursorPosition: (position: Position) => void;

    /**
     * Get the character offset of the primary cursor. The offset is 0-based.
     * Note that this is not always the byte offset
     */
    getCursorOffset: () => number | undefined;

    /**
     * Set the primary cursor position in the editor using the character offset.
     * Note that this is not always the byte offset
     */
    setCursorOffset: (offset: number) => void;

    /** Subscribe to editor events */
    subscribe: (event: CodeEditorEvent, callback: (filename: string) => void) => () => void;

    /** Set if the editor is read only (in the future, this might be changed to per-file based */
    setReadonly(isReadonly: boolean): void;
};

let editorOptions: EditorOption = { options: {} };

export const setEditorOptions = (options: EditorOption) => {
    editorOptions = options;
};

export class EditorState implements CodeEditorApi {
    private instance: monaco.editor.IStandaloneCodeEditor;
    private models: Map<string, monaco.editor.ITextModel>;
    private extraCleanup: () => void;
    private subscribers: Map<CodeEditorEvent, ((filename: string) => void)[]>;

    constructor(node: HTMLDivElement) {
        this.models = new Map();
        this.subscribers = new Map();

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

        const cursorListener = this.instance.onDidChangeCursorPosition(() => {
            const currentFile = this.getCurrentFile();
            if (currentFile === undefined) {
                return;
            }
            const subscribers = this.subscribers.get("cursor-position-changed");
            subscribers?.forEach((subscriber) => subscriber(currentFile));
        });

        this.extraCleanup = () => {
            cursorListener.dispose();
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
        filename = getNormalizedPath(filename);
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
        const model = this.models.get(getNormalizedPath(filename));
        if (model) {
            return model.getValue();
        }
        return "";
    }

    public setFileContent(filename: string, content: string) {
        const model = this.models.get(getNormalizedPath(filename));
        if (model && model.getValue() !== content) {
            model.setValue(content);
        }
    }

    public getCursorPosition(): Position | undefined {
        return this.instance.getPosition() || undefined;
    }

    public getCursorOffset(): number | undefined {
        const position = this.instance.getPosition();
        if (!position) {
            return undefined;
        }
        return this.instance.getModel()?.getOffsetAt(position);
    }

    public setCursorOffset(offset: number) {
        const model = this.instance.getModel();
        if (!model) {
            return;
        }
        const position = model.getPositionAt(offset);
        if (!position) {
            return;
        }
        this.instance.setPosition(position);
    }

    public setCursorPosition(position: Position) {
        this.instance.setPosition(position);
    }

    public subscribe(event: CodeEditorEvent, callback: (filename: string) => void): () => void {
        let subscribers = this.subscribers.get(event);
        if (!subscribers) {
            subscribers = [];
            this.subscribers.set(event, subscribers);
        }
        subscribers.push(callback);
        return () => {
            const subscribers = this.subscribers.get(event);
            if (!subscribers) {
                return;
            }
            const index = subscribers.indexOf(callback);
            if (index !== -1) {
                subscribers.splice(index, 1);
            }
        };
    }

    public openFile(filename: string, content: string, language: string) {
        const uri = getFileUri(filename);
        filename = uri.path;
        const model = this.models.get(filename);
        if (!model) {
            const model = monaco.editor.createModel(content, language, uri);
            // there can be only one change event listener, so this is not exposed
            model.onDidChangeContent(() => {
                void provideMarkers(filename, model, this.getCursorOffset() || 0);
                const subscribers = this.subscribers.get("content-changed");
                subscribers?.forEach((subscriber) => subscriber(filename));
            });
            model.updateOptions({
                tabSize: 2,
                indentSize: 2,
                insertSpaces: true,
                trimAutoWhitespace: true,
                bracketColorizationOptions: {
                    enabled: false,
                    independentColorPoolPerBracketType: false,
                },
            });
            this.models.set(filename, model);
            // invoke the callback once to provide markers initially
            void provideMarkers(filename, model, 0);
        }
        this.switchToFile(filename);
    }

    public switchToFile(filename: string) {
        const model = this.models.get(getNormalizedPath(filename));
        if (model) {
            this.instance.setModel(model);
        }
    }

    public setReadonly(isReadonly: boolean): void {
        this.instance.updateOptions({
            readOnly: isReadonly,
        });
    }
}
