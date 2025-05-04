import { type Erc, makeErcType } from "@pistonite/pure/memory";

import type { InvView_Gdt, InvView_PouchList } from "@pistonite/skybook-api";

import { parseScript } from "./parser.ts";
import { sendPerfData } from "./app.ts";

const RunOutput = Symbol("RunOutput");
export type RunOutput = typeof RunOutput;

const makeRunOutputErc = makeErcType<RunOutput, number>({
    marker: RunOutput,
    free: (ptr: number) => wasm_bindgen.free_run_output(ptr),
    addRef: (ptr: number) => wasm_bindgen.add_ref_run_output(ptr),
});

let isRunning = false;
// pointer for the awaiters for the current active run
let runAwaiters: ((x: Erc<RunOutput>) => void)[] = [];
let lastScript = "";
let serial = 0;
const cachedRunOutputErc = makeRunOutputErc(undefined);

export const executeScript = (script: string): Promise<Erc<RunOutput>> => {
    const isScriptUpToDate = lastScript === script;
    if (
        cachedRunOutputErc.value !== undefined &&
        !isRunning &&
        isScriptUpToDate
    ) {
        return Promise.resolve(cachedRunOutputErc.getStrong());
    }
    if (isRunning && isScriptUpToDate) {
        return new Promise((resolve) => {
            runAwaiters.push(resolve);
        });
    }

    return executeScriptInternal(script);
};

const executeScriptInternal = async (
    script: string,
): Promise<Erc<RunOutput>> => {
    isRunning = true;
    runAwaiters = [];
    const awaitersForThisRun = runAwaiters;

    const start = performance.now();
    console.log("[worker] start executing script");

    serial++;
    const serialBefore = serial;
    lastScript = script;
    const parseOutputErc = await parseScript(script);
    const parseOutputRaw = parseOutputErc.take();
    let outputRaw = undefined;
    if (parseOutputRaw) {
        const stepCount = wasm_bindgen.get_step_count(parseOutputRaw);
        // simulate some delay - until we have the real runtime
        await new Promise((resolve) => {
            setTimeout(resolve, 5000);
        });
        // shouldn't be possible to be null, but just checking
        outputRaw = await wasm_bindgen.run_parsed(parseOutputRaw);

        // TODO: have runtime report the actual instructions count in output
        const instructionsCount = 100000;
        const msElapsed = performance.now() - start;
        const ips = (instructionsCount / msElapsed) * 1000;
        const sps = (stepCount / msElapsed) * 1000;
        void sendPerfData({ ips, sps });
    }
    console.log(
        `[worker] executing script finished in ${Math.round(performance.now() - start)}ms`,
    );

    let returnStrongErc: Erc<RunOutput>;
    // update cached result if we are the latest run
    if (serialBefore === serial) {
        isRunning = false;
        runAwaiters = [];
        cachedRunOutputErc.assign(outputRaw);
        returnStrongErc = cachedRunOutputErc.getStrong();
    } else {
        returnStrongErc = makeRunOutputErc(outputRaw);
    }

    // resolve all awaiters - each must get its own strong pointer
    for (const resolve of awaitersForThisRun) {
        resolve(returnStrongErc.getStrong());
    }

    return returnStrongErc;
};

export const getPouchList = async (
    script: string,
    bytePos: number,
): Promise<InvView_PouchList> => {
    const parseOutputErc = await parseScript(script);
    const runOutputErc = await executeScript(script);

    // TODO: report error through return error type
    if (
        parseOutputErc.value === undefined ||
        runOutputErc.value === undefined
    ) {
        parseOutputErc.free();
        runOutputErc.free();
        throw new Error(
            `parseOutputErc or runOutputErc is null: ${parseOutputErc.value}, ${runOutputErc.value}`,
        );
    }

    const output = wasm_bindgen.get_pouch_list(
        runOutputErc.value,
        parseOutputErc.value,
        bytePos,
    );
    parseOutputErc.free();
    runOutputErc.free();
    return output;
};

export const getGdtInventory = async (
    script: string,
    bytePos: number,
): Promise<InvView_Gdt> => {
    const parseOutputErc = await parseScript(script);
    const runOutputErc = await executeScript(script);

    // TODO: report error through return error type
    if (
        parseOutputErc.value === undefined ||
        runOutputErc.value === undefined
    ) {
        parseOutputErc.free();
        runOutputErc.free();
        throw new Error(
            `parseOutputErc or runOutputErc is null: ${parseOutputErc.value}, ${runOutputErc.value}`,
        );
    }

    const output = wasm_bindgen.get_gdt_inventory(
        runOutputErc.value,
        parseOutputErc.value,
        bytePos,
    );
    parseOutputErc.free();
    runOutputErc.free();
    return output;
};
