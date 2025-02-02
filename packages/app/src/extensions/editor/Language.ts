import type {
    LanguageConfiguration,
    LanguageTokenizer,
} from "@pistonite/intwc";
import { initCodeEditor } from "@pistonite/intwc";

let initialized = false;

export const initLanguage = () => {
    if (initialized) {
        return;
    }
    initialized = true;
    initCodeEditor({
        language: {
            custom: [
                {
                    getId: () => "skyb",
                    getExtensions: () => [".skyb"],
                    getTokenizer: () => language,
                    getConfiguration: () => configuration,
                },
            ],
        },
    });
};

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

    commands: ["init"],
    supercommands: [],
    annotations: [],
    keywords: [],

    word: /[_a-zA-Z][-0-9a-zA-Z_]*/,

    tokenizer: {
        root: [
            [/\s+/, "white"],
            [/\/\/.*$/, "comment"],
            [/#.*$/, "comment"],
            [/[{}()[\]]/, "@brackets"],
            [/[=:,;]/, "delimiter"],
            [/(\d(_?\d)*)|(0x[\da-fA-F](_?[\da-fA-F])*)/, "number"],
            [/<@word>/, "string.item.literal"],
            [/"@word"/, "string.item.quoted"],
            [/!@word/, "function.command.super"],
            [/:@word/, "keyword.annotation"],
            [/(true|false)/, "constant.language.boolean"],
            [
                /@word/,
                {
                    cases: {
                        "@keywords": "keyword",
                        "@commands": "function.command",
                        "@default": "string.item",
                    },
                },
            ],
        ],
    },
};
