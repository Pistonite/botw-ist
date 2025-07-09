import type { AsyncErc } from "@pistonite/pure/memory";
import type { Result } from "@pistonite/pure/result";

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
    private runAwaiters: ((
        x: Result<AsyncErc<ParseOutput, TPtr>, WorkerError>,
    ) => void)[] = [];
    private lastScript: string;
    private serial: number;
    private cachedErc: AsyncErc<ParseOutput, TPtr>;

    constructor(napi: NativeApi<TPtr>) {
        this.napi = napi;
        this.isRunning = false;
        this.runAwaiters = [];
        this.lastScript = "";
        this.serial = 0;
        this.cachedErc = napi.makeParseOutputErc(undefined);
    }

    /** Parse the script and get diagnostics from the parser */
    public getParserDiagnostics(
        script: string,
    ): Pwr<ErrorReport<ParserError>[]> {
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

    /** Wrapper to call parseScript and use the result pointer, and free it afterwards */
    private async withParseOutput<T>(
        script: string,
        fn: (parseOutputBorrowed: TPtr) => Pwr<T>,
    ): Pwr<T> {
        const parseResult = await this.parseScript(script);
        if (parseResult.err) {
            return parseResult;
        }
        const ptr = parseResult.val.value;
        if (ptr === undefined) {
            log.error("parseScript returned nullptr, this is unexpected!!");
            return { err: nullptrError("parseScript returned nullptr") };
        }
        const out = await fn(ptr);
        await parseResult.val.free();
        return out;
    }

    /**
     * Parse the script and returns a strong pointer to the output (that must be freed)
     */
    public async parseScript(script: string): Pwr<AsyncErc<ParseOutput, TPtr>> {
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
    ): Pwr<AsyncErc<ParseOutput, TPtr>> {
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

        let returnStrongErc: AsyncErc<ParseOutput, TPtr>;
        // update cached result
        if (serialBefore === this.serial) {
            this.isRunning = false;
            this.runAwaiters = [];
            await this.cachedErc.assign(outputRaw.val);
            returnStrongErc = await this.cachedErc.getStrong();
        } else {
            returnStrongErc = this.napi.makeParseOutputErc(outputRaw.val);
        }

        // resolve all awaiters - each must get its own strong pointer
        for (const resolve of awaitersForThisRun) {
            resolve({ val: await returnStrongErc.getStrong() });
        }

        return { val: returnStrongErc };
    }
}
