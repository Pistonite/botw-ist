import type { Application } from "@pistonite/skybook-extension-api";
import type { Result } from "@pistonite/pure/result";
import type { WorkexPromise } from "@pistonite/workex";
import { searchItemLocalized } from "skybook-localization";
import { getActorParam } from "skybook-item-system";
import type { RuntimeApiClient } from "skybook-runtime-api/sides/app";

import { useApplicationStore } from "./store";

export class ApplicationApi implements Application {
    constructor(private runtime: RuntimeApiClient) {}

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

        const result = await this.runtime.resolveItemIdent(query);
        if (result.err) {
            return result;
        }
        return { val: { val: result.val } };
    }
}
