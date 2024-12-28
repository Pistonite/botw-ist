import { Extension } from "@pistonite/skybook-extension-api";
import { WorkexPromise } from "@pistonite/skybook-extension-api/workex";


export class EditorExtension implements Extension {
    onStart(): WorkexPromise<void> {
        throw new Error("Method not implemented.");
    }
    onStop(): WorkexPromise<void> {
        throw new Error("Method not implemented.");
    }
    onDarkModeChanged(dark: boolean): WorkexPromise<void> {
        throw new Error("Method not implemented.");
    }
    onLocaleChanged(locale: string): WorkexPromise<void> {
        throw new Error("Method not implemented.");
    }
    async getLocalizedName(locale: string): WorkexPromise<string> {
        return { val: "Editor" };
    }
    onScriptChanged(script: string): WorkexPromise<void> {
        throw new Error("Method not implemented.");
    }
}
