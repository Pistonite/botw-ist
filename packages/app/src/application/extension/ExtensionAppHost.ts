import {
    charPosToBytePos,
    createBytePosToCharPosArray,
} from "@pistonite/intwc";
import type { Result } from "@pistonite/pure/result";
import type { WxPromise } from "@pistonite/workex";

import type {
    Diagnostic,
    Runtime,
    ExtensionApp,
    ErrorReport,
} from "@pistonite/skybook-api";
import {
    searchItemLocalized,
    translateParserError,
    translateRuntimeError,
} from "skybook-localization";
import { getActorParam } from "skybook-item-system";

import { useSessionStore } from "self::application/store";

let theAppHost: ExtensionApp | undefined = undefined;

export const initExtensionAppHost = (runtime: Runtime) => {
    theAppHost = new ExtensionAppHost(runtime);
};

export const getExtensionAppHost = () => {
    if (!theAppHost) {
        throw new Error("Extension app host not initialized");
    }
    return theAppHost;
};

class ExtensionAppHost implements ExtensionApp {
    constructor(private runtime: Runtime) {}

    public async getScript() {
        return { val: useSessionStore.getState().activeScript };
    }
    public async setScript(script: string, charPos: number) {
        useSessionStore.getState().setActiveScript(script, charPos);
        return {};
    }
    public async resolveItem(
        query: string,
        localized: boolean,
        limit: number,
    ): WxPromise<Result<{ actor: string; cookEffect: number }[], string>> {
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
    ): WxPromise<Diagnostic[]> {
        const result = await this.runtime.getParserDiagnostics(script);
        if (result.err) {
            return result;
        }
        return {
            val: errorReportsToDiagnostics(
                script,
                result.val,
                translateParserError,
            ),
        };
    }

    public async provideRuntimeDiagnostics(
        script: string,
        taskId: string,
    ): WxPromise<Diagnostic[]> {
        const result = await this.runtime.getRuntimeDiagnostics(script, taskId);
        if (result.err) {
            return result;
        }
        return {
            val: errorReportsToDiagnostics(
                script,
                result.val,
                translateRuntimeError,
            ),
        };
    }

    public async cancelRuntimeDiagnosticsRequest(
        taskId: string,
    ): WxPromise<void> {
        return await this.runtime.abortTask(taskId);
    }

    public async provideSemanticTokens(
        script: string,
        start: number,
        end: number,
    ): WxPromise<Uint32Array> {
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

const errorReportsToDiagnostics = <T>(
    script: string,
    reports: ErrorReport<T>[],
    translator: (error: T) => string,
): Diagnostic[] => {
    const bytePosToCharPos = createBytePosToCharPosArray(script);
    return reports.map(({ span, error, isWarning }) => {
        const [start, end] = span;
        return {
            message: translator(error),
            isWarning,
            start: bytePosToCharPos[start],
            end: bytePosToCharPos[end],
        };
    });
};
