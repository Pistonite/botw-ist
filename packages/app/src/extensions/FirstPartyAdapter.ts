import type { Extension } from "@pistonite/skybook-extension-api";
import { setDark, setLocale } from "@pistonite/pure/pref";
import type { WorkexPromise } from "@pistonite/workex";

/**
 * Adapter for the first-party extensions with common
 * functionality for running both in standalone and embedded mode.
 */
export class FirstPartyExtensionAdapter implements Extension {
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
}
