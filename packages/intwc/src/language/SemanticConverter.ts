import type { TextModel } from "./LanguageClient.ts";

import { spanToRange } from "../utils.ts";

/**
 * Convert semantic tokens from triples [start, length, tokenType] to 
 * LSP format [deltaLine, deltaStart, length, tokenType, tokenModifiers]
 */
export const convertSemanticTokens = (
    inputs: Uint32Array | number[], model: TextModel, options: SemanticConverterOptions
): number[] => {
    const { convertType } = options;
    // inputs are triples: [start, length, type]
    // outputs are 5-tuples: [deltaLine, deltaStart, length, tokenType, tokenModifiers]
    const outputs: number[] = [];
    let prevLine = 1;
    let prevStart = 1;

    // since we only run this for the latest range that's requested,
    // we should never have to worry about having too many tokens
    // returned from the worker
    for (let i = 0; i + 3 <= inputs.length; i += 3) {
        const start = inputs[i];
        const length = inputs[i + 1];
        const [type, modifier] = convertType(inputs[i + 2]);
        if (type === undefined) {
            // invalid input, ignore
            continue;
        }

        const { 
            startLineNumber, 
            startColumn, 
            endLineNumber, 
            endColumn } = spanToRange(
            model,
            start,
            start + length,
        );
        if (startLineNumber === endLineNumber) {
            const deltaLine = startLineNumber - prevLine;
            const deltaStart = deltaLine === 0 ? startColumn - prevStart : startColumn - 1;

            outputs.push(deltaLine, deltaStart, length, type, modifier);
            prevStart = startColumn;
        } else {
            // token spanning multiple lines, convert it to separate entries
            const firstStart = startColumn - 1;
            const firstLength = model.getLineLength(startLineNumber) - firstStart;
            const firstDeltaLine = startLineNumber - prevLine;
            const firstDeltaStart = firstDeltaLine === 0 ? firstStart - prevStart : firstStart - 1;
            outputs.push(firstDeltaLine, firstDeltaStart, firstLength, type, modifier);

            // middle full lines
            for (let i = startLineNumber + 1; i < endLineNumber; i++) {
                // delta line is always 1, start is always 0
                const length = model.getLineLength(i);
                outputs.push(1, 0, length, type, modifier);
            }

            // last line, if we are not ending at the start of the line
            if (endColumn !== 1) {
                const lastLength = endColumn - 1;
                outputs.push(1, 0, lastLength, type, modifier);
            }
            prevStart = 1;
        }
        prevLine = endLineNumber;
    }

    return outputs;
}

export type SemanticConverterOptions = {
    /** 
     * Convert raw token type to [tokenType, tokenModifiers]
     *
     * The token type should be 1-indexed in the legend, and the modifier
     * should be a bit set. Return undefined for tokenType for invalid input.
     */
    convertType: (tokenType: number) => [number | undefined, number];
};
