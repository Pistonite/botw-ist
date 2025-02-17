/**
 * Handles invoking the parser in WASM and caching the results, as well
 * as managing the memory in WASM
 */
import type { ParserErrorReport } from "@pistonite/skybook-api";
import { makeExternalWeakRefType } from "@pistonite/pure/memory";

const makeParseOutputRef = makeExternalWeakRefType({
    marker: "ParseOutput" as const,
    free: (ptr: number) => wasm_bindgen.free_parse_output(ptr),
});
type WeakParseOutputRef = ReturnType<typeof makeParseOutputRef>;

/** Promise of the on-going parse run */
let parsePromise: Promise<WeakParseOutputRef> | undefined = undefined;
/** Script of the on-going parse run or last finished run */
let lastScript = "";
let serial = 0;
let parseOutputRef = makeParseOutputRef(undefined); // Pointer into WASM memory

/** Parse the script and get diagnostics from the parser */
export const getParserDiagnostics = async (
    script: string,
    resolver: QuotedItemResolverFn,
): Promise<ParserErrorReport[]> => {
    const parseOutputPtr = await parseScript(script, resolver);
    if (parseOutputPtr.ref === undefined) {
        // shouldn't happen, just for safety
        return [];
    }
    const errors = wasm_bindgen.get_parser_errors(parseOutputPtr.ref);
    freeParseOutput(parseOutputPtr);
    return errors;
};

/**
 * Parse the script and return the pointer to the output
 *
 * This will return the cached result if possible. When the result is done
 * being used, it must be freed using `freeParseOutput`.
 */
const parseScript = (
    script: string,
    resolver: QuotedItemResolverFn,
): Promise<WeakParseOutputRef> => {
    const isScriptUpToDate = lastScript === script;
    // if the cache result is up-to-date, return it
    if (parseOutputRef.ref !== undefined && !parsePromise && isScriptUpToDate) {
        return Promise.resolve(parseOutputRef);
    }
    // if the result is not up-to-date, but the on-going run is the same script,
    // use the on-going run's result
    if (parsePromise && isScriptUpToDate) {
        return parsePromise;
    }

    parsePromise = parseScriptInternal(script, resolver);
    return parsePromise;
};

const parseScriptInternal = async (
    script: string,
    resolver: QuotedItemResolverFn,
): Promise<WeakParseOutputRef> => {
    const serialBefore = serial;
    serial++;
    lastScript = script;
    const output = await wasm_bindgen.parse_script(script, resolver);
    const newRef = makeParseOutputRef(output);
    // update cached result
    if (serialBefore === serial) {
        parsePromise = undefined;
        if (parseOutputRef.ref !== output) {
            parseOutputRef.free();
        }
        parseOutputRef = newRef;
    }

    return newRef;
};

/**
 * Free the parse output. If the output reference is the same as the cached result,
 * do nothing (since the cached result is managed by the cache itself). Otherwise,
 * the result is not used and will be freed
 */
const freeParseOutput = (parseOutput: WeakParseOutputRef) => {
    if (parseOutput.ref !== parseOutputRef.ref) {
        parseOutput.free();
    }
};

export type QuotedItemResolverFn = (
    query: string,
) => Promise<{ actor: string; cookEffect: number } | undefined | null>;
