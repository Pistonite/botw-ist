import type { Result } from "@pistonite/pure/result";
import type { Emp } from "@pistonite/pure/memory";
import { scopedCapture } from "@pistonite/pure/sync";

import type { ErrorReport, ParserError } from "@pistonite/skybook-api";

import { resolveQuotedItem } from "./app_call.ts";
import type { NativeApi, ParseOutput } from "./native_api.ts";
import { type Pwr, type WorkerError, nullptrError } from "./error.ts";
import { log } from "./util.ts";

/** Manages caching and batching parse calls */
export class ParseMgr<TPtr> {
    private napi: NativeApi<TPtr>;
    private isRunning: boolean;
    /** Awaiters for the current run */
    private runAwaiters: ((x: Result<Emp<ParseOutput, TPtr>, WorkerError>) => void)[] = [];
    private lastScript: string;
    private serial: number;
    private cachedOutput: Emp<ParseOutput, TPtr> | undefined;

    constructor(napi: NativeApi<TPtr>) {
        this.napi = napi;
        this.isRunning = false;
        this.runAwaiters = [];
        this.lastScript = "";
        this.serial = 0;
        this.cachedOutput = undefined;
    }

    /** Wrapper to call parseScript, null check and use the result pointer */
    private async withParseOutput<T>(
        script: string,
        fn: (parseOutputBorrowed: TPtr) => Pwr<T>,
    ): Pwr<T> {
        const parseResult = await this.parseScript(script);
        if (parseResult.err) {
            return parseResult;
        }
        const parseOutput = parseResult.val;
        if (!parseOutput.value) {
            log.error("parseScript returned nullptr, this is unexpected!!");
            return { err: nullptrError("parseScript returned nullptr") };
        }
        return await scopedCapture(() => fn(parseOutput.value), parseOutput);
    }

    /**
     * Parse the script and returns a strong pointer to the output (that must be freed)
     */
    public async parseScript(script: string): Pwr<Emp<ParseOutput, TPtr>> {
        const isScriptUpToDate = this.lastScript === script;
        // if the cache result is up-to-date, return it
        if (this.cachedOutput !== undefined && !this.isRunning && isScriptUpToDate) {
            return { val: this.cachedOutput };
        }

        // if the result is not up-to-date, but the on-going run is the same script,
        // use the on-going run's result
        if (this.isRunning && isScriptUpToDate) {
            return new Promise((resolve) => {
                this.runAwaiters.push(resolve);
            });
        }

        return await this.parseScriptInternal(script);
    }

    private async parseScriptInternal(script: string): Pwr<Emp<ParseOutput, TPtr>> {
        this.isRunning = true;
        // clear the awaiters. Previous runs still have their awaiters,
        // but newer awaiters will be added to this run
        this.runAwaiters = [];
        const awaitersForThisRun = this.runAwaiters;

        this.serial++;
        const serialBefore = this.serial;
        this.lastScript = script;
        const outputRaw = await this.napi.parseScript(script, resolveQuotedItem);

        if (outputRaw.err) {
            if (serialBefore === this.serial) {
                this.isRunning = false;
                this.runAwaiters = [];
                this.cachedOutput = undefined;
            }
            for (const resolve of awaitersForThisRun) {
                resolve(outputRaw);
            }
            return outputRaw;
        }

        const returnEmp: Emp<ParseOutput, TPtr> = this.napi.makeParseOutputEmp(outputRaw.val);
        // update cached result
        if (serialBefore === this.serial) {
            this.isRunning = false;
            this.runAwaiters = [];
            this.cachedOutput = returnEmp;
        }

        // resolve all awaiters - each must get its own strong pointer
        for (const resolve of awaitersForThisRun) {
            resolve({ val: returnEmp });
        }

        return { val: returnEmp };
    }

    /** Parse the script and get diagnostics from the parser */
    public getParserDiagnostics(script: string): Pwr<ErrorReport<ParserError>[]> {
        return this.withParseOutput(script, (ptr) => {
            return this.napi.getParserErrors(ptr);
        });
    }

    public getStepFromPos(script: string, bytePos: number): Pwr<number> {
        return this.withParseOutput(script, (ptr) => {
            return this.napi.getStepFromPos(ptr, bytePos);
        });
    }

    public getStepBytePositions(script: string): Pwr<Uint32Array> {
        return this.withParseOutput(script, (ptr) => {
            return this.napi.getStepBytePositions(ptr);
        });
    }
}
