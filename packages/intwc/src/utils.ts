import type { TextModel } from "./language/LanguageClient.ts";

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
