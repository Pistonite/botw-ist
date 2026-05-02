/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import type * as monaco from "monaco-editor/esm/vs/editor/editor.api.ts";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore no type
import { language as original } from "monaco-editor/esm/vs/basic-languages/typescript/typescript.js";

export const language = <monaco.languages.IMonarchLanguage>{
    ...original,
    tokenizer: {
        ...original.tokenizer,
        common: [
            // New rules
            [/(true|false)/, "constant.language.boolean"],
            [/null/, "constant.language.null"],
            [/undefined/, "constant.language.undefined"],
            [/(this|super|self)/, "variable.language"],
            // something that *could* be a function call/declaration
            [
                /#?[a-z_$][\w$]*(?=(\s*\(|\s*<.*>\s*\(|\s*`))/,
                { cases: { "@keywords": "keyword", "@default": "function" } },
            ],

            // patch old rule
            [/#?[a-z_$][\w$]*/, { cases: { "@keywords": "keyword", "@default": "variable" } }],
            ...original.tokenizer.common.slice(1),
        ],
    },
};
