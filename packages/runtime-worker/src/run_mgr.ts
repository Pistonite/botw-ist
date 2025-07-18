import type { Emp } from "@pistonite/pure/memory";
import type { Err, Result } from "@pistonite/pure/result";
import { scopedCapture } from "@pistonite/pure/sync";

import type {
    ErrorReport,
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    RuntimeError,
    RuntimeViewError,
} from "@pistonite/skybook-api";

import {
    type NativeApi,
    type ParseOutput,
    type RunOutput,
} from "./native_api.ts";
import type { ParseMgr } from "./parse_mgr.ts";
import {
    type Pwr,
    type WorkerError,
    abortedError,
    nullptrError,
} from "./error.ts";
import type { TaskMgr } from "./task_mgr.ts";
import { log } from "./util.ts";
import { crashApplication } from "./app_call.ts";

type RunAwaiter<TPtr> = {
    /** Resolve function to take the output */
    resolve: (x: Result<Emp<RunOutput, TPtr>, WorkerError>) => void;
    /** Task handle id for this run task */
    taskId: string;
    /** Byte pos to execute until for this task */
    executeToBytePos: number;
};

class RunContext<TPtr> {
    awaiters: RunAwaiter<TPtr>[];
    nativeHandleId: number;
    lastNotifyBytePos: number; // -1 means not notified yet
    lastNotifyOutputEmp: Emp<RunOutput, TPtr> | undefined;

    constructor(nativeHandleId: number) {
        this.awaiters = [];
        this.nativeHandleId = nativeHandleId;
        this.lastNotifyBytePos = -1;
        this.lastNotifyOutputEmp = undefined;
    }

    /** Returns false if the underlying native task was already disposed */
    startAwaitingTask(
        taskMgr: TaskMgr<TPtr>,
        taskId: string,
        bytePos: number,
    ): Pwr<Emp<RunOutput, TPtr>> | undefined {
        taskMgr.registerTask(taskId);
        if (!taskMgr.addNativeHandleDependency(taskId, this.nativeHandleId)) {
            taskMgr.unregisterTask(taskId);
            return undefined;
        }
        const outputPromise = new Promise<
            Result<Emp<RunOutput, TPtr>, WorkerError>
        >((resolve) => {
            this.awaiters.push({
                resolve,
                taskId,
                executeToBytePos: bytePos,
            });
        });
        return outputPromise;
    }

    async areAllTasksAborted(taskMgr: TaskMgr<TPtr>): Promise<boolean> {
        if (this.awaiters.length === 0) {
            // awaiters should never be 0
            // since the triggering task is added as an awaiter
            // if this does happen however,
            // the run should be not abortable anyway
            return false;
        }

        for (const { taskId } of this.awaiters) {
            if (await taskMgr.isTaskActive(taskId)) {
                return false;
            }
        }

        return true;
    }
}

/** Manages caching and batching run (execute) calls */
export class RunMgr<TPtr> {
    private napi: NativeApi<TPtr>;
    private parseMgr: ParseMgr<TPtr>;
    private taskMgr: TaskMgr<TPtr>;

    /**
     * Context of the run that is currently running
     * (or undefined if no run is running)
     */
    private runContext: RunContext<TPtr> | undefined;

    private lastScript: string;
    private serial: number;
    private cachedEmp: Emp<RunOutput, TPtr> | undefined;

    constructor(
        napi: NativeApi<TPtr>,
        parseMgr: ParseMgr<TPtr>,
        taskMgr: TaskMgr<TPtr>,
    ) {
        this.napi = napi;
        this.taskMgr = taskMgr;
        this.parseMgr = parseMgr;
        this.runContext = undefined;
        this.lastScript = "";
        this.serial = 1;
        this.cachedEmp = undefined;
    }

