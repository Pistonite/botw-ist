import type { CodeEditorApi } from "@pistonite/intwc";
import type { WorkexPromise } from "@pistonite/workex";

import { FirstPartyExtensionAdapter } from "extensions/FirstPartyAdapter";

export class EditorExtension extends FirstPartyExtensionAdapter {
    constructor(
        private scriptFile: string,
        standalone: boolean,
        private editor: CodeEditorApi,
    ) {
        super(standalone);
    }
    public override async onScriptChanged(script: string): WorkexPromise<void> {
        if (!this.editor.getFiles().includes(this.scriptFile)) {
            this.editor.openFile(this.scriptFile, script, "skybook");
        } else {
            this.editor.setFileContent(this.scriptFile, script);
        }
        return {};
    }
}
