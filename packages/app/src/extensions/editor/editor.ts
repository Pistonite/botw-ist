import { CodeEditorApi } from "@pistonite/intwc";
import { setDark, setLocale } from "@pistonite/pure/pref";
import { Extension } from "@pistonite/skybook-extension-api";
import { WorkexPromise } from "workex";


export class EditorExtension implements Extension {

    constructor(
        private scriptFile: string,
        private standalone: boolean, private editor: CodeEditorApi) {
    }
    public async onDarkModeChanged(dark: boolean): WorkexPromise<void> {
        if (this.standalone) {
            setDark(dark);
        }
        return {};
    }
    public async onLocaleChanged(locale: string): WorkexPromise<void> {
        if (this.standalone) {
            setLocale(locale);
        }
        return {};
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
