import { wxMakePromise } from "@pistonite/workex";
import type { Result } from "@pistonite/pure/result";

import type {
    ErrorReport,
    ParserError,
    InvView_PouchList,
    InvView_Gdt,
    InvView_Overworld,
    ItemSearchResult,
    MaybeAborted,
    RuntimeInitParams,
    RuntimeInitError,
    RuntimeViewError,
} from "@pistonite/skybook-api";
import {
    crashApplication,
    type Pwr,
    type NativeApi,
    type QuotedItemResolverFn,
} from "skybook-runtime-worker";

export class WasmApi implements NativeApi {
    private panicked: boolean;
    private runtimeInitPromise: Promise<undefined>;
    private resolveRuntimeInitPromise: (x: undefined) => void;
    constructor() {
        this.panicked = false;
        const { promise, resolve } = wxMakePromise<undefined>();
        this.runtimeInitPromise = promise;
        this.resolveRuntimeInitPromise = resolve;
        // This is a hack to make WASM able to invoke crash directly
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (globalThis as any)["__global_crash_handler"] = () => {
            console.error("Panic invoked from WASM. Recovery is NOT possible!");
            this.panicked = true;
            void crashApplication();
        };
    }

    /** Initialize the WASM module, this is needed to call any WASM function */
    public async initWasmModule() {
        // This is injected by the build process
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const wasmModuleBase = (self as any)["__skybook_path_base"] as string;
        const wasmModulePath = wasmModuleBase + ".wasm";
        const wasmBindgenJSPath = wasmModuleBase + ".js";
        await wasm_bindgen({ module_or_path: wasmModulePath });
        await this.exec(() => {
            return wasm_bindgen.module_init(wasmModulePath, wasmBindgenJSPath);
        }, true);
    }

    /** Initialize the runtime and unblock the other API calls */
    public async initRuntime(
        customImage: Uint8Array | undefined,
        params: RuntimeInitParams | undefined,
    ): Pwr<Result<wasm_bindgen.RuntimeInitOutput, RuntimeInitError>> {
        const result = await this.exec(() => {
            return wasm_bindgen.init_runtime(customImage, params);
        }, true);
        if (result.err) {
            return result;
        }
        this.resolveRuntimeInitPromise(undefined);
        return result;
    }

    // Note that due to type issues with wasm-bindgen
    // TypeScript may not report errors here! be careful!

    public resolveItemIdent(query: string): Pwr<ItemSearchResult[]> {
        return this.exec(() => {
            return wasm_bindgen.resolve_item_ident(query);
        });
    }
    public parseScript(
        script: string,
        resolveQuotedItem: QuotedItemResolverFn,
    ): Pwr<number> {
        return this.exec(() => {
            return wasm_bindgen.parse_script(script, resolveQuotedItem);
        });
    }
    public parseScriptSemantic(
        script: string,
        start: number,
        end: number,
    ): Pwr<Uint32Array> {
        return this.exec(() => {
            return wasm_bindgen.parse_script_semantic(script, start, end);
        });
    }
    public getParserErrors(ptr: number): Pwr<ErrorReport<ParserError>[]> {
        return this.exec(() => {
            return wasm_bindgen.get_parser_errors(ptr);
        });
    }
    public getStepCount(ptr: number): Pwr<number> {
        return this.exec(() => {
            return wasm_bindgen.get_step_count(ptr);
        });
    }
    public getStepFromPos(ptr: number, bytePos: number): Pwr<number> {
        return this.exec(() => {
            return wasm_bindgen.get_step_from_pos(ptr, bytePos);
        });
    }

    public makeTaskHandle(): Pwr<number> {
        return this.exec(() => {
            return wasm_bindgen.make_task_handle();
        });
    }

    public abortTask(ptr: number): void {
        void this.exec(() => {
            return wasm_bindgen.abort_task(ptr);
        });
    }

    public runParsed(
        parsedOutputPtr: number,
        taskHandlePtr: number,
    ): Pwr<MaybeAborted<number>> {
        return this.exec(() => {
            return wasm_bindgen.run_parsed(parsedOutputPtr, taskHandlePtr);
        });
    }

    public getPouchList(
        runOutputPtr: number,
        parseOutputPtr: number,
        bytePos: number,
    ): Pwr<Result<InvView_PouchList, RuntimeViewError>> {
        return this.exec(() => {
            return wasm_bindgen.get_pouch_list(
                runOutputPtr,
                parseOutputPtr,
                bytePos,
            );
        });
    }

    public getGdtInventory(
        runOutputPtr: number,
        parseOutputPtr: number,
        bytePos: number,
    ): Pwr<Result<InvView_Gdt, RuntimeViewError>> {
        return this.exec(() => {
            return wasm_bindgen.get_gdt_inventory(
                runOutputPtr,
                parseOutputPtr,
                bytePos,
            );
        });
    }

    public getOverworldItems(
        runOutputPtr: number,
        parseOutputPtr: number,
        bytePos: number,
    ): Pwr<Result<InvView_Overworld, RuntimeViewError>> {
        return this.exec(() => {
            return wasm_bindgen.get_overworld_items(
                runOutputPtr,
                parseOutputPtr,
                bytePos,
            );
        });
    }

    public addRefNativeHandle(ptr: number): Promise<number> {
        return this.execAddRef("addRefNativeHandle", () => {
            return wasm_bindgen.add_ref_task_handle(ptr);
        });
    }

    public async freeNativeHandle(ptr: number): Promise<void> {
        await this.exec(() => {
            return wasm_bindgen.free_task_handle(ptr);
        });
    }

    public addRefParseOutput(ptr: number): Promise<number> {
        return this.execAddRef("addRefParseOutput", () => {
            return wasm_bindgen.add_ref_parse_output(ptr);
        });
    }

    public async freeParseOutput(ptr: number): Promise<void> {
        await this.exec(() => {
            return wasm_bindgen.free_parse_output(ptr);
        });
    }

    public addRefRunOutput(ptr: number): Promise<number> {
        return this.execAddRef("addRefRunOutput", () => {
            return wasm_bindgen.add_ref_run_output(ptr);
        });
    }

    public async freeRunOutput(ptr: number): Promise<void> {
        await this.exec(() => {
            return wasm_bindgen.free_run_output(ptr);
        });
    }

    private nullptrCrash(message: string) {
        console.error(`nullptr crash: ${message}`);
        this.panicked = true;
        void crashApplication();
    }

    private async execAddRef(
        marker: string,
        fn: () => number,
    ): Promise<number> {
        const result = await this.exec(fn);
        if (result.err) {
            console.error(result.err);
            this.nullptrCrash(`${marker} failed`);
            return 0;
        }
        return result.val;
    }

    /** Execute the closure if WASM did not previously panic */
    private async exec<T>(
        fn: () => T | Promise<T>,
        noWaitForRuntimeInit?: boolean,
    ): Pwr<Awaited<T>> {
        if (!noWaitForRuntimeInit) {
            await this.runtimeInitPromise;
        }
        if (this.panicked) {
            return { err: { type: "NativePanic" } };
        }
        try {
            const result = await fn();
            if (!this.panicked) {
                return { val: result };
            }
        } catch (e) {
            // with the console panic hook, the error won't be caught,
            // which is why we also have the global crash handler
            console.error(e);
            console.error("Panic detected in WASM. Recovery is NOT possible!");
            this.panicked = true;
            void crashApplication();
        }
        return { err: { type: "NativePanic" } };
    }
}
