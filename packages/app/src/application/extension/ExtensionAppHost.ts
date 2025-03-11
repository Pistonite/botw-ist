import {
    charPosToBytePos,
    createBytePosToCharPosArray,
} from "@pistonite/intwc";
import type { Result } from "@pistonite/pure/result";
import type { WorkexPromise } from "@pistonite/workex";

import type { Diagnostic, ExtensionApp } from "@pistonite/skybook-api";
import type { RuntimeClient } from "@pistonite/skybook-api/sides/app";
import {
    searchItemLocalized,
    translateParserError,
} from "skybook-localization";
import { getActorParam } from "skybook-item-system";

import { useSessionStore } from "self::application/store";

export const createExtensionAppHost = (
    runtime: RuntimeClient,
): ExtensionApp => {
    return new ExtensionAppHost(runtime);
};

class ExtensionAppHost implements ExtensionApp {
    constructor(private runtime: RuntimeClient) {}

    public async getScript() {
        return { val: useSessionStore.getState().activeScript };
    }
    public async setScript(script: string) {
        useSessionStore.getState().setActiveScript(script);
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

    public async provideParserDiagnostics(
        script: string,
    ): WorkexPromise<Diagnostic[]> {
        const result = await this.runtime.getParserDiagnostics(script);
        if (result.err) {
            return result;
        }
        const bytePosToCharPos = createBytePosToCharPosArray(script);
        const diagnostics = result.val.map(({ span, error, isWarning }) => {
            const [start, end] = span;
            return {
                message: translateParserError(error),
                isWarning,
                start: bytePosToCharPos[start],
                end: bytePosToCharPos[end],
            };
        });
        return { val: diagnostics };
    }

    public async provideSemanticTokens(
        script: string,
        start: number,
        end: number,
    ): WorkexPromise<Uint32Array> {
        const tokens = await this.runtime.getSemanticTokens(
            script,
            charPosToBytePos(script, start),
            charPosToBytePos(script, end),
        );
        if (tokens.err) {
            return tokens;
        }
        // convert byte positions to character positions
        const bytePosToCharPos = createBytePosToCharPosArray(script);
        for (let i = 0; i < tokens.val.length; i += 3) {
            const byteStart = tokens.val[i];
            const byteLength = tokens.val[i + 1];
            const byteEnd = byteStart + byteLength;
            const charStart = bytePosToCharPos[byteStart];
            const charEnd = bytePosToCharPos[byteEnd];
            const charLength = charEnd - charStart;
            tokens.val[i] = charStart;
            tokens.val[i + 1] = charLength;
        }
        return { val: tokens.val };
    }
}
