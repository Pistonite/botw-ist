import { setDark, setLocale } from "@pistonite/pure/pref";
import type { WorkexPromise } from "@pistonite/workex";

import type { ExtensionApp } from "@pistonite/skybook-api";
import type { ExtensionModule } from "@pistonite/skybook-api/client";

/**
 * Adapter for the first-party extensions with common
 * functionality for running both in standalone and embedded mode.
 */
export class FirstPartyExtensionAdapter implements ExtensionModule {
    protected app: ExtensionApp | undefined = undefined;
    constructor(private standalone: boolean) {}
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
    public async onScriptChanged(_script: string): WorkexPromise<void> {
        return {};
    }
    public onAppConnectionEstablished(app: ExtensionApp): void {
        this.app = app;
    }
}

export type FirstPartyExtension = ExtensionModule & {
    get Component(): React.FC;
};
