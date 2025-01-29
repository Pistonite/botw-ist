import { CodeEditorApi } from "@pistonite/intwc";
import { FirstPartyExtensionAdapter } from "extensions/FirstPartyAdapter";
import { WorkexPromise } from "workex";


export class EditorExtension extends FirstPartyExtensionAdapter {

    constructor(
        private scriptFile: string,
        standalone: boolean,
        private editor: CodeEditorApi) {
        super(standalone);
    }
    public async onScriptChanged(script: string): WorkexPromise<void> {
        if (!this.editor.getFiles().includes(this.scriptFile)) {
            this.editor.openFile(this.scriptFile, script, "skyb");
        } else {
            this.editor.setFileContent(this.scriptFile, script);
        }
        return {};
    }
}
