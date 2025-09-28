/**
 * Extension manager module
 *
 * This tracks all running extension instances. This is running in the APPLICATION layer,
 * not Extensions!!! (And popouts don't link with this)
 */

import { addDarkSubscriber, addLocaleSubscriber, getLocale, isDark } from "@pistonite/pure/pref";
import { wxPopup } from "@pistonite/workex";
import { bytePosToCharPos } from "@pistonite/intwc";

import type { Extension } from "@pistonite/skybook-api";
import type { ExtensionModule } from "@pistonite/skybook-api/client";
import { skybookExtension } from "@pistonite/skybook-api/interfaces/Extension.bus.ts";

import { usePersistStore } from "./persist_store.ts";
import { useExtensionStore } from "./extension_store.ts";
import { useSessionStore } from "./session_store.ts";
import { getExtensionAppHost } from "./extension_host.ts";
import { devLog, log } from "self::util";

type ExtensionInstanceEntry = {
    id: string;
    instance: Extension;
    isPopout: boolean;
};

/** Running instances of extensions */
const instances: ExtensionInstanceEntry[] = [];

export const initExtensionManager = () => {
    usePersistStore.subscribe((curr, prev) => {
        if (
            curr.enableHighQualityIcons !== prev.enableHighQualityIcons ||
            curr.enableAnimations !== prev.enableAnimations
        ) {
            instances.forEach(({ instance }) => {
                void instance.onIconSettingsChanged(
                    curr.enableHighQualityIcons,
                    curr.enableAnimations,
                );
            });
        }
    });
    // we only optimize for script updates, since that's the most
    // often updated state
    useSessionStore.subscribe((curr, prev) => {
        if (prev.activeScript !== curr.activeScript || prev.bytePos !== curr.bytePos) {
            const charPos = bytePosToCharPos(curr.activeScript, curr.bytePos);
            const len = instances.length;
            const { currentPrimary, currentSecondary } = useExtensionStore.getState();
            for (let i = 0; i < len; i++) {
                // skip updates for local extensions that are not visible right now
                if (!instances[i].isPopout) {
                    const id = instances[i].id;
                    if (id !== currentPrimary && id !== currentSecondary) {
                        continue;
                    }
                }
                void instances[i].instance.onScriptChanged(curr.activeScript, charPos);
            }
        }
        if (prev.mode !== curr.mode) {
            instances.forEach(({ instance }) => {
                void instance.onAppModeChanged(curr.mode);
            });
        }
        if (prev.dragData !== curr.dragData) {
            instances.forEach(({ isPopout, instance }) => {
                if (!isPopout) {
                    return;
                }
                void instance.onItemDragChanged(curr.dragData);
            });
        }
    });
    // send updates to non-popout extension when switching extensions
    useExtensionStore.subscribe((curr, prev) => {
        if (curr.currentPrimary !== prev.currentPrimary && curr.currentPrimary) {
            sendEventsToLocalExtensionById(curr.currentPrimary);
        }
        if (
            curr.currentSecondary !== prev.currentSecondary &&
            curr.currentSecondary &&
            curr.currentSecondary !== curr.currentPrimary
        ) {
            sendEventsToLocalExtensionById(curr.currentSecondary);
        }
    });
    addDarkSubscriber((dark) => {
        instances.forEach(({ instance }) => {
            void instance.onDarkModeChanged(dark);
        });
    }, false);
    addLocaleSubscriber((locale) => {
        instances.forEach(({ instance }) => {
            void instance.onLocaleChanged(locale);
        });
    }, false);
};

/**
 * Registers a local (non-popout) extension as running and connect it to the app.
 */
export const connectLocalExtensionToApp = (id: string, extension: ExtensionModule) => {
    extension.onAppConnectionEstablished(getExtensionAppHost());
    notifyAndPushInstance(id, extension, false);
};

const notifyAndPushInstance = (id: string, extension: Extension, isPopout: boolean) => {
    sendEventsToExtension(extension);
    instances.push({ id, instance: extension, isPopout });
};

/** Send updates to the extension by id */
const sendEventsToLocalExtensionById = (id: string) => {
    const len = instances.length;
    for (let i = 0; i < len; i++) {
        if (instances[i].isPopout) {
            continue;
        }
        if (instances[i].id !== id) {
            continue;
        }
        sendEventsToExtension(instances[i].instance);
        break;
    }
};

const sendEventsToExtension = (extension: Extension) => {
    void extension.onDarkModeChanged(isDark());
    void extension.onLocaleChanged(getLocale());
    const { enableHighQualityIcons, enableAnimations } = usePersistStore.getState();
    void extension.onIconSettingsChanged(enableHighQualityIcons, enableAnimations);
    const { activeScript, bytePos, mode } = useSessionStore.getState();
    void extension.onAppModeChanged(mode);
    const charPos = bytePosToCharPos(activeScript, bytePos);
    void extension.onScriptChanged(activeScript, charPos);
};

/** Open an extension in its current configured location */
export const openExtension = async (id: string) => {
    const isCustom = id.startsWith("custom-");
    if (isCustom) {
        await openExtensionPopup(id);
        return;
    }
    const { primaryIds, secondaryIds, open } = useExtensionStore.getState();
    if (primaryIds.includes(id)) {
        open(id, "primary");
        return;
    }
    if (secondaryIds.includes(id)) {
        open(id, "secondary");
        return;
    }
    await openExtensionPopup(id);
};

let popoutSerial = 0;

export const openExtensionPopup = async (id: string) => {
    const serial = popoutSerial++;
    const PREFIX = `[popout ${id} (${serial})]`;
    log.info(`${PREFIX}\nopening extension popout`);
    const isCustom = id.startsWith("custom-");
    const origin = window.location.origin;

    let url: string;
    if (isCustom) {
        url = id.substring(7);
    } else {
        if (import.meta.env.DEV) {
            devLog.info("using dev extension popout url");
            url = `${origin}/popout`;
        } else {
            const commitShort = import.meta.env.COMMIT.substring(0, 8);
            url = `${origin}/popout-${commitShort}`;
        }
    }

    const urlobj = new URL(url);
    urlobj.searchParams.set("skybookHostOrigin", origin);
    if (!isCustom) {
        urlobj.searchParams.set("skybookExtensionId", id);
    }

    const appHost = getExtensionAppHost();

    const result = await wxPopup(urlobj.href, {
        width: 800,
        height: 600,
    })({
        extension: skybookExtension(appHost),
    });
    if (result.err) {
        log.error(`${PREFIX}\nfailed to open extension popout window`);
        log.error(result.err);
        return;
    }

    const {
        connection,
        protocols: { extension },
    } = result.val;
    log.info(`${PREFIX}\nwindow connection established`);
    // disconnect the extension when popout window is closed
    connection.onClose(() => {
        for (let i = 0; i < instances.length; i++) {
            if (instances[i].instance === extension) {
                instances.splice(i, 1);
                break;
            }
        }
        log.info(`${PREFIX}\nextension disconnected`);
    });

    notifyAndPushInstance(id, extension, true);
};
