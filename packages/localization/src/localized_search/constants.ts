import { logger } from "@pistonite/pure/log";

export const log = logger("item-search", "#3D0985").info();

// https://github.com/liyt96/is-japanese/blob/main/lib/is_japanese.js
// LICENSE: MIT
const RangeJa = [
    [0x3041, 0x3096], // Hiragana
    [0x30a0, 0x30ff], // Katakana
    [0xff00, 0xffef], // Full-width roman characters and half-width katakana
    [0x4e00, 0x9faf], // Common and uncommon kanji
    [0x3000, 0x303f], // Japanese Symbols and Punctuation
] as const;

// https://github.com/alsotang/is-chinese/blob/master/src/is_chinese.ts
// LICENSE: MIT
const RangeZh = [
    // sequence is determine by occurrence probability

    [0x4e00, 0x9fff], // CJK Unified Ideographs

    [0x3400, 0x4dbf], // CJK Unified Ideographs Extension A
    [0x20000, 0x2a6df], // CJK Unified Ideographs Extension B
    [0x2a700, 0x2b73f], // CJK Unified Ideographs Extension C
    [0x2b740, 0x2b81f], // CJK Unified Ideographs Extension D
    [0x2b820, 0x2ceaf], // CJK Unified Ideographs Extension E

    [0x3300, 0x33ff], // https://en.wikipedia.org/wiki/CJK_Compatibility
    [0xfe30, 0xfe4f], // https://en.wikipedia.org/wiki/CJK_Compatibility_Forms
    [0xf900, 0xfaff], // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs
    [0x2f800, 0x2fa1f], // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs_Supplement
] as const;

const convertCharRangeToRegExp = (range: readonly (readonly [number, number])[]) => {
    const reStr = range
        .map((range) => {
            if (range[0] === range[1]) {
                return `\\u{${range[0].toString(16)}}`;
            }
            return `[\\u{${range[0].toString(16)}}-\\u{${range[1].toString(16)}}]`;
        })
        .join("|");

    return new RegExp(reStr, "v");
};

const reJa = convertCharRangeToRegExp(RangeJa);
const reZh = convertCharRangeToRegExp(RangeZh);

export const detectLanguage = (text: string) => {
    // note that Japanese and Chinese characters
    // overlap, so we will just search both
    if (reJa.test(text) || reZh.test(text)) {
        return ["ja", "zh"] as const;
    }
    // otherwise it's too slow to check for all languages
    // maybe add korean?
    return [] as const;
};

export type SearchError = {
    type: "UnknownTag",
    tag: string
};
