import type { Result } from "@pistonite/pure/result";
import type { AsyncErc } from "@pistonite/pure/memory";

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
    makeRunOutputErc,
} from "./NativeApi.ts";
import type { ParseMgr } from "./ParseMgr.ts";
import { type Pwr, type WorkerError, nullptrError } from "./Error.ts";
import type { TaskMgr } from "./TaskMgr.ts";
import { sendPerfData } from "./AppCall.ts";

type RunAwaiter = {
    /** Resolve function to take the output */
    resolve: (x: Result<AsyncErc<RunOutput>, WorkerError>) => void;
    /** Task handle id for this run task */
    taskId: string;
    /** Byte pos to execute until for this task */
    executeToBytePos: number;
};

type RunContext = {
    awaiters: RunAwaiter[];
    lastNotifyBytePos: number; // -1 means not notified yet
    lastNotifyOutputErc: AsyncErc<RunOutput>;
};
const makeRunContext = (): RunContext => {
    return {
        awaiters: [],
        lastNotifyBytePos: -1,
        lastNotifyOutputErc: makeRunOutputErc(undefined),
    };
};

/** Manages caching and batching run (execute) calls */
export class RunMgr {
    private napi: NativeApi;
    private parseMgr: ParseMgr;
    private taskMgr: TaskMgr;

    private isRunning: boolean;
    /** Context of the run that is currently running (or a blank context if no run is running) */
    private runContext: RunContext;

    private lastScript: string;
    private serial: number;
    private cachedErc: AsyncErc<RunOutput>;

    constructor(napi: NativeApi, parseMgr: ParseMgr, taskMgr: TaskMgr) {
        this.napi = napi;
        this.taskMgr = taskMgr;
        this.parseMgr = parseMgr;
        this.isRunning = false;
        this.runContext = makeRunContext();
        this.lastScript = "";
        this.serial = 0;
        this.cachedErc = makeRunOutputErc(undefined);
    }