    /**
     * Wrapper to parse and run the script. If successful, run the function
     * and ensure the references are alive
     */
    private async withParseAndRunOutput<T>(
        script: string,
        taskId: string,
        executeToBytePos: number,
        fn: (parseOutput: TPtr, runOutput: TPtr) => Pwr<T>,
    ): Pwr<T> {
        const parseOutput = await this.parseMgr.parseScript(script);
        if (parseOutput.err) {
            log.error(`${taskId}\nparse failed, not running`);
            return parseOutput;
        }
        const parseOutputEmp = parseOutput.val;
        if (!parseOutputEmp.value) {
            log.error(`${taskId}\nparseScript returned nullptr, not running`);
            return {
                err: nullptrError("parseScript (in run) returned nullptr"),
            };
        }
        const runOutput = await this.executeScript(
            script,
            taskId,
            executeToBytePos,
            parseOutputEmp,
        );
        if (runOutput.err) {
            return runOutput;
        }
        const runOutputEmp = runOutput.val;
        if (!runOutputEmp.value) {
            return { err: nullptrError("executeScript returned nullptr") };
        }
        return await scopedCapture(
            () => fn(parseOutputEmp.value, runOutputEmp.value),
            [parseOutputEmp, runOutputEmp],
        );
    }

    /**
     * Parse and execute the script through native API with caching and batching
     *
     * Takes ownership of the parse output
     */
    private async executeScript(
        script: string,
        taskId: string,
        executeToBytePos: number,
        parseOutputEmp: Emp<ParseOutput, TPtr>,
    ): Pwr<Emp<RunOutput, TPtr>> {
        const isScriptUpToDate = this.lastScript === script;
        if (
            this.cachedEmp !== undefined &&
            this.runContext === undefined &&
            isScriptUpToDate
        ) {
            log.debug(`${taskId}\nreturning cached run result`);
            return { val: this.cachedEmp };
        }

        const currContext = this.runContext;
        if (currContext && isScriptUpToDate) {
            // ^ a run is currently running for the same script

            // if the current run is already past the executeToBytePos
            // then we can just resolve with the latest result in the context
            if (
                currContext.lastNotifyBytePos >= 0 &&
                currContext.lastNotifyBytePos > executeToBytePos
            ) {
                const lastNotifyOutput = currContext.lastNotifyOutputEmp;
                if (lastNotifyOutput) {
                    log.debug(`${taskId}\nreturning cached partial run result`);
                    return { val: lastNotifyOutput };
                }
            }
            // otherwise, add this as an awaiter
            log.debug(`${taskId}\nawaiting on current run`);
            const outputPromise = await currContext.startAwaitingTask(
                this.taskMgr,
                taskId,
                executeToBytePos,
            );
            if (outputPromise) {
                return outputPromise;
            }
            // fall through to trigger a new run if the current run
            // is no longer available
        }

        // trigger a new run
        return await this.executeScriptTriggeredByTask(
            script,
            taskId,
            executeToBytePos,
            parseOutputEmp,
        );
    }
    /** Parse and execute the script through native API. Waits to acquire a native resource handle first. */
    private async executeScriptTriggeredByTask(
        script: string,
        taskId: string,
        executeToBytePos: number,
        parseOutputEmp: Emp<ParseOutput, TPtr>,
    ): Pwr<Emp<RunOutput, TPtr>> {
        log.debug(`${taskId}\ntriggering script execution`);
        this.lastScript = script;
        // make a new context for this run
        const contextResult = await this.makeNewRunContext(taskId);
        if (contextResult.err) {
            return contextResult;
        }
        const thisContext = contextResult.val;
        // add the task that triggered this run as an awaiter.
        // this is to ensure the task can finish early without waiting
        // for the whole run to finish
        const outputPromise = thisContext.startAwaitingTask(
            this.taskMgr,
            taskId,
            executeToBytePos,
        );
        if (!outputPromise) {
            log.error(
                `${taskId}\nfailed to schedule await - did native handle creation fail?`,
            );
            await crashApplication();
            return { err: { type: "UnexpectedThrow" } };
        }
        this.runContext = thisContext;

        try {
            void this.executeScriptInternal(
                taskId,
                parseOutputEmp,
                thisContext,
            );
            // wait for the task to finish
            return await outputPromise;
        } catch (e) {
            log.error(
                `${taskId}\nerror thrown from executeScriptTriggeredByTask, this should not happen. This catch exists as a fail-safe for memory cleanup.`,
            );
            log.error(e);
            return { err: { type: "UnexpectedThrow" } };
        }
    }

