import { once } from "@pistonite/pure/sync";

import type { FirstPartyExtension } from "./FirstParty.ts";
import { ItemExplorerExtension } from "./item-explorer/extindex.tsx";

const extensionInstances = new Map<
    string,
    () => Promise<FirstPartyExtension | undefined>
>();

export type ConnectExtensionFn = (
    extension: FirstPartyExtension,
) => Promise<() => void> | (() => void);

export const getExtension = async (
    id: string,
    standalone: boolean,
    connect: ConnectExtensionFn,
): Promise<FirstPartyExtension | undefined> => {
    const existing = extensionInstances.get(id);
    if (existing) {
        return await existing();
    }
    const creator = once({
        fn: async () => {
            const instance = await createExtensionInstance(id, standalone);
            if (!instance) {
                return undefined;
            }
            void (await connect(instance));
            return instance;
        },
    });
    extensionInstances.set(id, creator);
    return creator();
};

export const createExtensionInstance = async (
    id: string,
    standalone: boolean,
): Promise<FirstPartyExtension | undefined> => {
    console.log(`creating extension instance: ${id}`);
    switch (id) {
        case "editor": {
            const { EditorExtension } = await import("./editor/extindex.tsx");
            return new EditorExtension(standalone);
        }
        case "item-explorer": {
            return new ItemExplorerExtension(standalone);
        }
        case "stub1": {
            const { Stub1Extension } = await import("./Stub1");
            return new Stub1Extension();
        }
        default: {
            console.error(`unknown extension: ${id}`);
            return undefined;
        }
    }
};
