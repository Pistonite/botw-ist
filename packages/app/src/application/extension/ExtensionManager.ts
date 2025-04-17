import {
    addDarkSubscriber,
    addLocaleSubscriber,
    getLocale,
    isDark,
} from "@pistonite/pure/pref";
import { wxPopup } from "@pistonite/workex";

import type { Extension } from "@pistonite/skybook-api";
import type { ExtensionModule } from "@pistonite/skybook-api/client";
import { skybookExtension } from "@pistonite/skybook-api/interfaces/Extension.bus.ts";

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
export const connectExtensionToApp = (extension: ExtensionModule) => {
    extension.onAppConnectionEstablished(getExtensionAppHost());
    notifyAndPushInstance(extension);
};

const notifyAndPushInstance = (extension: Extension) => {
    void extension.onDarkModeChanged(isDark());
    void extension.onLocaleChanged(getLocale());
    void extension.onScriptChanged(useSessionStore.getState().activeScript);
    instances.push(extension);
};

let popoutSerial = 0;

export const openExtensionPopup = async (id: string) => {
    const serial = popoutSerial++;
    console.log(`opening extension popout: ${id} (serial: ${serial})`);
    const origin = window.location.origin;
    let url: string;
    if (import.meta.env.DEV) {
        console.log("[dev] using dev extension popout url");
        url = `${origin}/popout`;
    } else {
        const commitShort = import.meta.env.COMMIT.substring(0, 8);
        url = `${origin}/popout-${commitShort}`;
    }
    const urlobj = new URL(url);
    urlobj.searchParams.set("skybookHostOrigin", origin);
    urlobj.searchParams.set("skybookExtensionId", id);

    const appHost = getExtensionAppHost();

    const result = await wxPopup(urlobj.href, {
        width: 800,
        height: 600,
    })({
        extension: skybookExtension(appHost),
    });
    if (result.err) {
        console.error("failed to open extension popout window");
        return;
    }

    const {
        connection,
        protocols: { extension },
    } = result.val;
    console.log(`[popout ${id} (${serial})] window connection established`);
    // disconnect the extension when popout window is closed
    connection.onClose(() => {
        const index = instances.indexOf(extension);
        if (index >= 0) {
            instances.splice(index, 1);
        }
        console.log(`[popout (${serial})] extension disconnected`);
    });

    notifyAndPushInstance(extension);
};
