/**
 * Handles invoking the parser in WASM and caching the results, as well
 * as managing the memory in WASM
 */
import type { ParserErrorReport } from "@pistonite/skybook-api";
import { type Erc, makeErcType } from "@pistonite/pure/memory";

import { resolveQuotedItem } from "./app.ts";

const ParseOutput = Symbol("ParseOutput");
export type ParseOutput = typeof ParseOutput;

const makeParseOutputErc = makeErcType<ParseOutput, number>({
    marker: ParseOutput,
    free: (ptr: number) => wasm_bindgen.free_parse_output(ptr),
    addRef: (ptr: number) => wasm_bindgen.add_ref_parse_output(ptr),
});

let isRunning = false;
// pointer for the awaiters for the current active run
let runAwaiters: ((x: Erc<ParseOutput>) => void)[] = [];
/** Script of the on-going parse run or last finished run */
let lastScript = "";
let serial = 0;
const cachedParseOutputErc = makeParseOutputErc(undefined);

/** Parse the script and get diagnostics from the parser */
export const getParserDiagnostics = async (
    script: string,
): Promise<ParserErrorReport[]> => {
    const parseOutputErc = await parseScript(script);
    if (parseOutputErc.value === undefined) {
        // shouldn't happen, just for safety
        return [];
    }
    const errors = wasm_bindgen.get_parser_errors(parseOutputErc.value);
    parseOutputErc.free();
    return errors;
};

export const getStepFromPos = async (
    script: string,
    bytePos: number,
): Promise<number> => {
    const parseOutputErc = await parseScript(script);
    if (parseOutputErc.value === undefined) {
        // shouldn't happen, just for safety
        return 0;
    }
    const step = wasm_bindgen.get_step_from_pos(parseOutputErc.value, bytePos);
    parseOutputErc.free();
    return step;
};

/**
 * Parse the script and returns a strong pointer to the output.
 * The pointer needs to be freed to avoid memory leak (i.e. Returns ownership)
 */
export const parseScript = (script: string): Promise<Erc<ParseOutput>> => {
    const isScriptUpToDate = lastScript === script;
    // if the cache result is up-to-date, return it
    if (
        cachedParseOutputErc.value !== undefined &&
        !isRunning &&
        isScriptUpToDate
    ) {
        return Promise.resolve(cachedParseOutputErc.getStrong());
    }
    // if the result is not up-to-date, but the on-going run is the same script,
    // use the on-going run's result
    if (isRunning && isScriptUpToDate) {
        return new Promise((resolve) => {
            runAwaiters.push(resolve);
        });
    }

    return parseScriptInternal(script);
};

const parseScriptInternal = async (
    script: string,
): Promise<Erc<ParseOutput>> => {
    isRunning = true;
    runAwaiters = [];
    const awaitersForThisRun = runAwaiters;

    serial++;
    const serialBefore = serial;
    lastScript = script;
    const outputRaw = await wasm_bindgen.parse_script(
        script,
        resolveQuotedItem,
    );

    let returnStrongErc: Erc<ParseOutput>;
    // update cached result
    if (serialBefore === serial) {
        isRunning = false;
        runAwaiters = [];
        cachedParseOutputErc.assign(outputRaw);
        returnStrongErc = cachedParseOutputErc.getStrong();
    } else {
        returnStrongErc = makeParseOutputErc(outputRaw);
    }

    // resolve all awaiters - each must get its own strong pointer
    for (const resolve of awaitersForThisRun) {
        resolve(returnStrongErc.getStrong());
    }

    return returnStrongErc;
};
