import type { Result } from "@pistonite/pure/result";
import { type AsyncErc, makeAsyncErcType } from "@pistonite/pure/memory";
import { once } from "@pistonite/pure/sync";

import type {
    RuntimeInitOutput,
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    ItemSearchResult,
    MaybeAborted,
    ParserErrorReport,
    RuntimeInitError,
    CustomImageInitParams,
} from "@pistonite/skybook-api";

import type { Pwr, WorkerError } from "./Error.ts";

export type QuotedItemResolverFn = (
    query: string,
) => Promise<ItemSearchResult | undefined | null>;

export interface NativeApi {
    /** Initialize the runtime with the given image info */
    initRuntime(
        customImage: Uint8Array | undefined,
        customImageParams: CustomImageInitParams | undefined,
    ): Pwr<Result<RuntimeInitOutput, RuntimeInitError>>;

    // === item api ===
    resolveItemIdent: (query: string) => Pwr<ItemSearchResult[]>;

    // === parsing api ===

    /** Parses the script, returns a ptr to the parse output (that must be freed) */
    parseScript(
        script: string,
        resolveQuotedItem: QuotedItemResolverFn,
    ): Promise<Result<number, WorkerError>>;
    /**
     * Parse the semantics of the script in the given range
     *
     * The returned vector is triplets of (start, length, semantic token)
     */
    parseScriptSemantic(
        script: string,
        start: number,
        end: number,
    ): Promise<Result<Uint32Array, WorkerError>>;
    /** Get the errors from the parse output. Does not consume the ptr */
    getParserErrors(
        ptr: number,
    ): Promise<Result<ParserErrorReport[], WorkerError>>;
    /** Get number of steps in the parse output. Does not consume the ptr */
    getStepCount(ptr: number): Promise<Result<number, WorkerError>>;
    /**
     * Get the step index from the byte position in the script in the parse output.
     *
     * Returns 0 if the steps are empty. Does not consume the ptr.
     */
    getStepFromPos(
        ptr: number,
        bytePos: number,
    ): Promise<Result<number, WorkerError>>;

    // === run/task api ===

    /** Make a new task handle and returns the ptr to it (that must be freed) */
    makeTaskHandle(): Promise<Result<number, WorkerError>>;
    /**
     * Request aborting a task
     *
     * Consumes the task handle ptr.
     */
    abortTask(ptr: number): void;
    /**
     * Take the parse output and execute it.
     *
     * The handle can be used to abort the run
     *
     * Consumes both pointers. Returns a ptr to the run output (that must be freed)
     */
    runParsed(
        parsedOutputPtr: number,
        taskHandlePtr: number,
    ): Pwr<MaybeAborted<number>>;
    /**
     * Get the Pouch inventory view for the given byte position in the script.
     * Does not consume either ptr.
     * TODO: error type
     */
    getPouchList(
        runOutputPtr: number,
        parseOutputPtr: number,
        bytePos: number,
    ): Promise<Result<InvView_PouchList, WorkerError>>;
    /**
     * Get the GDT inventory view for the given byte position in the script.
     * Does not consume either ptr.
     * TODO: error type
     */
    getGdtInventory(
        runOutputPtr: number,
        parseOutputPtr: number,
        bytePos: number,
    ): Promise<Result<InvView_Gdt, WorkerError>>;
    /**
     * Get the overworld items for the given byte position in the script
     * Does not consume either ptr.
     * TODO: error type
     */
    getOverworldItems(
        runOutputPtr: number,
        parseOutputPtr: number,
        bytePos: number,
    ): Promise<Result<InvView_Overworld, WorkerError>>;

    // === ref counting api ===

    addRefNativeHandle(ptr: number): Promise<number>;
    freeNativeHandle(ptr: number): Promise<void>;
    addRefParseOutput(ptr: number): Promise<number>;
    freeParseOutput(ptr: number): Promise<void>;
    addRefRunOutput(ptr: number): Promise<number>;
    freeRunOutput(ptr: number): Promise<void>;
}

const NativeHandle = Symbol("NativeHandle");
export type NativeHandle = typeof NativeHandle;
let makeNativeHandleErcImpl: (
    ptr: number | undefined,
) => AsyncErc<NativeHandle>;

const RunOutput = Symbol("RunOutput");
export type RunOutput = typeof RunOutput;
let makeRunOutputErcImpl: (ptr: number | undefined) => AsyncErc<RunOutput>;

const ParseOutput = Symbol("ParseOutput");
export type ParseOutput = typeof ParseOutput;
let makeParseOutputErcImpl: (ptr: number | undefined) => AsyncErc<ParseOutput>;

export const initExternalRefCountTypes = once({
    fn: (api: NativeApi) => {
        makeNativeHandleErcImpl = makeAsyncErcType<NativeHandle, number>({
            marker: NativeHandle,
            free: (ptr: number) => api.freeNativeHandle(ptr),
            addRef: (ptr: number) => api.addRefNativeHandle(ptr),
        });
        makeParseOutputErcImpl = makeAsyncErcType<ParseOutput, number>({
            marker: ParseOutput,
            free: (ptr: number) => api.freeParseOutput(ptr),
            addRef: (ptr: number) => api.addRefParseOutput(ptr),
        });
        makeRunOutputErcImpl = makeAsyncErcType<RunOutput, number>({
            marker: RunOutput,
            free: (ptr: number) => api.freeRunOutput(ptr),
            addRef: (ptr: number) => api.addRefRunOutput(ptr),
        });
    },
});

export const makeNativeHandleErc = (ptr: number | undefined) => {
    if (!makeNativeHandleErcImpl) {
        throw new Error("Erc types not initialized");
    }
    return makeNativeHandleErcImpl(ptr);
};

export const makeParseOutputErc = (ptr: number | undefined) => {
    if (!makeParseOutputErcImpl) {
        throw new Error("Erc types not initialized");
    }
    return makeParseOutputErcImpl(ptr);
};

export const makeRunOutputErc = (ptr: number | undefined) => {
    if (!makeRunOutputErcImpl) {
        throw new Error("Erc types not initialized");
    }
    return makeRunOutputErcImpl(ptr);
};
