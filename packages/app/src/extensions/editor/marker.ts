/**
 * Diagnostic markers handler
 */

import type { MarkerData, TextModel, DiagnosticProvider, DiagnosticTask , DiagnosticMergeResult} from "@pistonite/intwc";
import { charPosToBytePos, MarkerSeverity, spanToRange } from "@pistonite/intwc";
import { logger } from "@pistonite/pure/log";

import type { Diagnostic, ExtensionApp } from "@pistonite/skybook-api";

const EDITOR_EXTENSION_UUID = "b1b45de4-1df7-4832-ae0b-99b516f81df6";

const log = logger("diagnostics", "#02648B").default();

export type CustomMarkerData = MarkerData & {
    charPos: [number, number];
};
export type Provider = DiagnosticProvider<Diagnostic, CustomMarkerData>;
export type Task = DiagnosticTask<Diagnostic>;

/** Merge diagnostic tasks by replacing all markers with new run */
export const mergeDataByReplace = (
    model: TextModel,
    _currData: Diagnostic[], 
newBatch: Diagnostic[], 
): DiagnosticMergeResult<
Diagnostic, CustomMarkerData> => {
    const nextMarkers = newBatch.map((x) => convertDiagnosticToMarker(model, x));
    return { nextData: newBatch, nextMarkers };
}

export const mergeData = (
    model: TextModel,
    currData: Diagnostic[], 
newBatch: Diagnostic[], 
    prevBatch: Diagnostic[],
currMarkers: CustomMarkerData[]): DiagnosticMergeResult<
Diagnostic, CustomMarkerData> => {
    /*
     * Currently, this algorithm has some issues that cause
     * diagnostics to be duplicated, so it's not used.
     * the replace method is sufficient although the experience
     * is slightly worse
     */


    // The new and prev batches contain diagnostics
    // from the very beginning up to the current step
    // executed to
    
    const prevBatchLen = prevBatch.length;
    const newBatchLen = newBatch.length;
    const currDataLen = currData.length;
    const currMarkersLen = currMarkers.length;

    // find the highest end pos for any diagnostic
    // newer steps can add diagnostic to previous steps,
    // but not later steps, so we can just search the new Batch
    let highestEnd = 0;
    for (let i = 0;i<newBatchLen;i++) {
        highestEnd = Math.max(highestEnd, newBatch[i].end);
    }
    
    if (currData === newBatch) {
        // final update, delete old markers that have pos after end
        const newMarkers = currMarkers.filter((marker) => {
            return marker.charPos[1] <= highestEnd;
        });
        return { nextMarkers: newMarkers, nextData: currData };
    }
    // we can keep all data from current batch
    const newData: Diagnostic[] = [...newBatch];
    // and include from current data that don't overlap with good range
    for (let i = prevBatchLen; i < currDataLen; i++) {
        const data = currData[i];
        if (data.start < highestEnd) {
            continue;
        }
        newData.push(data);
    }

    // we can keep all markers from up to previous batch
    const newMarkers: CustomMarkerData[] = currMarkers.slice(0, prevBatchLen);
    // create new markers for new diagnostics in this batch
    for (let i = prevBatchLen; i < newBatchLen; i++) {
        const marker = convertDiagnosticToMarker(model, newBatch[i]);
        newMarkers.push(marker);
    }
    // include previous markers that don't overlap with good range
    for (let i = prevBatchLen; i < currMarkersLen; i++) {
        const marker = currMarkers[i];
        if (marker.charPos[0] < highestEnd) {
            continue;
        }
        newMarkers.push(currMarkers[i]);
    }

    return { nextData: newData, nextMarkers: newMarkers };
}

export const provideParserDiagnostics = async (app: ExtensionApp, script: string): Promise<Task[]> => {
    // parser is fast so just use one task
    return [{
        data: (async () => {
            const diagnostics = await app.provideParserDiagnostics(script);
            if (diagnostics.err) {
                log.error("failed to get parser diagnostics");
                log.error(diagnostics.err);
                return undefined;
            }
            return diagnostics.val;
        })()
    }];
}

export const provideRuntimeDiagnostics = async (app: ExtensionApp, script: string, charPos: number): Promise<Task[]> => {
    const bytePositions = await app.getStepBytePositions(script);
    if (bytePositions.err) {
        log.error("failed to get byte positions for runtime diagnostics");
        return [];
    }
    // the steps should all be cached before the current position,
    // so we just need the position after it
    const currentBytePos = charPosToBytePos(script, charPos);
    const startIdx = binarySearchForPosition(bytePositions.val, currentBytePos);
    let positions: number[];
    if (startIdx === 0) {
        positions = [...bytePositions.val];
    } else {
        positions = [currentBytePos, ...bytePositions.val.subarray(startIdx)];
    }
    const len = positions.length;

    log.info(`requesting runtime diagnostics for ${len} steps`);
    const taskIdResult = await app.requestNewTaskIds(EDITOR_EXTENSION_UUID, len);
    if (taskIdResult.err) {
        log.error("failed to get taskIds for runtime diagnostics");
        return [];
    }
    const [newIds, oldIds] = taskIdResult.val;
    if (oldIds.length) {
        void app.cancelRuntimeTasks(oldIds);
    }
    const outTasks: Task[] = [];
    for (let i = 0; i<len;i++) {
        const i2 = i;
        outTasks.push({
            // the step index
            data: (async () => {
                const taskId = newIds[i2];
                log.debug(`${taskId}\ndiagnostic step ${i2} starting`);
                const result = await app.providePartialRuntimeDiagnostics(script, newIds[i2], positions[i2]);
                log.debug(`${taskId}\ndiagnostic step ${i2} finished`);
                if (result.val?.type === "Aborted") {
                    // don't update the markers if the run was aborted,
                    // since the next run will update it
                    return undefined;
                }
                if (result.err) {
                    log.error("failed to get runtime diagnostics");
                    log.error(result.err);
                    return undefined;
                }
                return result.val.value;
            })()
        })
    }

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
): CustomMarkerData => {
    const { message, isWarning, start, end } = diagnostic;
    const range = spanToRange(model, start, end);
    return {
        charPos: [start, end],
        message,
        severity: isWarning ? MarkerSeverity.Warning : MarkerSeverity.Error,
        ...range,
    };
};
