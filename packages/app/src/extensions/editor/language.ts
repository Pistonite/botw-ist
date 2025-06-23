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

        "hold",
        "unhold",
        "hold-smuggle",
        "hold-attach",
        "drop",
        "dnp",
        "pick-up",
        "cook",

        "eat",
        "sell",

        "equip",
        "unequip",
        "shoot",
        "use",

        "roast",
        "bake",
        "boil",
        "freeze",
        "destroy",

        "sort",
        "entangle",
        "sync",
        "break",
        "save",
        "save-as",
        "reload",
        "close-game",
        "new-game",

        "open-inventory",
        "open-inv",
        "pause",
        "close-inventory",
        "close-inv",
        "unpause",

        "talk-to",
        "untalk",
        "enter",
        "exit",
        "leave",
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

        "armor-head",
        "head-armor",
        "head-armors",
        "armor-body",
        "body-armor",
        "body-armors",
        "armor-chest",
        "chest-armor",
        "chest-armors",
        "armor-upper",
        "upper-armor",
        "upper-armors",
        "armor-leg",
        "armor-legs",
        "leg-armor",
        "leg-armors",
        "armor-lower",
        "lower-armor",
        "lower-armors",

        "material",
        "materials",
        "food",
        "foods",
        "key-item",
        "key-items",
    ],
    keywords: ["time", "times", "slot", "slots"],
    annotaions: [
        ":test", // TODO
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