    /**
     * Parse and execute the script through native API. Waits to acquire a native resource handle first.
     *
     * Consumes the ParseOutput
     */
    private async executeScriptInternal(
        triggeringTaskId: string,
        parseOutputEmp: Emp<ParseOutput, TPtr>,
        thisContext: RunContext<TPtr>,
    ): Promise<void> {
        this.serial++;
        const thisSerial = this.serial;
        const PREFIX = `${triggeringTaskId} #${thisSerial} NA#${thisContext.nativeHandleId}`;
        log.info(`${PREFIX}\nstarting script execution run`);

        const resolveAwaiters = (x: Err<WorkerError>) => {
            for (const { resolve, taskId } of thisContext.awaiters) {
                this.taskMgr.unregisterTask(taskId);
                resolve(x);
            }
        };

        // Take it out of Erc, we will free it manually (by passing it into runParsed)
        let outputRaw: TPtr | undefined = undefined;
        // shouldn't be possible, but we will just return nullptr if parseoutput is null
        const stepCount = await this.napi.getStepCount(parseOutputEmp.value);
        if (stepCount.err) {
            log.error(`${PREFIX}\nrun failed`);
            log.error(stepCount.err);
            this.handleError(thisSerial);
            resolveAwaiters(stepCount);
            return;
        }

        // ready to execute the steps - first check it's not aborted yet
        if (await thisContext.areAllTasksAborted(this.taskMgr)) {
            log.debug(`${PREFIX}\nrun aborted`);
            this.handleError(thisSerial);
            resolveAwaiters(abortedError());
            return;
        }
        const start = performance.now();

        const nativeHandleEmp = this.taskMgr.getNativeHandle(
            thisContext.nativeHandleId,
        );
        if (!nativeHandleEmp) {
            log.warn(
                `${PREFIX}\nnative handle is null so the run will not be abortable`,
            );
        }

        const outputResult = await scopedCapture(() => {
            return this.napi.runParsed(
                parseOutputEmp.value,
                nativeHandleEmp?.value,
                async (upToBytePos, outputRaw) => {
                    const outputEmp = this.napi.makeRunOutputEmp(outputRaw);
                    const awaiters = thisContext.awaiters;
                    thisContext.awaiters = [];
                    for (const x of awaiters) {
                        const { resolve, taskId, executeToBytePos } = x;
                        if (
                            executeToBytePos < 0 ||
                            upToBytePos <= executeToBytePos
                        ) {
                            thisContext.awaiters.push(x);
                            continue;
                        }
                        this.taskMgr.unregisterTask(taskId);
                        resolve({ val: outputEmp });
                    }
                    thisContext.lastNotifyBytePos = upToBytePos;
                    thisContext.lastNotifyOutputEmp = outputEmp;
                },
            );
        }, [parseOutputEmp, nativeHandleEmp]);

        if (outputResult.err) {
            log.error(`${PREFIX}\nrun failed`);
            log.error(outputResult.err);
            this.handleError(thisSerial);
            resolveAwaiters(outputResult);
            return;
        }
        if (outputResult.val.type === "Aborted") {
            log.debug(`${PREFIX}\nrun aborted`);
            this.handleError(thisSerial);
            resolveAwaiters(abortedError());
            return;
        }

        const msElapsed = performance.now() - start;
        if (
            thisContext.awaiters.length &&
            (await thisContext.areAllTasksAborted(this.taskMgr))
        ) {
            // only warn if the run took very long
            const emit =
                msElapsed > 10000 ? log.warn.bind(log) : log.debug.bind(log);
            emit(
                `${PREFIX}\nall tasks are aborted, but the run didn't abort successfully!`,
            );
        }

        log.info(
            `${PREFIX}\nscript execution finished in ${Math.round(msElapsed)}ms`,
        );

        for (const { taskId } of thisContext.awaiters) {
            this.taskMgr.unregisterTask(taskId);
        }
        outputRaw = outputResult.val.value;

        const returnEmp: Emp<RunOutput, TPtr> =
            this.napi.makeRunOutputEmp(outputRaw);
        // update cached result if we are the latest run
        if (thisSerial === this.serial) {
            log.info(`${PREFIX}\nsaving execution result to cache`);
            this.runContext = undefined;
            this.cachedEmp = returnEmp;
        }

        // resolve remaining awaiters
        for (const { resolve } of thisContext.awaiters) {
            resolve({ val: returnEmp });
        }
    }

