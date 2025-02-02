import type { Application } from "@pistonite/skybook-extension-api";
import { debounce } from "@pistonite/pure/sync";
import type { Result } from "@pistonite/pure/result";
import type { WorkexPromise } from "@pistonite/workex";
import { searchItemLocalized } from "skybook-localization";
import { getActorParam } from "skybook-item-system";

import { useApplicationStore } from "./store";

const setScript = debounce({
    fn: (script: string) => {
        useApplicationStore.setState({ script });
    },
    interval: 200,
});

export class ApplicationApi implements Application {
    public async getScript() {
        return { val: useApplicationStore.getState().script };
    }
    public async setScript(script: string) {
        // await setScript(script);
        // return {};
        useApplicationStore.setState({ script });
        return {};
    }
    public async resolveItem(
        query: string,
        localized: boolean,
        limit: number,
    ): WorkexPromise<Result<{ actor: string; cookEffect: number }[], string>> {
        if (localized) {
            const result = await searchItemLocalized(query, limit);
            if ("err" in result) {
                return { val: result };
            }
            // filter out upgraded armors
            const filtered = result.val.filter((item) => {
                return (
                    !item.actor.startsWith("Armor") ||
                    getActorParam(item.actor, "armorStarNum") === 1
                );
            });
            return { val: { val: filtered } };
        }
        return {
            err: {
                type: "Internal",
                message: "Not implemented yet",
            },
        };
    }
}
