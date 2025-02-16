import type { Diagnostic, ExtensionApp } from "@pistonite/skybook-api";
import type { RuntimeClient } from "@pistonite/skybook-api/sides/app";
import type { Result } from "@pistonite/pure/result";
import type { WorkexPromise } from "@pistonite/workex";
import { searchItemLocalized, translateParserError } from "skybook-localization";
import { getActorParam } from "skybook-item-system";

import { useApplicationStore } from "./store";

export const createExtensionAppHost = (runtime: RuntimeClient): ExtensionApp => {
    return new ExtensionAppHost(runtime);
}

class ExtensionAppHost implements ExtensionApp {
    constructor(private runtime: RuntimeClient) {}

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

    public async provideParserDiagnostics(script: string): WorkexPromise<Diagnostic[]> {
        const result = await this.runtime.getParserDiagnostics(script);
        if (result.err) {
            return result;
        }
        const diagnostics = result.val.map(({span, error, isWarning}) => {
            const [start, end] = span;
            return {
                message: translateParserError(error),
                isWarning,
                start,
                end,
            }
        });
        return { val: diagnostics };
    }
}