    private handleError(thisSerial: number) {
        if (thisSerial !== this.serial) {
            return;
        }
        this.runContext = undefined;
        this.cachedEmp = undefined;
    }

    private async makeNewRunContext(
        requestingTaskId: string,
    ): Pwr<RunContext<TPtr>> {
        const nativeHandleId =
            await this.taskMgr.registerNativeHandle(requestingTaskId);
        if (nativeHandleId.err) {
            return nativeHandleId;
        }
        return { val: new RunContext(nativeHandleId.val) };
    }

    /* === below are bindings for runtime API === */

    public triggerFullExecution(script: string, taskId: string): Pwr<unknown> {
        return this.withParseAndRunOutput(script, taskId, -1, async () => {
            return { val: {} };
        });
    }

    public getPouchList(
        script: string,
        taskId: string,
        bytePos: number,
    ): Pwr<Result<InvView_PouchList, RuntimeViewError>> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            bytePos,
            (parseOutputBorrowed, runOutputBorrowed) => {
                return this.napi.getPouchList(
                    runOutputBorrowed,
                    parseOutputBorrowed,
                    bytePos,
                );
            },
        );
    }

    public getGdtInventory(
        script: string,
        taskId: string,
        bytePos: number,
    ): Pwr<Result<InvView_Gdt, RuntimeViewError>> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            bytePos,
            (parseOutputBorrowed, runOutputBorrowed) => {
                return this.napi.getGdtInventory(
                    runOutputBorrowed,
                    parseOutputBorrowed,
                    bytePos,
                );
            },
        );
    }

    public getOverworldItems(
        script: string,
        taskId: string,
        bytePos: number,
    ): Pwr<Result<InvView_Overworld, RuntimeViewError>> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            bytePos,
            (parseOutputBorrowed, runOutputBorrowed) => {
                return this.napi.getOverworldItems(
                    runOutputBorrowed,
                    parseOutputBorrowed,
                    bytePos,
                );
            },
        );
    }

    public getCrashInfo(
        script: string,
        taskId: string,
        bytePos: number,
    ): Pwr<string> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            bytePos,
            (parseOutputBorrowed, runOutputBorrowed) => {
                return this.napi.getCrashInfo(
                    runOutputBorrowed,
                    parseOutputBorrowed,
                    bytePos,
                );
            },
        );
    }

    public getRuntimeDiagnostics(
        script: string,
        taskId: string,
        bytePos: number,
    ): Pwr<ErrorReport<RuntimeError>[]> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            bytePos,
            (_, runOutputBorrowed) => {
                return this.napi.getRunErrors(runOutputBorrowed);
            },
        );
    }

    public getSaveNames(
        script: string,
        taskId: string,
        bytePos: number,
    ): Pwr<string[]> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            bytePos,
            (parseOutputBorrowed, runOutputBorrowed) => {
                return this.napi.getSaveNames(
                    runOutputBorrowed,
                    parseOutputBorrowed,
                    bytePos,
                );
            },
        );
    }

    public getSaveInventory(
        script: string,
        taskId: string,
        bytePos: number,
        name: string | undefined,
    ): Pwr<Result<InvView_Gdt, RuntimeViewError>> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            bytePos,
            (parseOutputBorrowed, runOutputBorrowed) => {
                return this.napi.getSaveInventory(
                    runOutputBorrowed,
                    parseOutputBorrowed,
                    bytePos,
                    name,
                );
            },
        );
    }
}
