import { charPosToBytePos, createBytePosToCharPosArray } from "@pistonite/intwc";
import type { Result } from "@pistonite/pure/result";
import type { WxPromise } from "@pistonite/workex";
import { v4 as makeUUID } from "uuid";

import type {
    Diagnostic,
    Runtime,
    ExtensionApp,
    ErrorReport,
    MaybeAborted,
    RuntimeViewError,
    InvView_PouchList,
    InvView_Gdt,
    InvView_Overworld,
    ItemDragData,
} from "@pistonite/skybook-api";
import {
    searchItemLocalized,
    translateParserError,
    translateRuntimeError,
    translateUI,
} from "skybook-localization";
import { getActorParam } from "@pistonite/skybook-itemsys";

import { useSessionStore } from "./session_store.ts";

/**
 * This is the host that handles function calls from Extensions using the ExtensionApp API
 */
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
    private taskIdMap: Map<string, string[]>;

    constructor(private runtime: Runtime) {
        this.taskIdMap = new Map();
    }
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
            if ("err" in result && result.err) {
                const { tag } = result.err;
                const message = translateUI("item_explorer.search_error.unknown_tag", { tag });
                return { val: { err: message } };
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

    public async provideParserDiagnostics(script: string): WxPromise<Diagnostic[]> {
        const result = await this.runtime.getParserDiagnostics(script);
        if (result.err) {
            return result;
        }
        return {
            val: errorReportsToDiagnostics(script, result.val, translateParserError),
        };
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

    public async getStepFromCharPos(
        inputScript: string | undefined,
        charPos: number | undefined,
    ): WxPromise<number> {
        const [script, bytePos] = convertScriptAndCharPosArg(inputScript, charPos);
        return await this.runtime.getStepFromPos(script, bytePos);
    }

    public async getStepBytePositions(script: string): WxPromise<Uint32Array> {
        return await this.runtime.getStepBytePositions(script);
    }

    public async getStepCharPositions(script: string): WxPromise<Uint32Array> {
        const bytePositions = await this.runtime.getStepBytePositions(script);
        if (bytePositions.err) {
            return bytePositions;
        }
        const array = bytePositions.val;
        const len = array.length;
        const bytePosToCharPos = createBytePosToCharPosArray(script);
        for (let i = 0; i < len; i++) {
            array[i] = bytePosToCharPos[array[i]];
        }
        return { val: array };
    }

    public async requestNewTaskIds(uniqueId: string, count: number): WxPromise<string[]> {
        const oldTaskIds = this.taskIdMap.get(uniqueId);
        const newTaskIds = Array.from({ length: count }).map(() => makeUUID());
        this.taskIdMap.set(uniqueId, newTaskIds);
        if (oldTaskIds?.length) {
            void this.cancelRuntimeTasks(oldTaskIds);
        }
        return { val: newTaskIds };
    }

    public async cancelRuntimeTasks(taskId: string[]): WxPromise<void> {
        const promises = taskId.map((x) => this.runtime.abortTask(x));
        // eat all the errors
        await Promise.all(promises);
        return {};
    }

    public async provideRuntimeDiagnostics(
        script: string,
        taskId: string,
    ): WxPromise<MaybeAborted<Diagnostic[]>> {
        return await this.providePartialRuntimeDiagnostics(script, taskId, -1);
    }

    public async providePartialRuntimeDiagnostics(
        script: string,
        taskId: string,
        bytePos: number,
    ): WxPromise<MaybeAborted<Diagnostic[]>> {
        const result = await this.runtime.getRuntimeDiagnostics(script, taskId, bytePos);
        return mapMaybeAbortedResult(result, (value) => {
            return errorReportsToDiagnostics(script, value, translateRuntimeError);
        });
    }

    public async getPouchList(
        taskId: string,
        inputScript: string | undefined,
        charPos: number | undefined,
    ): WxPromise<MaybeAborted<Result<InvView_PouchList, RuntimeViewError>>> {
        const [script, bytePos] = convertScriptAndCharPosArg(inputScript, charPos);
        return await this.runtime.getPouchList(script, taskId, bytePos);
    }

    public async getGdtInventory(
        taskId: string,
        inputScript: string | undefined,
        charPos: number | undefined,
    ): WxPromise<MaybeAborted<Result<InvView_Gdt, RuntimeViewError>>> {
        const [script, bytePos] = convertScriptAndCharPosArg(inputScript, charPos);
        return await this.runtime.getGdtInventory(script, taskId, bytePos);
    }

    public async getOverworldItems(
        taskId: string,
        inputScript: string | undefined,
        charPos: number | undefined,
    ): WxPromise<MaybeAborted<Result<InvView_Overworld, RuntimeViewError>>> {
        const [script, bytePos] = convertScriptAndCharPosArg(inputScript, charPos);
        return await this.runtime.getOverworldItems(script, taskId, bytePos);
    }

    public async getCrashInfo(
        taskId: string,
        inputScript: string | undefined,
        charPos: number | undefined,
    ): WxPromise<MaybeAborted<string>> {
        const [script, bytePos] = convertScriptAndCharPosArg(inputScript, charPos);
        return await this.runtime.getCrashInfo(script, taskId, bytePos);
    }

    public async getSaveNames(
        taskId: string,
        inputScript: string | undefined,
        charPos: number | undefined,
    ): WxPromise<MaybeAborted<string[]>> {
        const [script, bytePos] = convertScriptAndCharPosArg(inputScript, charPos);
        return await this.runtime.getSaveNames(script, taskId, bytePos);
    }

    public async getSaveInventory(
        taskId: string,
        inputScript: string | undefined,
        charPos: number | undefined,
        name: string | undefined,
    ): WxPromise<MaybeAborted<Result<InvView_Gdt, RuntimeViewError>>> {
        const [script, bytePos] = convertScriptAndCharPosArg(inputScript, charPos);
        return await this.runtime.getSaveInventory(script, taskId, bytePos, name);
    }

    public async handleItemDrag(data: ItemDragData | undefined): WxPromise<void> {
        const { setDragData } = useSessionStore.getState();
        setDragData(data);
        return {};
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

const mapMaybeAbortedResult = <TIn, TOut>(
    result: Awaited<WxPromise<MaybeAborted<TIn>>>,
    fn: (t: TIn) => TOut,
): Awaited<WxPromise<MaybeAborted<TOut>>> => {
    if (result.err) {
        return result;
    }
    if (result.val.type === "Aborted") {
        return { val: { type: "Aborted" } };
    }
    return {
        val: {
            type: "Ok",
            value: fn(result.val.value),
        },
    };
};

/**
 * Convert script and charPos arguments from calls from extensions
 * to script and bytePos
 */
const convertScriptAndCharPosArg = (
    script: string | undefined,
    charPos: number | undefined,
): [string, number] => {
    if (script === undefined) {
        script = useSessionStore.getState().activeScript;
        if (charPos === undefined) {
            const bytePos = useSessionStore.getState().bytePos;
            return [script, bytePos];
        }
        return [script, charPosToBytePos(script, charPos)];
    }
    if (charPos === undefined) {
        charPos = 0;
    }
    return [script, charPosToBytePos(script, charPos)];
};
