import type { TokenStream } from "./tokenize";
// This must be kept in sync with grammar
// Terminal special characters: [ ] , = : " ! #
// \s matches whitespace and is used to separate other words
export const SpecialSymbols = /[\s[\],=:"!#]/;
// Function to try to get a T out of tokens
export type ParseFunction<T> = (tokens: TokenStream) => T | undefined;

export const ParseResultEpsilon = null;
export const ParseResultFail = undefined;
