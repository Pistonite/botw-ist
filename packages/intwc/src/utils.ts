import type { Range, TextModel } from "./language/LanguageClient.ts";

/** 
 * Convert text span (start, end) to line number and column range
 */
export const spanToRange = (model: TextModel, start: number, end: number) => {
    const { lineNumber: startLineNumber, column: startColumn } = model.getPositionAt(start);
    const { lineNumber: endLineNumber, column: endColumn } = model.getPositionAt(end);
    return {
        startLineNumber,
        startColumn,
        endLineNumber,
        endColumn,
    };
};

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
}

/** 
 * Create an array that maps byte position to character position
 *
 * i.e. bytePosToCharPos[bytePos] = charPos
 * If bytePos is not valid, then the output is UB
 */
export const createBytePosToCharPosArray = (script: string): Uint32Array => {
    const encoder = new TextEncoder(); // UTF-8 encoder
    const byteLength = encoder.encode(script).length;
    const bytePosToCharPos = new Uint32Array(byteLength);
    let bytePos = 0;
    for (let charPos = 0; charPos < script.length; charPos++) {
        bytePosToCharPos[bytePos] = charPos;
        bytePos += encoder.encode(script[charPos]).length;
    }
    return bytePosToCharPos;
}

/** Convert character position to byte position */
export const charPosToBytePos = (script: string, charPos: number): number => {
    return new TextEncoder().encode(script.slice(0, charPos)).length;
}
