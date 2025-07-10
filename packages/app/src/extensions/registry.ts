import { once } from "@pistonite/pure/sync";

import { ItemExplorerExtension } from "./item-explorer/extindex.tsx";
import { CrashViewerExtension } from "./crash-viewer/extindex.tsx";

import { extLog, type FirstPartyExtension } from "self::util";

const extensionInstances = new Map<
    string,
    () => Promise<FirstPartyExtension | undefined>
>();

export type ConnectExtensionFn = (
    id: string,
    extension: FirstPartyExtension,
) => Promise<void> | void;

/**
 * Get or create an instance of a FirstPartyExtension, and connect it to the app
 * if newly created
 */
export const getExtension = async (
    id: string,
    standalone: boolean,
    connect: ConnectExtensionFn,
): Promise<FirstPartyExtension | undefined> => {
    const existing = await getLoadedExtension(id);
    if (existing) {
        return existing;
    }
    const creator = once({
        fn: async () => {
            const instance = await createExtensionInstance(id, standalone);
            if (!instance) {
                return undefined;
            }
            void (await connect(id, instance));
            return instance;
        },
    });
    extensionInstances.set(id, creator);
    return creator();
};

/** Get the created instance of a FirstPartyExtension by its id */
export const getLoadedExtension = async (
    id: string,
): Promise<FirstPartyExtension | undefined> => {
    const existing = extensionInstances.get(id);
    if (existing) {
        return await existing();
    }
    return undefined;
};

export const createExtensionInstance = async (
    id: string,
    standalone: boolean,
): Promise<FirstPartyExtension | undefined> => {
    extLog.info(`creating extension instance: ${id}`);
    switch (id) {
        case "editor": {
            const { EditorExtension } = await import("./editor/extindex.tsx");
            return new EditorExtension(standalone);
        }
        case "item-explorer": {
            return new ItemExplorerExtension(standalone);
        }
        case "crash-viewer": {
            return new CrashViewerExtension(standalone);
        }
        default: {
            extLog.error(`unknown extension: ${id}`);
            return undefined;
        }
    }
};
