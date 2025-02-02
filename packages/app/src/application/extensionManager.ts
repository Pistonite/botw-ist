import { addDarkSubscriber, addLocaleSubscriber } from "@pistonite/pure/pref";
import type { Extension } from "@pistonite/skybook-extension-api";
import { useApplicationStore } from "./store";

/** Running instances of extensions */
const instances: Extension[] = [];

export const initExtensionManager = () => {
    const store = useApplicationStore;
    store.subscribe((curr, prev) => {
        if (prev.script !== curr.script) {
            instances.forEach((x) => {
                void x.onScriptChanged(curr.script);
            });
        }
    });
};

/**
 * Registers an extension as running and connect it to the app.
 *
 * Returns a function to disconnect the extension from the app.
 */
export const connectExtensionToApp = (extension: Extension): (() => void) => {
    const unsubscribeDark = addDarkSubscriber((x) => {
        void extension.onDarkModeChanged(x);
    }, true);
    const unsubscribeLocale = addLocaleSubscriber((x) => {
        void extension.onLocaleChanged(x);
    }, true);
    void extension.onScriptChanged(useApplicationStore.getState().script);
    instances.push(extension);
    return () => {
        const index = instances.indexOf(extension);
        if (index >= 0) {
            instances.splice(index, 1);
        }
        unsubscribeLocale();
        unsubscribeDark();
    };
};

export const openExtensionPopup = (id: string) => {
    console.error("Not implemented", id);
};