    public getRuntimeDiagnostics(
        script: string,
        taskId: string,
    ): Pwr<ErrorReport<RuntimeError>[]> {
        return this.withParseAndRunOutput(
            script,
            taskId,
            -1,
            (_, runOutputBorrowed) => {
                return this.napi.getRunErrors(runOutputBorrowed);
            },
        );
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

    /**
     * Wrapper to parse and run the script. If successful, run the function with the borrowed (strong) pointers,
     * and free them afterwards
     */
    private async withParseAndRunOutput<T>(
        script: string,
        taskId: string,
        executeToBytePos: number,
        fn: (parseOutputBorrowed: number, runOutputBorrowed: number) => Pwr<T>,
    ): Pwr<T> {
        const parseOutput = await this.parseMgr.parseScript(script);
        if (parseOutput.err) {
            this.taskMgr.finish(taskId);
            return parseOutput;
        }
        const parseOutputErc = parseOutput.val;
        if (parseOutputErc.value === undefined) {
            this.taskMgr.finish(taskId);
            return {
                err: nullptrError("parseScript (in run) returned nullptr"),
            };
        }
        const runOutput = await this.executeScript(
            script,
            taskId,
            executeToBytePos,
            await parseOutputErc.getStrong(),
        );
        if (runOutput.err) {
            await parseOutputErc.free();
            return runOutput;
        }
        const runOutputErc = runOutput.val;
        if (runOutputErc.value === undefined) {
            await parseOutputErc.free();
            return { err: nullptrError("executeScript returned nullptr") };
        }
        const out = await fn(parseOutputErc.value, runOutputErc.value);
        await parseOutputErc.free();
        await runOutputErc.free();
        return out;
    }

    /** Parse and execute the script through native API with caching and batching */
    private async executeScript(
        script: string,
        taskId: string,
        executeToBytePos: number,
        parseOutputErc: AsyncErc<ParseOutput>,
    ): Pwr<AsyncErc<RunOutput>> {
        const isScriptUpToDate = this.lastScript === script;
        if (
            this.cachedErc.value !== undefined &&
            !this.isRunning &&
            isScriptUpToDate
        ) {
            return { val: await this.cachedErc.getStrong() };
        }

        if (this.isRunning && isScriptUpToDate) {
            // if the current run is already past the executeToBytePos
            // then we can just resolve with the latest result in the context
            if (
                this.runContext.lastNotifyBytePos >= 0 &&
                this.runContext.lastNotifyBytePos > executeToBytePos
            ) {
                if (this.runContext.lastNotifyOutputErc.value) {
                    return {
                        val: await this.runContext.lastNotifyOutputErc.getStrong(),
                    };
                }
            }
            // otherwise, add this as an awaiter
            const outputPromise = new Promise<
                Result<AsyncErc<RunOutput>, WorkerError>
            >((resolve) => {
                this.runContext.awaiters.push({
                    resolve,
                    taskId,
                    executeToBytePos,
                });
            });
            // abort previous task with the same ID (to allow reuse of the
            // same task ID to automatically cancel previous)
            this.taskMgr.abort(taskId);
            this.taskMgr.deleteHandle(taskId);
            // mark task as running, but not holding on to a resource
            this.taskMgr.run(taskId);

            const output = await outputPromise;
            return output;
        }

        // trigger a new run
        return await this.executeScriptTriggeredByTask(
            script,
            taskId,
            executeToBytePos,
            parseOutputErc,
        );
    }
    /** Parse and execute the script through native API. Waits to acquire a native resource handle first. */
    private async executeScriptTriggeredByTask(
        script: string,
        taskId: string,
        executeToBytePos: number,
        parseOutputErc: AsyncErc<ParseOutput>,
    ): Pwr<AsyncErc<RunOutput>> {
        // abort previous task with the same ID (to allow reuse of the
        // same task ID to automatically cancel previous)
        this.taskMgr.abort(taskId);
        this.taskMgr.deleteHandle(taskId);
        console.log(`[worker] execute script triggered by ${taskId}`);
        this.taskMgr.register(taskId);
        this.isRunning = true;
        this.lastScript = script;
        // make a new context for this run
        const thisContext = makeRunContext();

        // add the task that triggered this run as an awaiter.
        // this is to ensure the task can finish early without waiting
        // for the whole run to finish
        const outputPromise = new Promise<
            Result<AsyncErc<RunOutput>, WorkerError>
        >((resolve) => {
            thisContext.awaiters.push({
                resolve,
                taskId,
                executeToBytePos,
            });
        });
        this.runContext = thisContext;

        let output: Awaited<Pwr<AsyncErc<RunOutput>>> | undefined = undefined;
        let fullRunPromise: Promise<void> | undefined = undefined;
        try {
            fullRunPromise = this.executeScriptInternal(
                taskId,
                parseOutputErc,
                thisContext,
            );
            // wait for the task to finish
            output = await outputPromise;
        } catch (e) {
            console.error(
                "Error thrown from executeScriptTriggeredByTask, this should not happen. This catch exists as a fail-safe for memory cleanup.",
            );
            console.error(e);
        }

        if (fullRunPromise) {
            // schedule cleanup of context
            void fullRunPromise.finally(() => {
                void thisContext.lastNotifyOutputErc.free();
            });
        } else {
            // no run happened because of error, cleanup now
            void thisContext.lastNotifyOutputErc.free();
        }

        if (output) {
            return output;
        }

        // if output is not set, it must be because of the throw
        return { err: { type: "UnexpectedThrow" } };
    }

    /**
     * Parse and execute the script through native API. Waits to acquire a native resource handle first.
     *
     * Consumes the ParseOutput
     */
    private async executeScriptInternal(
        // script: string,
        triggeringTaskId: string,
        parseOutputErc: AsyncErc<ParseOutput>,
        thisContext: RunContext,
    ): Promise<void> {
        //Pwr<AsyncErc<RunOutput>> {
        const resolveAwaiters = (
            x: Result<AsyncErc<RunOutput>, WorkerError>,
        ) => {
            for (const { resolve, taskId } of thisContext.awaiters) {
                this.taskMgr.finish(taskId);
                resolve(x);
            }
        };
        // helper to check if all tasks are aborted
        const areAllTasksAborted = () => {
            if (thisContext.awaiters.length === 0) {
                return false;
            }
            for (const { taskId } of thisContext.awaiters) {
                if (!this.taskMgr.isAborted(taskId)) {
                    return false;
                }
            }
            return true;
        };

        const start = performance.now();

        this.serial++;
        const serialBefore = this.serial;

        // Take it out of Erc, we will free it manually (by passing it into runParsed)
        const parseOutputRaw = parseOutputErc.take();
        let outputRaw: number | undefined = undefined;
        // shouldn't be possible, but we will just return nullptr if parseoutput is null
        if (parseOutputRaw) {
            const stepCount = await this.napi.getStepCount(parseOutputRaw);
            if (stepCount.err) {
                this.handleError(serialBefore);
                resolveAwaiters(stepCount);
                return;
            }

            // ready to execute the steps - first check it's not aborted yet
            if (areAllTasksAborted()) {
                this.handleError(serialBefore);
                const result = {
                    err: {
                        type: "Aborted",
                    },
                } as const;
                resolveAwaiters(result);
                return;
            }

            // request resource
            const nativeHandle =
                await this.taskMgr.acquireNativeResourceAndRun(
                    triggeringTaskId,
                );
            if (nativeHandle.err) {
                this.handleError(serialBefore);
                resolveAwaiters(nativeHandle);
                return;
            }

            // execute with the native resource handle
            // passing in 0 if somehow the handle is null
            // should be fine since the native has redundant null checks
            const outputResult = await this.napi.runParsed(
                parseOutputRaw,
                nativeHandle.val.value || 0,
                async (upToBytePos, outputRaw) => {
                    const outputErc = makeRunOutputErc(outputRaw);
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
                        this.taskMgr.finish(taskId);
                        resolve({ val: await outputErc.getStrong() });
                    }
                    thisContext.lastNotifyBytePos = upToBytePos;
                    void thisContext.lastNotifyOutputErc.free();
                    thisContext.lastNotifyOutputErc = outputErc;
                },
            );

            // if the run failed (or aborted), don't report performance data
            if (outputResult.err) {
                this.handleError(serialBefore);
                resolveAwaiters(outputResult);
                return;
            }
            if (outputResult.val.type === "Aborted") {
                this.handleError(serialBefore);
                const result = {
                    err: {
                        type: "Aborted",
                    },
                } as const;
                resolveAwaiters(result);
                return;
            }

            // TODO: have runtime report the actual instructions count in output
            const instructionsCount = 100000;
            const msElapsed = performance.now() - start;
            const ips = (instructionsCount / msElapsed) * 1000;
            const sps = (stepCount.val / msElapsed) * 1000;
            void sendPerfData({ ips, sps });

            // run is done - not abortable now :)
            for (const { taskId } of thisContext.awaiters) {
                this.taskMgr.finish(taskId);
            }
            outputRaw = outputResult.val.value;
            console.log(
                `[worker] executing script finished in ${Math.round(msElapsed)}ms`,
            );
        } else {
            console.warn(`[worker] parse failed, not executing.`);
        }

        let returnStrongErc: AsyncErc<RunOutput>;
        // update cached result if we are the latest run
        if (serialBefore === this.serial) {
            this.isRunning = false;
            this.runContext = makeRunContext();
            await this.cachedErc.assign(outputRaw);
            returnStrongErc = await this.cachedErc.getStrong();
        } else {
            returnStrongErc = makeRunOutputErc(outputRaw);
        }

        // resolve remaining awaiters
        for (const { resolve } of thisContext.awaiters) {
            resolve({ val: await returnStrongErc.getStrong() });
        }
    }

    private handleError(serialBefore: number) {
        if (serialBefore !== this.serial) {
            return;
        }
        this.isRunning = false;
        this.runContext = makeRunContext();
        void this.cachedErc.free();
    }
}
