import type { RuntimeApp } from "@pistonite/skybook-api";
import { type Delegate, hostFromDelegate } from "@pistonite/workex";
import { searchItemLocalized } from "skybook-localization";

export const createRuntimeAppHost = (): RuntimeApp => {
    const appHostDelegate = {
        resolveQuotedItem: async (query) => {
            const result = await searchItemLocalized(query, 1);
            if ("err" in result || !result.val.length) {
                return undefined;
            }
            const item = result.val[0];
            return item;
        },

        onRunCompleted: async () => {
            return;
        },
    } satisfies Delegate<RuntimeApp>;

    return hostFromDelegate(appHostDelegate);
};
