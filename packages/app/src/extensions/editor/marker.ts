/**
 * Diagnostic markers handler
 */

import type { MarkerData, TextModel, DiagnosticProvider, DiagnosticTask } from "@pistonite/intwc";
import { charPosToBytePos, MarkerSeverity, spanToRange } from "@pistonite/intwc";
import type { WxPromise } from "@pistonite/workex";

import type { Diagnostic, ExtensionApp } from "@pistonite/skybook-api";

const EDITOR_EXTENSION_UUID = "b1b45de4-1df7-4832-ae0b-99b516f81df6";

export const provideParserDiagnostics = async (app: ExtensionApp, model: TextModel, script: string): Promise<DiagnosticTask[]> => {
    // parser is fast so just use one task
    return [{
        toOrder: 1,
        data: (async () => {
            const diagnostics = await app.provideParserDiagnostics(script);
            if (diagnostics.err) {
                console.error("failed to get parser diagnostics", diagnostics.err);
                return undefined;
            }
            // convert to the editor marker format
            return diagnostics.val.map((diagnostic) => {
                return convertDiagnosticToMarker(model, diagnostic);
            });
        })()
    }];
}

export const provideRuntimeDiagnostics = async (app: ExtensionApp, model: TextModel, script: string, charPos: number): Promise<DiagnosticTask[]> => {
    const bytePositions = await app.getStepBytePositions(script);
    if (bytePositions.err) {
        console.error("failed to get byte positions for runtime diagnostics");
        return [];
    }
    // the steps should all be cached before the current position,
    // so we just need the position after it
    const currentBytePos = charPosToBytePos(script, charPos);
    const startIdx = binarySearchForPosition(bytePositions.val, currentBytePos);
    let positions: number[];
    let orderBase: number;
    if (startIdx === 0) {
        positions = [...bytePositions.val];
        orderBase = 0;
    } else {
        positions = [currentBytePos, ...bytePositions.val.subarray(startIdx)];
        orderBase = startIdx - 1;
    }
    const len = positions.length;

    console.log(`requesting runtime diagnostics for ${len} steps`);
    const taskIdResult = await app.requestNewTaskIds(EDITOR_EXTENSION_UUID, len);
    if (taskIdResult.err) {
        console.error("failed to get taskIds for runtime diagnostics");
        return [];
    }
    const [newIds, oldIds] = taskIdResult.val;
    const outTasks: DiagnosticTask[] = [];
    for (let i = 0; i<len;i++) {
        outTasks.push({
            // the step index
            toOrder: positions[i],
            data: (async () => {
                const result = await app.providePartialRuntimeDiagnostics(script, newIds[i], positions[i]);
                if (result.val?.type === "Aborted") {
                    // don't update the markers if the run was aborted,
                    // since the next run will update it
                    return undefined;
                }
                // cancel previous run when we know the new one is running
                if (i === 0) {
                    void app.cancelRuntimeTasks(oldIds);
                }
                if (result.err) {
                    console.error("failed to get runtime diagnostics", result.err);
                    return undefined;
                }
                const diagnostics = result.val.value;
                return diagnostics.map((diagnostic) => {
                    return convertDiagnosticToMarker(model, diagnostic);
                });
            })()
        })
    }
    console.log(outTasks);

    return outTasks;
}

const binarySearchForPosition = (bytePositions: Uint32Array, bytePos: number): number => {
  let low = 0;
  let high = bytePositions.length;
  while (low < high) {
    const mid = Math.floor((low + high) / 2);
    if (bytePositions[mid] <= bytePos) {
      low = mid + 1;
    } else {
      high = mid;
    }
  }
  return low;
}
//
// export const provideDiagnostics = async (
//     app: ExtensionApp,
//     model: TextModel,
//     owner: string,
// ): Promise<MarkerData[] | undefined> => {
//     const script = model.getValue();
//     let diagnostics: Awaited<WxPromise<Diagnostic[]>>;
//     if (owner === "runtime") {
//         const taskId = await app.requestNewTaskId(EDITOR_EXTENSION_UUID);
//         if (taskId.err) {
//             console.error("failed to get new task id", taskId.err);
//             return undefined;
//         }
//         const result = await app.provideRuntimeDiagnostics(script, taskId.val);
//         if (result.val?.type === "Aborted") {
//             // don't update the markers if the run was aborted,
//             // since the next run will update it
//             return undefined;
//         }
//         if (result.val) {
//             diagnostics = { val: result.val.value };
//         } else {
//             diagnostics = result;
//         }
//     } else {
//         diagnostics = await app.provideParserDiagnostics(script);
//     }
//     if (diagnostics.err) {
//         console.error("failed to get diagnostics", diagnostics.err);
//         return undefined;
//     }
//     // convert to the editor marker format
//     return diagnostics.val.map((diagnostic) => {
//         return convertDiagnosticToMarker(model, diagnostic);
//     });
// };

const convertDiagnosticToMarker = (
    model: TextModel,
    diagnostic: Diagnostic,
): MarkerData => {
    const { message, isWarning, start, end } = diagnostic;
    const range = spanToRange(model, start, end);
    return {
        message,
        severity: isWarning ? MarkerSeverity.Warning : MarkerSeverity.Error,
        ...range,
    };
};
