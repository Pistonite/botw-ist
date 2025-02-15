import type { ParserErrorReport } from "skybook-parser";
import { makeExternalWeakRefType } from "@pistonite/pure/memory";

const makeParseOutputRef = makeExternalWeakRefType({
    marker: "ParseOutput" as const,
    free: (ptr: number) => wasm_bindgen.free_parse_output(ptr),
});
type WeakParseOutputRef = ReturnType<typeof makeParseOutputRef>;

// Runtime states

// === Parsing states ===
let isParsing = false;
let parseScript = "";
let parseSerial = 0;
let parseOutputRef = makeParseOutputRef(undefined); // Pointer into WASM memory

export const getParserDiagnostics = async (script: string, resolver: QuotedItemResolverFn): Promise<ParserErrorReport[]> => {
    const parseOutputPtr = await parseScriptInternal(script, resolver);
    if (parseOutputPtr.ref === undefined) {
        // shouldn't happen, just for safety
        return [];
    }
    return wasm_bindgen.get_parser_errors(parseOutputPtr.ref);
}

/**
 * Parse the script and return the pointer to the output
 *
 * If the current cached result is up-to-date, return it,
 * otherwise, the result will be freed and a new result
 * will be parsed and returned
 */
const parseScriptInternal = async (script: string, resolver: QuotedItemResolverFn): Promise<WeakParseOutputRef> => {
    // if the cache result is up-to-date, return it
    if (parseOutputRef.ref !== undefined && !isParsing && parseScript === script) {
        return parseOutputRef;
    }

    const serialBefore = parseSerial;
    parseSerial++;
    isParsing = true;
    parseScript = script;
    const output = await wasm_bindgen.parse_script(script, resolver);
    if (serialBefore === parseSerial) {
        isParsing = false;
        if (parseOutputRef.ref !== output) {
            parseOutputRef.free();
        }
        parseOutputRef = makeParseOutputRef(output);
    }

    return parseOutputRef;
}

export type QuotedItemResolverFn = (query: string) => Promise<{ actor: string; cookEffect: number } | undefined | null>;
