
import { type Erc, makeErcType } from "@pistonite/pure/memory";

import { QuotedItemResolverFn, parseScript } from "./parser.ts";
import { InventoryListView } from "@pistonite/skybook-api";

const RunOutput = Symbol("RunOutput");
export type RunOutput = typeof RunOutput;

const makeRunOutputErc = makeErcType<RunOutput, number>({
    marker: RunOutput,
    free: (ptr: number) => wasm_bindgen.free_parse_output(ptr),
    addRef: (ptr: number) => wasm_bindgen.add_ref_parse_output(ptr),
});

let runPromise: Promise<Erc<RunOutput>> | undefined = undefined;
let lastScript = "";
let serial = 0;
const cachedRunOutputErc = makeRunOutputErc(undefined);

export const executeScript = (script: string, resolver: QuotedItemResolverFn)
    : Promise<Erc<RunOutput>> => {

    const isScriptUpToDate = lastScript === script;
    if (cachedRunOutputErc.value !== undefined && !runPromise && isScriptUpToDate) {
        return Promise.resolve(cachedRunOutputErc.getStrong());
    }
    if (runPromise && isScriptUpToDate) {
        return runPromise;
    }

    runPromise = executeScriptInternal(script, resolver);
    return runPromise;
}

const executeScriptInternal = async (
    script: string,
    resolver: QuotedItemResolverFn,
): Promise<Erc<RunOutput>> => {
    const start = performance.now();
    console.log("[worker] start executing script");
    const serialBefore = serial;
    serial++;
    lastScript = script;
    const parseOutputErc = await parseScript(script, resolver);
    const parseOutputRaw = parseOutputErc.take();
    let outputRaw = undefined;
    if (parseOutputRaw) { // shouldn't be possible to be null, but just checking
        outputRaw = await wasm_bindgen.run_parsed(parseOutputRaw);
    }
    console.log(`[worker] executing script finished in ${Math.round(performance.now() - start)}ms`);
    // update cached result
    if (serialBefore === serial) {
        runPromise = undefined;
        cachedRunOutputErc.assign(outputRaw);
        return cachedRunOutputErc.getStrong();
    }

    return makeRunOutputErc(outputRaw);
};

export const getInventoryListView = async (
script: string,
    resolver: QuotedItemResolverFn,
bytePos: number): Promise<InventoryListView> => {
    const parseOutputErc = await parseScript(script, resolver);
    const runOutputErc = await executeScript(script, resolver);
    const output = wasm_bindgen.get_inventory_list_view(runOutputErc.value || 0, 
        parseOutputErc.value || 0, bytePos);
    parseOutputErc.free();
    runOutputErc.free();
    return output;
}
