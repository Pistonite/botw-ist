import type { Result } from "@pistonite/pure/result";
import type { AsyncErc } from "@pistonite/pure/memory";

import type {
    InvView_Gdt,
    InvView_Overworld,
    InvView_PouchList,
    RuntimeViewError,
} from "@pistonite/skybook-api";

import {
    type NativeApi,
    type RunOutput,
    makeRunOutputErc,
} from "./NativeApi.ts";
import type { ParseMgr } from "./ParseMgr.ts";
import { type Pwr, type WorkerError, nullptrError } from "./Error.ts";
import type { TaskMgr } from "./TaskMgr.ts";
import { sendPerfData } from "./AppCall.ts";

/** Manages caching and batching run (execute) calls */
export class RunMgr {
    private napi: NativeApi;
    private parseMgr: ParseMgr;
    private taskMgr: TaskMgr;

    private isRunning: boolean;
    private runAwaiters: ((
        x: Result<AsyncErc<RunOutput>, WorkerError>,
    ) => void)[] = [];
    private runAwaiterTaskIds: string[] = [];

    private lastScript: string;
    private serial: number;
    private cachedErc: AsyncErc<RunOutput>;

    constructor(napi: NativeApi, parseMgr: ParseMgr, taskMgr: TaskMgr) {
        this.napi = napi;
        this.taskMgr = taskMgr;
        this.parseMgr = parseMgr;
        this.isRunning = false;
        this.runAwaiters = [];
        this.runAwaiterTaskIds = [];
        this.lastScript = "";
        this.serial = 0;
        this.cachedErc = makeRunOutputErc(undefined);
    }

    public getPouchList(
        script: string,
        taskId: string,
        bytePos: number,
    ): Pwr<Result<InvView_PouchList, RuntimeViewError>> {
        return this.withParseAndRunOutput(
            script,
            taskId,
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
        const runOutput = await this.executeScript(script, taskId);
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
            // put the resolve in the awaiters synchronously
            const outputPromise = new Promise<
                Result<AsyncErc<RunOutput>, WorkerError>
            >((resolve) => {
                this.runAwaiters.push(resolve);
                this.runAwaiterTaskIds.push(taskId);
            });
            // mark task as running, but not holding on to a resource
            this.taskMgr.run(taskId);
            const output = await outputPromise;
            return output;
        }

        return await this.executeScriptInternal(script, taskId);
    }

    /** Parse and execute the script through native API. Waits to acquire a native resource handle first. */
    private async executeScriptInternal(
        script: string,
        taskId: string,
    ): Pwr<AsyncErc<RunOutput>> {
        this.taskMgr.register(taskId);
        this.isRunning = true;
        // clear the awaiters. Previous runs still have their awaiters,
        // but newer awaiters will be added to this run
        this.runAwaiters = [];
        this.runAwaiterTaskIds = [];
        const awaitersForThisRun = this.runAwaiters;
        const awaiterTaskIdsForThisRun = this.runAwaiterTaskIds;
        const resolveAwaiters = (
            x: Result<AsyncErc<RunOutput>, WorkerError>,
        ) => {
            for (const taskId of awaiterTaskIdsForThisRun) {
                this.taskMgr.finish(taskId);
            }
            for (const resolve of awaitersForThisRun) {
                resolve(x);
            }
        };
        const checkAborted = () => {
            return (
                this.taskMgr.isAborted(taskId) &&
                this.taskMgr.areAllAborted(awaiterTaskIdsForThisRun)
            );
        };

        const start = performance.now();
        console.log(`[worker] task: ${taskId} start executing script`);

        this.serial++;
        const serialBefore = this.serial;
        this.lastScript = script;
        const parseOutputErc = await this.parseMgr.parseScript(script);
        if (parseOutputErc.err) {
            // failed to parse
            this.taskMgr.finish(taskId);
            this.handleError(serialBefore);
            resolveAwaiters(parseOutputErc);
            return parseOutputErc;
        }

        // manually manage the parse output pointer
        const parseOutputRaw = parseOutputErc.val.take();
        let outputRaw: number | undefined = undefined;
        // shouldn't be possible, but we will just return nullptr if parseoutput is null
        if (parseOutputRaw) {
            const stepCount = await this.napi.getStepCount(parseOutputRaw);
            if (stepCount.err) {
                this.taskMgr.finish(taskId);
                this.handleError(serialBefore);
                resolveAwaiters(stepCount);
                return stepCount;
            }

            // ready to execute the steps
            // first check it's not aborted yet
            if (checkAborted()) {
                console.warn("aborted???");
                this.taskMgr.finish(taskId);
                this.handleError(serialBefore);
                const result = {
                    err: {
                        type: "Aborted",
                    },
                } as const;
                resolveAwaiters(result);
                return result;
            }

            // request resource
            const nativeHandle =
                await this.taskMgr.acquireNativeResourceAndRun(taskId);
            if (nativeHandle.err) {
                this.taskMgr.finish(taskId);
                this.handleError(serialBefore);
                resolveAwaiters(nativeHandle);
                return nativeHandle;
            }

            // execute with the native resource handle

            // simulate some delay - until we have the real runtime
            for (let i = 0; i < 5; i++) {
                await new Promise((resolve) => {
                    setTimeout(resolve, 1000);
                });
                if (checkAborted()) {
                    this.taskMgr.finish(taskId);
                    this.handleError(serialBefore);
                    const result = {
                        err: {
                            type: "Aborted",
                        },
                    } as const;
                    resolveAwaiters(result);
                    return result;
                }
            }

            // passing in 0 if somehow the handle is null
            // should be fine since the native has redundant null checks
            const outputResult = await this.napi.runParsed(
                parseOutputRaw,
                nativeHandle.val.value || 0,
            );
            // if the run failed (or aborted), don't report performance data
            if (outputResult.err) {
                this.taskMgr.finish(taskId);
                this.handleError(serialBefore);
                resolveAwaiters(outputResult);
                return outputResult;
            }
            if (outputResult.val.type === "Aborted") {
                this.taskMgr.finish(taskId);
                this.handleError(serialBefore);
                const result = {
                    err: {
                        type: "Aborted",
                    },
                } as const;
                resolveAwaiters(result);
                return result;
            }

            // TODO: have runtime report the actual instructions count in output
            const instructionsCount = 100000;
            const msElapsed = performance.now() - start;
            const ips = (instructionsCount / msElapsed) * 1000;
            const sps = (stepCount.val / msElapsed) * 1000;
            void sendPerfData({ ips, sps });

            // run is done - not abortable now :)
            console.log(
                `[worker] finished tasks:\n  ${taskId}\n  ${awaiterTaskIdsForThisRun.join("\n  ")}`,
            );
            this.taskMgr.finish(taskId);
            for (const taskId of awaiterTaskIdsForThisRun) {
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
            this.runAwaiters = [];
            await this.cachedErc.assign(outputRaw);
            returnStrongErc = await this.cachedErc.getStrong();
        } else {
            returnStrongErc = makeRunOutputErc(outputRaw);
        }

        // resolve all awaiters - each must get its own strong pointer
        for (const resolve of awaitersForThisRun) {
            resolve({ val: await returnStrongErc.getStrong() });
        }

        return { val: returnStrongErc };
    }

    private handleError(serialBefore: number) {
        if (serialBefore !== this.serial) {
            return;
        }
        this.isRunning = false;
        this.runAwaiters = [];
        void this.cachedErc.free();
    }
}
