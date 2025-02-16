
/**
 * Lanaguage definition for the Skybook script
 */
import type {
    LanguageConfiguration,
    LanguageTokenizer,
} from "@pistonite/intwc";

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

    commands: [
        "get",
        "buy",
        "pick-up",
        "hold",
        "hold-smuggle",
        "hold-attach",
        "drop",
        "dnp",
        "cook",
        "eat",
        "sell",
        "equip",
        "unequip",
        "use",
        "shoot",
        "roast",
        "bake",
        "boil",
        "freeze",
        "destroy",
        "sort",
        "entangle",
        "save",
        "save-as",
        "reload",
        "close-game",
        "new-game",
        "open-inventory",
        "close-inventory",
        "talk-to",
        "untalk",
        "enter",
        "exit",
        "leave"
    ],
    types: [
        "weapon",
        "weapons",
        "bow",
        "bows",
        "shield",
        "shields",
        "armor",
        "armors",
        "material",
        "materials",
        "food",
        "foods",
        "key-item",
        "key-items",
    ],
    keywords: [
        "time",
        "times",
    ],
    annotaions:  [
        ":test" // TODO
    ],

    word: /[_a-zA-Z][-0-9a-zA-Z_]*/,

    tokenizer: {
        root: [
            [/\s+/, "white"],
            [/\/\/.*$/, "comment"],
            [/#.*$/, "comment"],
            [/[{}()[\]]/, "@brackets"],
            // this is before delimiter so the ":" is matched
            [/(:)(@word)/, {
                cases: {
                    "@annotaions": "keyword.annotation",
                    "@default": ["delimiter", "string.item"],
                },
            }],
            [/[=:,;]/, "delimiter"],
            [/(\d(_?\d)*)|(0x[\da-fA-F](_?[\da-fA-F])*)/, "number"],
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
        ],
    },
};
