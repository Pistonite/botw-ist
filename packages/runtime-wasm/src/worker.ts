import { type Delegate, hostFromDelegate } from "@pistonite/workex";

import { bindRuntimeApiHost } from "skybook-runtime-api/sides/runtime";
import type { RuntimeApi } from "skybook-runtime-api";
import { latest } from "@pistonite/pure/sync";


type ParseResult = {
    stepPositions: number[];
    errors: unknown[];
    semanticTokens: Uint32Array;
}

async function boot() {
    await wasm_bindgen({ module_or_path: "/runtime/skybook.wasm" });

    const doParseScript = latest({
        fn: async (script: string): Promise<ParseResult> => {
            return wasm_bindgen.parse_script(script);
        }
    });
    let script: string = "";
    let parseResult: Promise<ParseResult> = Promise.resolve({ stepPositions: [], errors: [] });
    const parseScript = (newScript: string): Promise<ParseResult> => {
        if (script === newScript) {
            return parseResult;
        }
        script = newScript;
        parseResult = doParseScript(newScript);
        return parseResult;
    }

    let runResult: Promise<void> = Promise.resolve();
    const doRunScript = latest({
        fn: async (script: string): Promise<void> => {
            return wasm_bindgen.run_script(script);
        }
    });

    // TODO: any init here

    const api = {
        // non-script-life-cycle methods
        resolveItemIdent: async (query) => {
            return wasm_bindgen.resolve_item_ident(query);
        },

        // script-life-cycle methods
        onScriptChange: async (newScript) => {
            await parseScript(newScript);
            if (newScript === script) {
                void doRunScript(newScript);
            }
        },

        getSemanticTokens: async (newScript, startPos, endPos) => {
            const result = await parseScript(newScript);
            // TODO: fix this
            return result.semanticTokens.slice(startPos, endPos);
        },
    } satisfies Delegate<RuntimeApi>;

    const handshake = bindRuntimeApiHost(hostFromDelegate(api), {
        worker: self,
    });
    await handshake.initiate();
}

void boot();
