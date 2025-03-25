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
import { ExtensionClient } from "@pistonite/skybook-api/interfaces/Extension.send";
import { bindExtensionAppHost } from "@pistonite/skybook-api/interfaces/ExtensionApp.recv";

/** Running instances of extensions */
const instances: Extension[] = [];
const popoutWindows: Window[] = [];

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
    // close all popout windows when app is closed
    window.addEventListener("pagehide", () => {
        const len = popoutWindows.length;
        for (let i = 0; i < len; i++) {
            try {
                popoutWindows[i].close();
            } catch(e) {
                console.error(e);
            }
        }
    });
};

/**
 * Registers an extension as running and connect it to the app.
 *
 * Returns a function to disconnect the extension from the app.
 */
export const connectExtensionToApp = (
    extension: ExtensionModule,
) => {
    extension.onAppConnectionEstablished(getExtensionAppHost());
    notifyAndPushInstance(extension);
};

const connectExtensionPopoutToApp = (
    serial: number,
    extension: Extension,
    extensionWindow: Window,
) => {
    const disconnect = () => {
        console.log(`[popout (${serial})] disconnecting extension`);
        try {
            if (!extensionWindow.closed) {
                extensionWindow.close();
            }
        } catch (e) {
            console.error(e);
        }
        const index = instances.indexOf(extension);
        if (index >= 0) {
            instances.splice(index, 1);
        }
        console.log(`[popout (${serial})] extension disconnected`);
    };
    // when user closes the popout window, disconnects the extension
    extensionWindow.addEventListener("pagehide", () => {
        disconnect();
    });
    notifyAndPushInstance(extension);
    console.log(`[popout (${serial})] extension connected to host app`);
};

const notifyAndPushInstance = (extension: Extension) => {
    void extension.onDarkModeChanged(isDark());
    void extension.onLocaleChanged(getLocale());
    void extension.onScriptChanged(useSessionStore.getState().activeScript);
    instances.push(extension);
};

const createChannel = () => {
    const subscribers1: ((x: unknown) => void)[] = [];
    const subscribers2: ((x: unknown) => void)[] = [];
    const addEventListener1 = (event: string, fn: (x: unknown) => void) => {
        if (event === "message") {
            const len = subscribers1.length;
            for (let i = 0; i < len; i++) {
                if (subscribers1[i] === fn) {
                    return;
                }
            }
            subscribers1.push(fn);
        }
    }
    const postMessage1 = (x: unknown) => {
        const len = subscribers1.length;
        for (let i = 0; i < len; i++) {
            subscribers1[i]({ data: x} );
        }
    }
    const addEventListener2 = (event: string, fn: (x: unknown) => void) => {
        if (event === "message") {
            const len = subscribers2.length;
            for (let i = 0; i < len; i++) {
                if (subscribers2[i] === fn) {
                    return;
                }
            }
            subscribers2.push(fn);
        }
    }
    const postMessage2 = (x: unknown) => {
        const len = subscribers2.length;
        for (let i = 0; i < len; i++) {
            subscribers2[i]({ data: x });
        }
    }
    return {
        a: {addEventListener: addEventListener1, postMessage: postMessage2},
        b: {addEventListener: addEventListener2, postMessage: postMessage1},
    }
}
    

let popoutSerial = 0;

export const openExtensionPopup = async (id: string) => {
    const serial = popoutSerial++;
    console.log(`opening extension popout: ${id} (serial: ${serial})`);
    const origin = window.location.origin;
    let url: string;
    if (import.meta.env.DEV) {
        console.log("[dev] using dev extension popout url")
        url = `${origin}/popout`;
    } else {
        const commitShort = import.meta.env.COMMIT.substring(0, 8);
        url = `${origin}/popout-${commitShort}`;
    }
    const urlobj = new URL(url);
    urlobj.searchParams.set("skybookHostOrigin", origin);
    urlobj.searchParams.set("skybookExtensionId", id);

    const extensionWindow = window.open(urlobj.href, "_blank", "popup,width=800,height=600");
    if (!extensionWindow) {
        console.error("failed to open extension popout window");
        return;
    }
    const {a, b} = createChannel();
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (extensionWindow as any).skybookChannel = b;
    console.log(`[popout ${id} (${serial})] establishing connection with popout window`);
    bindExtensionAppHost(getExtensionAppHost(), {worker: a});
    const extension = new ExtensionClient({worker: a});
    await extension.handshake().established();
    console.log(`[popout ${id} (${serial})] window connection established`);

    connectExtensionPopoutToApp(serial, extension, extensionWindow);
};
