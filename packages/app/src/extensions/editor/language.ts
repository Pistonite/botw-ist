/**
 * Lanaguage definition for the Skybook script
 */
import type {
    LanguageConfiguration,
    LanguageTokenizer,
} from "@pistonite/intwc";

import { GenSyntax } from "./syntax.gen.ts";

export const configuration: LanguageConfiguration = {
    comments: {
        lineComment: "#",
    },

    brackets: [
        ["{", "}"],
        ["[", "]"],
        ["(", ")"],
        ["<", ">"],
    ],
    autoClosingPairs: [
        { open: "{", close: "}" },
        { open: "[", close: "]" },
        { open: "(", close: ")" },
        { open: "<", close: ">" },
        { open: '"', close: '"', notIn: ["string"] },
    ],
};

export const language: LanguageTokenizer = {
    defaultToken: "invalid",
    tokenPostfix: ".skyb",

    commands: GenSyntax.commands,
    types: GenSyntax.types,
    keywords: GenSyntax.keywords,
    // some keywords are also used as annotation keywords
    // so we just allow all of them to be highlighted as annotation
    annotaions: [
        ...GenSyntax.annotations,
        ...GenSyntax.keywords.map((x) => `:${x}`),
    ],

    word: /[_a-zA-Z][-0-9a-zA-Z_]*/,

    tokenizer: {
        root: [
            [/\s+/, "white"],
            [/\/\/.*$/, "comment"],
            [/#.*$/, "comment"],
            [/[{}()[\]]/, "@brackets"],
            // this is before delimiter so the ":" is matched
            [
                /(:)(@word)/,
                {
                    cases: {
                        "@annotaions": "keyword.annotation",
                        "@default": ["delimiter", "string.item"],
                    },
                },
            ],
            [/[=:,;]/, "delimiter"],
            [
                /(0x[\da-fA-F](_?[\da-fA-F])*)|(-?\d(_?\d)*(\.(\d(_?\d)*)?)?)/,
                "number",
            ],
            [/<@word>/, "string.item.literal"],
            [/"[^"]*"/, "string.item.quoted"],
            [/!@word/, "function.command.super"],
            [/(true|false)/, "constant.language.boolean"],
            [
                /@word/,
                {
                    cases: {
                        "@keywords": "keyword",
                        "@types": "type",
                        "@commands": "function.command",
                        "@default": "string.item",
                    },
                },
            ],
            // this has to be a separate state
            // because monarch doesn't support multiline tokens
            [/'''[-0-9a-zA-Z_]*/, "string.blockliteral", "@blockliteral"],
        ],
        blockliteral: [
            [/'''/, "string.blockliteral", "@pop"],
            [/./, "string.blockliteral"],
        ],
    },
};
