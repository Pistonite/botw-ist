import { CodeEditor, type CodeEditorApi } from "@pistonite/intwc";
import type { WorkexPromise } from "@pistonite/workex";

import type { ExtensionApp } from "@pistonite/skybook-api";

import {
    FirstPartyExtensionAdapter,
    type FirstPartyExtension,
} from "../FirstParty.ts";

import { init, setApp, updateScriptInApp } from "./init.ts";

init();

const FILE = "script.skyb";

export class EditorExtension
    extends FirstPartyExtensionAdapter
    implements FirstPartyExtension
{
    private editor: CodeEditorApi | undefined;
    private scriptChangedBeforeEditorReady: string | undefined;
    private detachEditor: (() => void) | undefined;

    private component: React.FC;

    constructor(standalone: boolean) {
        super(standalone);

        this.component = () => {
            return (
                <CodeEditor
                    onCreated={(editor) => {
                        void this.attachEditor(editor);
                        return undefined;
                    }}
                />
            );
        };
    }

    public get Component() {
        return this.component;
    }

    public override onAppConnectionEstablished(app: ExtensionApp): void {
        setApp(app);
    }

    /**
     * Attach the extension instance to an editor.
     * Automatically detaches other previously attached editor
     */
    public async attachEditor(editor: CodeEditorApi): Promise<() => void> {
        if (this.editor === editor) {
            return this.detachEditor || (() => {});
        }
        const detachEditor = this.detachEditor;
        this.detachEditor = undefined;
        detachEditor?.();

        this.editor = editor;
        const unsubscribeEditor = editor.subscribe((filename) => {
            if (filename !== FILE) {
                return;
            }
            updateScriptInApp(editor.getFileContent(FILE));
        });
        this.detachEditor = () => {
            this.detachEditor = undefined;
            unsubscribeEditor();
            this.editor = undefined;
        };
        if (this.scriptChangedBeforeEditorReady) {
            const script = this.scriptChangedBeforeEditorReady;
            this.scriptChangedBeforeEditorReady = undefined;
            await this.onScriptChanged(script);
        }
        return this.detachEditor || (() => {});
    }

    public override async onScriptChanged(script: string): WorkexPromise<void> {
        if (!this.editor) {
            this.scriptChangedBeforeEditorReady = script;
            return {};
        }
        if (!this.editor.getFiles().includes(FILE)) {
            this.editor.openFile(FILE, script, "skybook");
        } else {
            this.editor.setFileContent(FILE, script);
        }
        return {};
    }
}
