import type { AsyncErc } from "@pistonite/pure/memory";
import type { Result } from "@pistonite/pure/result";

import type { ParserErrorReport } from "@pistonite/skybook-api";

import { resolveQuotedItem } from "./AppCall.ts";
import {
    type NativeApi,
    type ParseOutput,
    makeParseOutputErc,
} from "./NativeApi.ts";
import { type Pwr, type WorkerError, nullptrError } from "./Error.ts";

/** Manages caching and batching parse calls */
export class ParseMgr {
    private napi: NativeApi;
    private isRunning: boolean;
    /** Awaiters for the current run */
    private runAwaiters: ((
        x: Result<AsyncErc<ParseOutput>, WorkerError>,
    ) => void)[] = [];
    private lastScript: string;
    private serial: number;
    private cachedErc: AsyncErc<ParseOutput>;

    constructor(napi: NativeApi) {
        this.napi = napi;
        this.isRunning = false;
        this.runAwaiters = [];
        this.lastScript = "";
        this.serial = 0;
        this.cachedErc = makeParseOutputErc(undefined);
    }

    /** Parse the script and get diagnostics from the parser */
    public getParserDiagnostics(script: string): Pwr<ParserErrorReport[]> {
        return this.withParseOutput(script, (ptr) => {
            return this.napi.getParserErrors(ptr);
        });
    }

    public getStepFromPos(
        script: string,
        bytePos: number,
    ): Promise<Result<number, WorkerError>> {
        return this.withParseOutput(script, (ptr) => {
            return this.napi.getStepFromPos(ptr, bytePos);
        });
    }

    /** Wrapper to call parseScript and use the result pointer, and free it afterwards */
    private async withParseOutput<T>(
        script: string,
        fn: (parseOutputBorrowed: number) => Promise<Result<T, WorkerError>>,
    ): Promise<Result<T, WorkerError>> {
        const parseResult = await this.parseScript(script);
        if (parseResult.err) {
            return parseResult;
        }
        const ptr = parseResult.val.value;
        if (ptr === undefined) {
            return { err: nullptrError("parseScript returned nullptr") };
        }
        const out = await fn(ptr);
        await parseResult.val.free();
        return out;
    }

    /**
     * Parse the script and returns a strong pointer to the output (that must be freed)
     */
    public async parseScript(
        script: string,
    ): Promise<Result<AsyncErc<ParseOutput>, WorkerError>> {
        const isScriptUpToDate = this.lastScript === script;
        // if the cache result is up-to-date, return it
        if (
            this.cachedErc.value !== undefined &&
            !this.isRunning &&
            isScriptUpToDate
        ) {
            return { val: await this.cachedErc.getStrong() };
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

    private async parseScriptInternal(
        script: string,
    ): Promise<Result<AsyncErc<ParseOutput>, WorkerError>> {
        this.isRunning = true;
        // clear the awaiters. Previous runs still have their awaiters,
        // but newer awaiters will be added to this run
        this.runAwaiters = [];
        const awaitersForThisRun = this.runAwaiters;

        this.serial++;
        const serialBefore = this.serial;
        this.lastScript = script;
        const outputRaw = await this.napi.parseScript(
            script,
            resolveQuotedItem,
        );

        if (outputRaw.err) {
            if (serialBefore === this.serial) {
                this.isRunning = false;
                this.runAwaiters = [];
                await this.cachedErc.free();
            }
            for (const resolve of awaitersForThisRun) {
                resolve(outputRaw);
            }
            return outputRaw;
        }

        let returnStrongErc: AsyncErc<ParseOutput>;
        // update cached result
        if (serialBefore === this.serial) {
            this.isRunning = false;
            this.runAwaiters = [];
            await this.cachedErc.assign(outputRaw.val);
            returnStrongErc = await this.cachedErc.getStrong();
        } else {
            returnStrongErc = makeParseOutputErc(outputRaw.val);
        }

        // resolve all awaiters - each must get its own strong pointer
        for (const resolve of awaitersForThisRun) {
            resolve({ val: await returnStrongErc.getStrong() });
        }

        return { val: returnStrongErc };
    }
}
