import {
    getNormalizedPath,
    CodeEditor,
    type CodeEditorApi,
} from "@pistonite/intwc";
import type { WxPromise } from "@pistonite/workex";

import type { ExtensionApp } from "@pistonite/skybook-api";

import {
    FirstPartyExtensionAdapter,
    type FirstPartyExtension,
} from "../FirstParty.ts";

import { init, setApp, updateScriptInApp } from "./init.ts";

void init();

const FILE = getNormalizedPath("script.skyb");

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
        super.onAppConnectionEstablished(app);
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

        const updateHandler = (filename: string) => {
            if (filename !== FILE) {
                return;
            }
            updateScriptInApp(
                editor.getFileContent(FILE),
                editor.getCursorOffset() || 0,
            );
        };

        const unsubscribeContentChange = editor.subscribe(
            "content-changed",
            updateHandler,
        );
        const unsubscribeCursorPositionChange = editor.subscribe(
            "cursor-position-changed",
            updateHandler,
        );

        this.detachEditor = () => {
            this.detachEditor = undefined;
            unsubscribeContentChange();
            unsubscribeCursorPositionChange();
            this.editor = undefined;
        };
        if (this.scriptChangedBeforeEditorReady !== undefined) {
            const script = this.scriptChangedBeforeEditorReady;
            this.scriptChangedBeforeEditorReady = undefined;
            await this.onScriptChanged(script);
        } else if (this.app) {
            const script = await this.app.getScript();
            if (script.val && this.editor) {
                await this.onScriptChanged(script.val);
            }
        }
        return this.detachEditor || (() => {});
    }

    public override async onScriptChanged(script: string): WxPromise<void> {
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
