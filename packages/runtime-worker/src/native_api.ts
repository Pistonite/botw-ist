import { makeEmpType } from "@pistonite/pure/memory";
import type { Result } from "@pistonite/pure/result";

import type {
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    ItemSearchResult,
    MaybeAborted,
    ErrorReport,
    ParserError,
    RuntimeInitError,
    RuntimeInitParams,
    RuntimeViewError,
    RuntimeError,
} from "@pistonite/skybook-api";

import type { Pwr } from "./error.ts";

export type QuotedItemResolverFn = (
    query: string,
) => Promise<ItemSearchResult | undefined | null>;

export type RuntimeInitOutput = {
    /** Custom image version initialized, should be in the form of "X.X.X" */
    gameVersion: string;
};

/** API bindings for calls into native runtime, plus mixin functions used by the worker */
export interface NativeApi<TPtr>
    extends NativeApiFunctions<TPtr>,
        NativeEmpFactory<TPtr> {}

/** API bindings for calls into native runtime */
export interface NativeApiFunctions<TPtr> {
    /** Initialize the runtime with the given image info */
    initRuntime(
        customImage: Uint8Array | undefined,
        customImageParams: RuntimeInitParams | undefined,
    ): Pwr<Result<RuntimeInitOutput, RuntimeInitError>>;

    // === item api ===
    resolveItemIdent: (query: string) => Pwr<ItemSearchResult[]>;

    // === parsing api ===

    /** Parses the script, returns a ptr to the parse output (that must be freed) */
    parseScript(
        script: string,
        resolveQuotedItem: QuotedItemResolverFn,
    ): Pwr<TPtr>;
    /**
     * Parse the semantics of the script in the given range
     *
     * The returned vector is triplets of (start, length, semantic token)
     */
    parseScriptSemantic(
        script: string,
        start: number,
        end: number,
    ): Pwr<Uint32Array>;
    /** Get the errors from the parse output. Does not consume the ptr */
    getParserErrors(ptr: TPtr): Pwr<ErrorReport<ParserError>[]>;
    /** Get number of steps in the parse output. Does not consume the ptr */
    getStepCount(ptr: TPtr): Pwr<number>;
    /**
     * Get the step index from the byte position in the script in the parse output.
     *
     * Returns 0 if the steps are empty. Does not consume the ptr.
     */
    getStepFromPos(ptr: TPtr, bytePos: number): Pwr<number>;

    /** Get the start byte positions for each step, does not consume the ptr */
    getStepBytePositions(ptr: TPtr): Pwr<Uint32Array>;

    // === run/task api ===

    /** Make a new task handle and returns the ptr to it (that must be freed) */
    makeTaskHandle(): Pwr<TPtr>;

    /**
     * Request aborting a task
     *
     * Consumes the task handle ptr.
     */
    abortTask(ptr: TPtr): void;

    /**
     * Take the parse output and execute it.
     *
     * The handle can be used to abort the run
     *
     * Consumes both pointers. Returns a ptr to the run output (that must be freed)
     */
    runParsed(
        parsedOutputPtr: TPtr,
        taskHandlePtr: TPtr | undefined,
        notifyFn: (upToBytePos: number, outputPtr: TPtr) => Promise<void>,
    ): Pwr<MaybeAborted<TPtr>>;

    /** Get the errors from the run output. Does not consume the ptr */
    getRunErrors(ptr: TPtr): Pwr<ErrorReport<RuntimeError>[]>;

    /**
     * Get the Pouch inventory view for the given byte position in the script.
     * Does not consume either ptr.
     */
    getPouchList(
        runOutputPtr: TPtr,
        parseOutputPtr: TPtr,
        bytePos: number,
    ): Pwr<Result<InvView_PouchList, RuntimeViewError>>;

    /**
     * Get the GDT inventory view for the given byte position in the script.
     * Does not consume either ptr.
     */
    getGdtInventory(
        runOutputPtr: TPtr,
        parseOutputPtr: TPtr,
        bytePos: number,
    ): Pwr<Result<InvView_Gdt, RuntimeViewError>>;

    /**
     * Get the overworld items for the given byte position in the script
     * Does not consume either ptr.
     */
    getOverworldItems(
        runOutputPtr: TPtr,
        parseOutputPtr: TPtr,
        bytePos: number,
    ): Pwr<Result<InvView_Overworld, RuntimeViewError>>;

    /**
     * Get crash info for the given byte position in the script.
     * Does not consume either ptr. Returns empty string if no crash
     */
    getCrashInfo(
        runOutputPtr: TPtr,
        parseOutputPtr: TPtr,
        bytePos: number,
    ): Pwr<string>;

    /** Get list of saves. Does not consume either ptr. */
    getSaveNames(
        runOutputPtr: TPtr,
        parseOutputPtr: TPtr,
        bytePos: number,
    ): Pwr<string[]>;

    /** Get the save inventory. Does not consume either ptr. Use undefined for manual save */
    getSaveInventory(
        runOutputPtr: TPtr,
        parseOutputPtr: TPtr,
        bytePos: number,
        name: string | undefined,
    ): Pwr<Result<InvView_Gdt, RuntimeViewError>>;

    // === ref counting api ===

    freeNativeHandle(ptr: TPtr): Promise<void>;
    freeParseOutput(ptr: TPtr): Promise<void>;
    freeRunOutput(ptr: TPtr): Promise<void>;
}

const NativeHandle = Symbol("NativeHandle");
export type NativeHandle = typeof NativeHandle;
const RunOutput = Symbol("RunOutput");
export type RunOutput = typeof RunOutput;
const ParseOutput = Symbol("ParseOutput");
export type ParseOutput = typeof ParseOutput;

/** Factory type to create Erc (Externally-RefCounted) pointers */
export type NativeEmpFactory<TPtr> = {
    readonly nullptr: TPtr; // get the nullptr value to pass into native function as fallback when Erc has undefined value
    readonly makeNativeHandleEmp: ReturnType<
        typeof makeEmpType<NativeHandle, TPtr>
    >;
    readonly makeRunOutputEmp: ReturnType<typeof makeEmpType<RunOutput, TPtr>>;
    readonly makeParseOutputEmp: ReturnType<
        typeof makeEmpType<ParseOutput, TPtr>
    >;
};

/** Bind the ref counting API to a Emp factory */
export const createNativeEmpFactory = <TPtr>(
    nullptr: TPtr,
    napi: NativeApiFunctions<TPtr>,
): NativeEmpFactory<TPtr> => {
    return {
        nullptr,
        makeNativeHandleEmp: makeEmpType<NativeHandle, TPtr>({
            marker: NativeHandle,
            free: (ptr: TPtr) => void napi.freeNativeHandle(ptr),
        }),
        makeRunOutputEmp: makeEmpType<RunOutput, TPtr>({
            marker: RunOutput,
            free: (ptr: TPtr) => void napi.freeRunOutput(ptr),
        }),
        makeParseOutputEmp: makeEmpType<ParseOutput, TPtr>({
            marker: ParseOutput,
            free: (ptr: TPtr) => void napi.freeParseOutput(ptr),
        }),
    };
};
