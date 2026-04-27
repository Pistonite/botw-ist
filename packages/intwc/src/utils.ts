import * as monaco from "monaco-editor";

import type { Uri, Range, TextModel } from "./monaco_types.ts";

export const getNormalizedPath = (filename: string): string => {
    return getFileUri(filename).path;
};

export const getFileUri = (filename: string): Uri => {
    return monaco.Uri.file(filename);
};

/** Convert text span (start, end) to line number and column range */
export const spanToRange = (model: TextModel, start: number, end: number) => {
    const { lineNumber: startLineNumber, column: startColumn } = model.getPositionAt(start);
    const { lineNumber: endLineNumber, column: endColumn } = model.getPositionAt(end);
    return { startLineNumber, startColumn, endLineNumber, endColumn };
};

/** Convert range to text span (start, end)*/
export const rangeToSpan = (model: TextModel, range: Range): [number, number] => {
    const start = model.getOffsetAt({
        lineNumber: range.startLineNumber,
        column: range.startColumn,
    });
    const end = model.getOffsetAt({
        lineNumber: range.endLineNumber,
        column: range.endColumn,
    });
    return [start, end];
};

/**
 * Create an array that maps byte position to character position
 *
 * i.e. bytePosToCharPos[bytePos] = charPos
 * If bytePos is not valid, then the output is UB
 */
export const createBytePosToCharPosArray = (script: string): Uint32Array => {
    const encoder = new TextEncoder(); // UTF-8 encoder
    const byteLength = encoder.encode(script).length;
    // + 1 for the ending byte pos
    const bytePosToCharPos = new Uint32Array(byteLength + 1);
    let bytePos = 0;
    for (let charPos = 0; charPos < script.length; charPos++) {
        bytePosToCharPos[bytePos] = charPos;
        bytePos += encoder.encode(script[charPos]).length;
    }
    bytePosToCharPos[bytePos] = script.length;
    return bytePosToCharPos;
};

/** Convert byte position to character position (createBytePosToCharPosArray is faster for large number of calls) */
export const bytePosToCharPos = (script: string, bytePos: number): number => {
    const encoder = new TextEncoder(); // UTF-8 encoder
    let b = 0;
    for (let charPos = 0; charPos < script.length; charPos++) {
        if (b >= bytePos) {
            return charPos;
        }
        b += encoder.encode(script[charPos]).length;
    }
    return script.length;
};

/** Convert character position to byte position */
export const charPosToBytePos = (script: string, charPos: number): number => {
    return new TextEncoder().encode(script.slice(0, charPos)).length;
};
