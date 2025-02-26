import type { Token } from "./tokenize";
import { type ParseFunction, ParseResultFail, SpecialSymbols } from "./types";

export type ASTInteger = {
    type: "ASTInteger";
    value: number;
    range: [number, number];
};
export const isInteger = <T extends { type: string }>(
    node: T | ASTInteger | null,
): node is ASTInteger => Boolean(node && node.type === "ASTInteger");
export const parseInteger: ParseFunction<ASTInteger> = (tokens) => {
    tokens.push();
    const rangeTokens: Token[] = [];
    const value = tokens.consume(rangeTokens);
    if (!value) {
        tokens.restore();
        tokens.pop();
        return ParseResultFail;
    }
    // is next symbol integer?
    const num = parseInt(value);
    if (!Number.isInteger(num)) {
        tokens.restore();
        tokens.pop();
        return ParseResultFail;
    }
    // yes - consume it
    tokens.pop();
    return {
        type: "ASTInteger",
        value: num,
        range: [rangeTokens[0].start, rangeTokens[0].end],
    };
};
export type ASTIdentifier = {
    type: "ASTIdentifier";
    value: string;
    range: [number, number];
};
export const isIdentifier = <T extends { type: string }>(
    node: T | ASTIdentifier | null,
): node is ASTIdentifier => Boolean(node && node.type === "ASTIdentifier");
export const parseIdentifier: ParseFunction<ASTIdentifier> = (tokens) => {
    tokens.push();
    const rangeTokens: Token[] = [];
    const value = tokens.consume(rangeTokens);
    if (!value) {
        tokens.restore();
        tokens.pop();
        return ParseResultFail;
    }
    // first character must be alphabetical
    if (!value[0].match(/[a-zA-Z]/)) {
        tokens.restore();
        tokens.pop();
        return ParseResultFail;
    }
    if (value.match(SpecialSymbols)) {
        tokens.restore();
        tokens.pop();
        return ParseResultFail;
    }
    // yes - consume it
    tokens.pop();
    return {
        type: "ASTIdentifier",
        value: value,
        range: [rangeTokens[0].start, rangeTokens[0].end],
    };
};

export const createIdentifier = (
    fullText: string,
    start: number,
    end: number,
): ASTIdentifier => {
    return {
        type: "ASTIdentifier",
        value: fullText.substring(start, end),
        range: [start, end],
    };
};
