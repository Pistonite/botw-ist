import type { WxPromise } from "@pistonite/workex";
import type { Result } from "@pistonite/pure/result";

import type {
    ErrorReport,
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    MaybeAborted,
    ParserError,
    RuntimeError,
    RuntimeViewError,
} from "./native";
import type {
    ItemSearchResult,
    RuntimeWorkerInitArgs,
    RuntimeWorkerInitOutput,
    RuntimeWorkerInitError,
} from "./types.ts";

/**
 * API provided by the simulator runtime, called by the application.
 */
export interface Runtime {
    /**
     * Initialize the runtime with the given arguments.
     */
    initialize(
        args: RuntimeWorkerInitArgs,
    ): WxPromise<Result<RuntimeWorkerInitOutput, RuntimeWorkerInitError>>;

    /**
     * Resolve an item identifier search query to a list of items, ordered by score (best first).
     * Returns an empty list if no items are found.
     */
    resolveItemIdent(query: string): WxPromise<ItemSearchResult[]>;

    /**
     * Parse the script and get diagnostics from the parser.
     *
     * Note that the span in the errors are byte offsets, not character offsets.
     */
    getParserDiagnostics(script: string): WxPromise<ErrorReport<ParserError>[]>;

    /**
     * Parse the script and get semantic tokens in the range from the parser.
     *
     * The output is triples of [start, length, tokenType]
     *
     * The offsets in both inputs and outputs should be byte offsets, not character offsets.
     */
    getSemanticTokens(
        script: string,
        start: number,
        end: number,
    ): WxPromise<Uint32Array>;

    /** Get index of the step from byte position in the script */
    getStepFromPos(script: string, pos: number): WxPromise<number>;

    /** Abort a task by task id passed into one of the runtime functions that execute the script */
    abortTask(taskId: string): WxPromise<void>;

    /**
     * Run the script and get diagnostics from the runtime.
     *
     * Note that the span in the errors are byte offsets, not character offsets.
     *
     * The taskId should be a UUID, and can be passed into abortTask() to abort this run
     */
    getRuntimeDiagnostics(
        script: string,
        taskId: string,
    ): WxPromise<ErrorReport<RuntimeError>[]>;

    /**
     * Execute the script if not up-to-date, and return the pouch inventory list view
     * at the byte offset `pos` in the script.
     *
     * The taskId should be a UUID, and can be passed into abortTask() to abort this run
     */
    getPouchList(
        script: string,
        taskId: string,
        pos: number,
    ): WxPromise<MaybeAborted<Result<InvView_PouchList, RuntimeViewError>>>;

    /**
     * Execute the script if not up-to-date, and return the GDT inventory view
     * at the byte offset `pos` in the script.
     *
     * The taskId should be a UUID, and can be passed into abortTask() to abort this run
     */
    getGdtInventory(
        script: string,
        taskId: string,
        pos: number,
    ): WxPromise<MaybeAborted<Result<InvView_Gdt, RuntimeViewError>>>;

    /**
     * Execute the script if not up-to-date, and return the overworld item view
     * at the byte offset `pos` in the script.
     *
     * The taskId should be a UUID, and can be passed into abortTask() to abort this run
     */
    getOverworldItems(
        script: string,
        taskId: string,
        pos: number,
    ): WxPromise<MaybeAborted<Result<InvView_Overworld, RuntimeViewError>>>;
}
