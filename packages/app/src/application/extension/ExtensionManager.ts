import {
    addDarkSubscriber,
    addLocaleSubscriber,
    getLocale,
    isDark,
} from "@pistonite/pure/pref";
import type { Extension } from "@pistonite/skybook-api";
import type { ExtensionModule } from "@pistonite/skybook-api/client";

import { useSessionStore } from "self::application/store";
import { getExtensionAppHost } from "./ExtensionAppHost.ts";

/** Running instances of extensions */
const instances: Extension[] = [];

export const initExtensionManager = () => {
    useSessionStore.subscribe((curr, prev) => {
        if (prev.activeScript !== curr.activeScript) {
            instances.forEach((x) => {
                void x.onScriptChanged(curr.activeScript);
            });
        }
    });
    addDarkSubscriber((dark) => {
        instances.forEach((x) => {
            void x.onDarkModeChanged(dark);
        });
    }, false);
    addLocaleSubscriber((locale) => {
        instances.forEach((x) => {
            void x.onLocaleChanged(locale);
        });
    }, false);
};

/**
 * Registers an extension as running and connect it to the app.
 *
 * Returns a function to disconnect the extension from the app.
 */
export const connectExtensionToApp = (
    extension: ExtensionModule,
): (() => void) => {
    extension.onAppConnectionEstablished(getExtensionAppHost());

    void extension.onDarkModeChanged(isDark());
    void extension.onLocaleChanged(getLocale());
    void extension.onScriptChanged(useSessionStore.getState().activeScript);
    instances.push(extension);

    return () => {
        const index = instances.indexOf(extension);
        if (index >= 0) {
            instances.splice(index, 1);
        }
    };
};

export const openExtensionPopup = (id: string) => {
    console.error("Not implemented", id);
};
